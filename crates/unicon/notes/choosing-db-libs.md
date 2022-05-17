## Choosing DB libraries.

I have chosen to wrap Rust DB libraries into a UniCon and UniTx `enum` instead of:

- Using something like `sqlx` (one interface for all SQL libs).
    - I checked SQLite support, it seems to be an afterthought.
    - It violates the "use tools for their primary purpose" principle.
        - It is an async interface, which is not useful for TD as there is only ever one TX being worked on.
            - Async database interfaces are useful for webservers that are serving many read queries.
            - TD is just a CLI with a single task/user, and database writes happen sequentially to a single TX.
        
        - It works for all database's, but it looks like they either:
            - 1. Wrap another db lib.
            - 2. Create their own.
        - Both of which are bad:
            - 1. May as well use the db lib without the wrapping.
            - 2. Probably low quality as there are 4 DB engines to focus on, with the lowest common features being implemented.
    
- Using generic functions.
    - Generic functions are complicated when you have a small set of ahead-of-time known concrete types.
        - I feel an enum in this case is better, as I am creating the interface directly for use specifically in TD, not as a general library.
    

### MySQL.

Code lines counted with `tokei`.

Options:
- rust-mysql-simple, 5.6k
- sqlx, 35.3k
- diesel, 59.3k

Attributes that matter:
- Complexity.
    - Ability to fix any issues or add features.
    - Compile time cost.

- Specificity.
    
rust-mysql-simple wins for these attributes:
- Complexity
    - It has 25% of the lines of code as sqlx.
    - It should not add too much compile time overhead.

- Specificity
    - MySQL is the primary use case, so it likely works very well for that engine.
