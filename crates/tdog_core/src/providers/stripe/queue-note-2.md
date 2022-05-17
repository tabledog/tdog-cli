

# The aim of the "queue"

- Download data from Stripe to a local database at the maximum possible speed.
    - Each Stripe account has a different rate limit.
    - Other external processes may be taking read capacity.

- Write the async code as if the rate limit of the Stripe API is unlimited.
    - But then queue up by priority and rate limit behind the scenes.
    - High priority requests are the ones that will produce the most child requests, keeping the capacity at maximum speed.
        - This means that if Stripe API limits were lifted, the maximum throughput possible can be achieved.
    
- Avoid `Send` and `'static`.
    - Tokio `spawn` requires `Send`, `spawn_local` requires `'static`.
        - '`static' essentially means no references to stack data (completely owned or moved tree).
            - This is not possible as:
                - Most DB libraries use references to refer to the connection that is owned when converting to a transaction struct.
                - Rusqlite in particular cannot be moved (not Send).
    
    - This means you cannot "run a future in the background but not await it yet".
        - `.await` means "run AND wait".
        - `tokio::spawn*` means "run, wait later"
            - This is not possible without having `'static`.
            - https://stackoverflow.com/questions/69615110/how-can-i-fut-await-to-run-a-future-in-the-background-execute-it-but-not-wa
        - This would be useful to add futures to the queue to keep the queue as large as possible as early on as possible.
            - E.g. looping customers, adding their payment methods (5x requests per customer) to a background queue.

    - This essentially means that you cannot move futures; you must either `.await` them, or `join_all(vec).await`
        - Rust references passed as fn args to these do not seem to need a Mutex or Arc - they operate as if it is the same thread/function stack.
        - A single thread is OK as the Stripe rate limit means that it most likely will never max out the possible CPU time.

    - This reason is why the scheduler never takes a Box<Future>.
        - It just uses messaging, each future gets assigned an ID which is a `oneshot` channel that returns when it is OK to run.
        - By design, the future **stays inside the function scope/AST node** where it will be awaited.
            - In other words: the scheduler will not .await something and pass the result set.
            - The `.await` is always attached to the owning function.
        - This means that `join_all(vec)` can kind of run futures "in the background without Send or 'static".
            - Each branch in this join can then schedule futures at high/med/low. 

# Notes

- This "rate limits" the amount of futures that can be **started** per second.
    - It **does not** monitor if they have completed.
        - This means that starting the futures is rate limited, but if they take a very long time to complete, the futures will queue up in RAM waiting for time on the event loop.
            - E.g. if a DB is slow writing data inside a future, the DB could be the bottleneck (and not an external HTTP API).
                - In this case it should be fine, as the DB write capacity will be used 100%, and RAM will be used to queue up data coming from the network.
                - This is also unlikely as a local database is always going to be much faster than the incoming data from Stripe (due to their API rate limits).
    

- Do not queue over 20k futures.
    - Although these do queue in RAM, they can use 6GB or more.
    - I think it may use less RAM to just keep the data based args in a Vec, and convert them into futures in chunks of 500 and await them.
        - This way it allows using 100% of read capacity for a given Stripe account.
    - CPU is still < 10% in this case.
    - Maybe: Queue tasks like a web crawler using disk based SQLite.
        - Disk is much cheaper than RAM.
        - May allow more concise code as no need to use batch convert args to futures to queue using `chunk`.
    

# Issues

- 429 does not slow down the rate at which items are started.
    - This is a signal from the remote system to slow down. 
  
- "Too many open files" error beyond 20 requests per second.


