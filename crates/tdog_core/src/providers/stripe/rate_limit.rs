use std::collections::BinaryHeap;
use std::pin::Pin;
use tokio::sync::Mutex;
use tokio::time::{delay_for, Duration};
use tokio::task::JoinHandle;
use futures::FutureExt;
use futures::future::{BoxFuture, LocalBoxFuture};
use futures::future::join_all;
use std::cmp::Ordering;
use std::sync::Arc;
use std::future::Future;
use crate::providers::stripe::queue::Queue;
use async_trait::async_trait;

// Handles the "backpressure" issue by queueing up futures that send requests to an external resource.
// - In place - no need to Box<Future> and move them to a queue.
//      - Keep the future tree.
//      - Reduce lifetime/generic/trait issues.
//
// Modelled after `FutureExt` from `github.com-1ecc6299db9ec823/futures-util-0.3.8/src/future/future/mod.rs`
#[async_trait]
pub trait RateLimit: Future {
    // Queue High
    async fn q_high(self, q_mt_a: &Arc<Mutex<Queue>>) -> Self::Output
        where
            Self: Sized,
    // A: Future<Output = Self::Output>
    {
        let wait = async {
            let rx = {
                let mut q = q_mt_a.lock().await;
                q.add_high()
            };

            // Wait to be scheduled.
            rx.await.unwrap();
        };


        wait.await;
        self.await
    }

    // Queue Low
    async fn q_low(self, q_mt_a: &Arc<Mutex<Queue>>) -> Self::Output
        where
            Self: Sized,
    // A: Future<Output = Self::Output>
    {
        let wait = async {
            let rx = {
                let mut q = q_mt_a.lock().await;
                let f = q.add_low();
                f
            };

            // Wait to be scheduled.
            rx.await.unwrap();
        };

        wait.await;
        self.await
    }
}


impl<T: Sized> RateLimit for T where T: Future {}
// impl<T: ?Sized> RateLimit for T where T: Future {}


// Note: this approach to retries does not work.
// - Once you have a future that is not-yet-started, you cannot clone it ((fn, args)) to retry it.
//      - This means the function call site will have to be wrapped in a closure to capture the fn args and allow retrying the request by re creating a future.
//      - It also does not work with streams, as they wrap the pagination state, so the retry logic needs to go inside the stream implementation function (network API retries can only happen inside that function, not outside it).
//
// Fix: Place the retry logic inside of StripeClient.

// use stripe_client::http::http::UniErr;
// use futures::stream::Next;
//
// #[async_trait]
// pub trait RateLimitStripeSt: RateLimit {
//     async fn q_high(self, q_mt_a: &Arc<Mutex<Queue>>) -> Self::Output
//         where Self: Sized
//     {
//         dbg!("q_high RateLimitStripeSt");
//
//         RateLimit::q_h(self, &q_mt_a).await
//
//     }
// }
//
//
// #[async_trait]
// pub trait RateLimitStripe: RateLimit {
//     async fn q_high(self, q_mt_a: &Arc<Mutex<Queue>>) -> Self::Output
//         where Self: Sized
//     {
//         dbg!("q_high RateLimitStripe");
//
//         RateLimit::q_h(self, &q_mt_a).await
//     }
//
//     async fn q_low(self, q_mt_a: &Arc<Mutex<Queue>>) -> Self::Output
//         where Self: Sized
//     {
//         dbg!("q_low RateLimitStripe");
//
//         RateLimit::q_l(self, &q_mt_a).await
//     }
// }
//
// impl<T, X> RateLimitStripeSt for T where T: Future<Output = Option<Result<X, UniErr>>> {}
// impl<T, X> RateLimitStripe for T where T: Future<Output = Result<X, UniErr>> {}





