# Downloading large Stripe accounts 

## Request an increase in your maximum concurrent requests from Stripe
    
- The default maximums are roughly 25 for test accounts, and 100 for live.
- Set your max via the `max_requests_per_second` Table Dog JSON config value.
    - Note that you need to be aware of other processes that may be using this per second request quota too.

## Reduce the possible impact of a long running database transaction

- The first download of the Table Dog CLI is wrapped in a single database transaction.
- Transactions vary between database engines (some support table/row level locking).
- Ensure that a long running transaction will not stop other business processes.
    - If this is the case, you can use the TD CLI against a fresh docker SQL engine, export the first download via
      SQL dump, and apply that to your live database.
        - This will be much faster as there are no HTTP requests, just a flat text file of SQL statements.
    - The next TD CLI run will see the first download and only apply the events in a very short transaction which
      should not impact other processes reading/writing to the database.

## Increase total file descriptors allowed for the TD process

- On Unix-like operating systems, each TCP connection requires a file descriptor.
    - If there are not enough descriptors you will get a `Too many open files` error.
- **Stripe uses the older HTTP 1.1**, which does not allow multiplexing multiple HTTP requests over a single
  connection (like HTTP 2 and HTTP 3).
    - This means 1000 concurrent requests will require 1000 TCP connections/file descriptors.
- macOS processes are limited to 256 by default,
  but [can be increased per process using `ulimit`](https://superuser.com/questions/302754/increase-the-maximum-number-of-open-file-descriptors-in-snow-leopard)


## Do not interrupt the TD CLI process on the first download

- HTTP requests that fail will be retried, but if not resolved after a minute will rollback the transaction and exit the TD CLI process.
- Database connection errors are assumed to be unrecoverable and will cause TD to exit.
- Observe the logs to ensure you are not hitting the HTTP 429 rate limit for your account.
    - Even though the TD CLI may not reach the limit by itself, the sum of all processes using that Stripe account may exceed the "requests per second" limit.


## Minimize network latency

- Run the TD CLI on the same LAN as your database.
    - This will decrease the latency of database writes.
    - HTTP requests are concurrent, but database writes are serialized/blocking due to the open transaction required.
- Stripe servers appear to be inside an AWS US region.
    - Running the TD CLI in a US AWS region will reduce latency. 


## Local databases vs Remote databases.

For the **first download**, the CLI is currently designed with the assumption that the database write rate will always be faster than the API download rate (which is rate limited by Stripe).

This may not be the case if you are using a remote database, or if you are using Docker with QEMU to emulate a local one which can be 10x slower compared to running a database with native code.

If your database cannot keep up with the download rate (**during the first download only**), back pressure will cause each HTTP request to process more slowly, and ultimately hit the HTTP timeout limit, killing the `tdog` process and rolling back the database transaction. This is done to prevent queueing up the data in RAM.

Fixes for this:

- A. Set a slower download rate with the config option `max_requests_per_second`.
    - A rate of `1` will prevent any issues but download very slowly.
    
- B. Use a local database on the same machine, do not use Docker with QEMU.
    

If you want to use a hosted cloud database that is remote to your code, you can workaround this issue by using a local database instance for the first download only, then SQL dump the database, and import it to the remote database. `tdog` will see the initial download has completed, and can apply events from that point onwards in short transactions.

Note: Database writes are done serially because they are inside a transaction, and in some cases basic queries are used to determine the current state.


### Relative database write rates. 

Notes:
- An "object insert" here is 2 database round trips: one for the insert, and another for the write log.
- You can view the durations of each write by using the `debug` log level.
- These are very rough approximations.

| Relative speed | Location | Engine   | Note                                | Microseconds per object | Max object writes per second |
| -------------- | -------- | -------- | ----------------------------------- | ----------------------- | ---------------------------- |
| 1x             | Local    | SQLite   | macOS M1, debug build               | 200                     | 5,000                        |
| 2x             | Local    | Postgres | macOS M1, native                    | 400                     | 2,500                        |
| 20x            | Local    | Postgres | macOS M1, Docker, QEMU              | 4,000                   | 250                          |
| 450x           | Remote   | Postgres | Consumer internet to AWS datacenter | 90,000                  | 11                           |

