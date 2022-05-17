@todo

- Do not require users to depend on every SQL engine lib.
    - E.g: The macro references types from (mysql, rusqlite, ...), so when `#[derive(Insert)]]` is used an opaque
      compile error occurs which only points at the struct attribute line.
    - https://stackoverflow.com/questions/64288426/is-there-a-way-to-have-a-public-trait-in-a-proc-macro-crate
        - Re-export the types.


- UniScalar
    - To/from implemented for all scalar<->db_engine_x combos?


- A way to normalise SQL query argument placeholders (and SQL queries, output types).
  
    - E.g. Postgres only uses `$n`:
        - `UPDATE {} SET discount = $1 WHERE id= $2`
    - Others use `?`
        - `UPDATE {} SET discount = ? WHERE id = ?`
    - Some use key based args, with different prefixes:
        - `UPDATE {} SET discount = :discount WHERE id = :id`
    
    - See TD Stripe code (each query is re-written for Postgres).    


    - This is an issue as simple statements need to be re-written even though they differ only by arg syntax.
        - Java based JOOQ represents SQL in an AST that it then re-writes for a given dialect.
    

    - Idealy it would be good to be able to have a function signature for writing generic queries/updates:
        
        - `run<T>(uc: &mut UniCon, sql: GenericSQLAST, params: GenericSQLParams) -> Vec<T>`

        - `run<(i32, String, Option<String>, serde_json::Value)>(uc, sql.into(), params.into())`
        
        - This way the end user just needs to:
            
            - Write the SQL query once (ignoring small differences between each engine).
            - Map params from Rust primtives to a single param type, which can be done automatically.
            - Represent the output set as a generic type.
                - Can work with multi table joins as long as the types match.
            
    - How does Go/Java do this? Do their common interfaces just fail on SQL differences?
        - E.g. it would not be possible just to run MySQL queries on Postgres without changing the param syntax?
            
    - @see https://stackoverflow.com/questions/38697600/how-to-convert-mysql-style-question-mark-bound-parameters-to-postgres-style
        - Note: `$1` acts like a string key as it can be re-used in a single SQL string (as opposed to `?` which cannot).
            


- Queryable for (UniCon or UniTx)
    - Issue: Queries have to be written twice for each of these enums, but they are essentially the same thing just with a different remote server state (inisde/outside a tx).
    - Fix: A single interface that allows running queries regardless of if you have a UinCon or a UniTx.
        - See how Rusqlite/Postgres achieve this.
        - Example of duplicated code: `get_where`, `get_where_tx`.


- RAM target.
    - Issue:
        - The current UniCon/UniTx interface is sync because SQLite's connection is not Send (which is required for moving it to a background thead to unblock the event loop).
        - When doing a lot of inserts (e.g. converting concurrent HTTP responses that represent a list of JSON objects into 100's of inserts), this can block the event loop which prevents any HTTP requests completing causing them to timeout.
        - This is a basic form of backpressure, but in most cases the user has enough RAM to queue all the writes.
        
    - Fix, A:
        - Queue writes in RAM explicitly.
        - Keep the sync interface, but create a new UniCon target which is just RAM (a struct, each field being a table, with a value which is a HashMap of rows). A HashMap insert is 5us (vs 100us for SQLite, 200us for local native Postgres).
        - Once the HTTP download is complete, and the whole dataset is in RAM, convert it to SQL insert statements and batch apply them to reduce the high round trip time to the remote server.
    
    - Fix, B:
        - Write to a local SQLite file, and then convert the SQLite DB to a Postgres/MySQL one using the Insert trait to read the rows and write them back.
            - Question: Would this be a lossy operation as SQLite has the most general SQL types.


### Issues

- When writing custom SQL queries, the user has to implement every sql engine in UniCon/UniTx, which is time consuming.
    - The user will just be copy and pasting boiler plate.
        - They will interact with 1-10 or so compiler errors.
            - The time spent on boiler plate compiler errors could be 0.

    - Fix:
        - macro_x(sql_std, sql_overwrite, row_type: Vec<X, Y, Z>)
            - Apply a macro to a Rust struct representing the tabular results.
            - SQL standard is standard sql, sql overwrite is custom SQL for dialects that do not work with the chosen
              standard SQL dialect.
            - row type will be used to construct the boiler plate for each SQL engine type.
            - Functions will be generated:
                - get_first
                - get_all
            - etc.
            - This would allow the user experience to be:
                - Write SQL query in standard/chosen dialect.
                - Define the tabular output format.
                    - Optional: Deal with SQL dialect errors, create a specific string for a given engine.
                - Use in code.

                - Note: user will not have to `match UniCon` and implement each branch, visiting X different engine
                  API's.
                - This can also be the basis for mapping many tabular results into nested Rust structs.




- Allow using uc/utx across await points in the same thread
    
    - E.g. HTTP handlers can have many await points, but a single db tx for the entire handler.

        - Are await points semantically the same as different threads to the Rust compiler (even though the code may be operating on a single thread and never moved)?

    - This is fixed by using an await compatible Mutex?
        - @see https://stackoverflow.com/a/67277503/4949386
        - No as russqlite Connection is not Send

Avoid this:
```
    = help: within `impl warp::Future`, the trait `Send` is not implemented for `std::sync::MutexGuard<'_, UniCon>`
note: future is not `Send` as this value is used across an await
   --> src/main.rs:245:35
    |
198 |     let mut uc = uc_lock.lock().unwrap();
    |         ------ has type `std::sync::MutexGuard<'_, UniCon>` which is not `Send`
...
245 |                 let mut object = (Object::create("td-data", license_obj_json.into(), file_abs.as_str(), "application/json").await).unwrap();
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here, with `mut uc` maybe used later
```

    