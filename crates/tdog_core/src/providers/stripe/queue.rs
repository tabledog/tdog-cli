use std::collections::BinaryHeap;
use std::pin::Pin;
use tokio::sync::{Mutex, oneshot};
use tokio::time::{delay_for, Duration};
use tokio::task::JoinHandle;
use futures::FutureExt;
use futures::future::{BoxFuture, LocalBoxFuture, AbortHandle, Abortable};
use futures::future::join_all;
use std::cmp::Ordering;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::oneshot::{Sender, Receiver};
use core::cmp;


// Queues items, starts them in priority order in a rate limited way.
// - Allows starting futures based on both ("position in the tree" AND "queue decides").
//      - Other queueing mechanisms require moving the futures to a central list, which removes the async/await benefits (function based composition *without* custom messaging nodes and types, or hand written state machines).
// - Partially solves the "back pressure" problem.
//      - Rate limits requests to a target node, pauses and queues those requests when target signals that it is overloaded.
//          - There are two queue states for an item:
//              - 1. Future that has not been `.await` (similar to a closure).
//              - 2. Future that has been `.await`, but is in the queue and has not yet reached the front to start the actual wrapped function.
//          - Instead of over loading the target, local RAM is used to queue and rate limit those requests.
//              - Futures in RAM are like a queue.
//              - May end up using/exceeding max RAM (queue is not infinite, there still needs to be consideration from the user of the API).
//                  - One fix is to move the queue to a disk based database.
//                      - Q: Can a future be serialized to disk and restored when it is at the front of the queue to allow infinite queues without having to write serialization functions, database schemas or deal with Redis-like servers?
//
// - One queue per "rate limited target resource" (E.g. you might have a queue for each target HTTP API with custom rate limiting for each queue).
// - Rate limit is defined by "maximum of X items **started** per second".
//      - Does not monitor how long they take (currently no limit on maximum currently running items).
//      - The rate limited target system should signal it is full/overloaded, and this queue can be `paused` to allow the currently running futures to complete.
// - Design aimed at futures, can be used for other items.
// - Avoids moving futures.
//      - Queued futures are .awaited in their natural AST/function position.
//          - This means the start order of futures in the tree is going to be a combination of ("position in the tree", "queue decides").
//              - The "queue decides" part comes into play when:
//                  - You have a `join_all([a, b]).await` - "competing sibling futures".
//                  - You have many async branches - "competing branch-descendant futures".
//                      - E.g. a join_all, where each of the futures each represent a tree of futures.
//              - The position in the tree is still useful as it is the natural way to compose the output of async functions with the parent function scope (instead of moving the future to a queue, executing it, and then trying to move the output value back to where it is needed. E.g. lots of messaging protocols and sender/receiver nodes that just replicate an async function call stack. The async call stack removes all of the messaging nodes and types and allows the programmer to just use functions as their native composition tool, rather than "functions + messaging" which adds indirection that the IDE and compiler does not understand).
//      - `tokio::spawn()` requires Send.
//      - `tokio::spawn_local()` requires 'static (no stack references).
//      - If the entire `.await` tree of futures is on the same thread, then normal Rust refs can be used (&ref or &mut ref).
//          - Avoids wrapping in lock types.
//          - Reduces complex type signatures (For futures/output, lifetimes).
//      - `join_all(vec![a.q_high(), b.q_low()])` seems like it is a "background future", but it is not.
//          - This is a valid tree composition function.
pub struct Queue {
    heap: BinaryHeap<QueueItem>,
    max_started_per_second: u32,
    second_counter: (Instant, u32),
    pub(crate) paused: bool,

    // max_active_per_second: u32,
    // active: u32,
}

impl Queue {
    pub fn new(max_started_per_second: u32) -> Queue {
        Queue {
            heap: BinaryHeap::new(),
            max_started_per_second,
            second_counter: (Instant::now(), 0),
            paused: false,
        }
    }

    pub fn len(&self) -> u64 {
        self.heap.len() as u64
    }

    pub fn add_high(&mut self) -> Receiver<()> {
        let (tx, rx) = oneshot::channel();

        self.heap.push(QueueItem {
            priority: Priority::First as isize,
            sender: tx,
        });

        rx
    }
    pub fn add_med(&mut self) -> Receiver<()> {
        let (tx, rx) = oneshot::channel();

        self.heap.push(QueueItem {
            priority: Priority::Second as isize,
            sender: tx,
        });

        rx
    }
    pub fn add_low(&mut self) -> Receiver<()> {
        let (tx, rx) = oneshot::channel();

        self.heap.push(QueueItem {
            priority: Priority::Third as isize,
            sender: tx,
        });

        rx
    }

    // Assumption: second is reset at top of loop before calling this.
    fn take_capacity_for_this_second(&mut self) {
        let mut q = self;
        let one_sec = std::time::Duration::from_millis(1050);
        let elapsed = q.second_counter.0.elapsed();

        // This prevents not starting all the items in this second (and then overflowing into the next second).
        assert!(elapsed < one_sec);

        q.second_counter.1 += 1;
    }


    fn get_ms_remaining(&self) -> Option<u64> {
        let sec = std::time::Duration::from_secs(1);
        let elapsed = self.second_counter.0.elapsed();

        if elapsed >= sec {
            // Instead of negative number.
            return None;
        }

        let ms = (sec - elapsed).as_millis();

        if ms < 1 {
            return None;
        }

        Some(ms as u64)
    }


    // Capacity = A max of `x` items can be **started** per second.
    fn remaining_second_capacity(&self) -> u32 {
        assert!(self.second_counter.1 <= self.max_started_per_second);
        self.max_started_per_second - self.second_counter.1
    }


    // If one second or more has elapsed, reset the counter to zero to start fresh.
    fn reset_second_counter_if_gt_1s_elapsed(&mut self) {
        let mut q = self;
        let one_sec = std::time::Duration::from_secs(1);
        let elapsed = q.second_counter.0.elapsed();

        if elapsed > one_sec {
            q.second_counter = (Instant::now(), 0);
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }
    pub fn resume(&mut self) {
        self.paused = false;
    }

    // @see https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=dba3959057d9f4beae3fc844d6ae9a99
    // - Tokio Mutex locks are held beyond await points.
    pub fn run_scheduler(q_mt_a: Arc<Mutex<Queue>>) -> AbortHandle {
        let (a_h, a_r) = AbortHandle::new_pair();

        tokio::task::spawn(Abortable::new(async move {
            loop {
                // Do not hold lock over long `.await` points (will prevent items being queued).
                let (heap_len, remaining_second_capacity, ms_remaining, paused) = {
                    let mut q = q_mt_a.lock().await;

                    q.reset_second_counter_if_gt_1s_elapsed();

                    (q.heap.len(), q.remaining_second_capacity(), q.get_ms_remaining(), q.paused)
                };

                // dbg!((heap_len, remaining_second_capacity, ms_remaining));

                if paused {
                    delay_for(Duration::from_millis(1000)).await;
                    continue;
                }


                // Queue empty - check again quickly to run added items ASAP.
                if heap_len == 0 {
                    delay_for(Duration::from_millis(20)).await;
                    continue;
                }

                // Wait for next second.
                if remaining_second_capacity == 0 {
                    delay_for(Duration::from_millis(ms_remaining.unwrap() + 5)).await;
                    continue;
                }


                {
                    let mut q = q_mt_a.lock().await;

                    for i in 0..q.remaining_second_capacity() {
                        if let Some(x) = q.heap.pop() {
                            q.take_capacity_for_this_second();
                            // Signal future to start running (it is currently waiting in-place in its owners parent fn scope async-tree).
                            // - It is important that the future is not moved as this avoids having to implement `Send` or `'static` for all future fn args.
                            //      - Any struct with references to other fn stack variables is not `'static'.
                            //      - This allows passing normal references like `&x` or `&mut x` to a future without lifetimes/locking - as long as its a single thread event loop and none of the descendant future tree needs to be moved into `tokio::spawn` or `tokio::spawn_local` task (first requires Send, second requires 'static ).
                            //      - "Not moved" includes `Box<Future>`.
                            // Alternatives:
                            // - https://docs.rs/futures/0.3.17/futures/future/trait.FutureExt.html#method.shared
                            //      - Clone a future for moving to new threads (original future will still need to be awaited in place).
                            // - https://docs.rs/futures-util/0.3.17/futures_util/future/trait.FutureExt.html#method.remote_handle


                            // @todo/next remove this, it was added to debug network issues with >100 concurrent requests.
                            // delay_for(Duration::from_millis(20)).await;


                            x.sender.send(()).unwrap();
                        } else {
                            break;
                        }
                    }
                }
            }

            ()
        }, a_r));

        a_h
    }
}


struct QueueItem {
    // Bigger number == higher priority == ordered first
    priority: isize,
    sender: Sender<()>,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
        // Is this trait used for identity, or just for ordering?
    }
}

impl Eq for QueueItem {}


// Tag `Priority` usage.
// - Allow "find places" IDE lookup.
// - Allow adding new enum levels between the existing ones.
//      - "IDE rename all" OR `10.pow(2) + 1`
// - Allow hard override by just hard coding a number.
enum Priority {
    First = 10_i32.pow(4) as isize,
    Second = 10_i32.pow(3) as isize,
    Third = 10_i32.pow(2) as isize,
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::oneshot;

    // Test traits implemented properly.
    #[test]
    fn test_queue() {
        let mut q = Queue::new(1);
        let (tx1, rx) = oneshot::channel::<()>();
        let (tx2, rx) = oneshot::channel::<()>();
        let (tx3, rx) = oneshot::channel::<()>();

        q.heap.push(QueueItem {
            priority: 100,
            sender: tx1,
        });
        q.heap.push(QueueItem {
            priority: 10,
            sender: tx2,
        });

        // Assert: Highest priority first.
        assert_eq!(q.heap.pop().unwrap().priority, 100);
        assert_eq!(q.heap.pop().unwrap().priority, 10);
    }
}
