# Intro

- This is a Rust package for compile time SQL create and inserts.
- Its a macro that:
    - Converts a Rust struct into SQL string for create/insert statements.
    - Converts an enum into a single string representing the complete db schema.
- Each SQL dialect (the syntax required for a particular engine) has its own `SQLStringX` trait.
- Each (SQL engine, lib) combo has its own `SQLFuncXY` trait that depends on the SQL string traits.
    - Note: Each SQL-engine-lib writes to the same network protocol for that database; there can be many libs with their own types that all write to the same db engine.
- The macro converts the Rust struct into (`SQLStringX` and `SQLFuncXY`) implementations.
    - It does the boiler plate work that the programmer would typically do:
        - Convert struct fields into 
            - `create table` statements.
            - `insert` statements.
    - This enables passing the struct instances into a function that restricts the type to `impl SQLEngineX`, and allows looping over each item to insert it.

# @todo/med

- Updating rows.

- `#[col_type = "json"]`
    - Define and validate root JSON type ({}, [], or scalar).
        - E.g.`#[col_type = "json_array"]` 

- `get_id` trait for enums with a `::String(String)` variant.
    - Enable  generic methods. 

- Use JSON key for enums without values.
    - E.g. `enum X {A, B, C}` stored on a row should insert as "a".
        - Current workaround is to just use String. 


- Output `type TableCol = Option<String>` for each column type so that the types can be used in queries.
    - E.g. a custom query may join and select a few columns, but it should be tied to the column def in the Rust struct some how.

# Not implemented

- The focus is on writing complete rows into tables, not being a general ORM.
- There is no compile time support for partial inserts, or selecting query results.
- The user must map the auto increment primary key to the struct themselves.
    - E.g Before insert the primary key of the struct will be `None`, and after, the user can choose to set it to `Some(x)`.
        - This removes timing and rollback issues from the trait.
        - Nested row inserts that set FK's on each struct can be concisely described in Rust syntax directly.
- Dialect and engine agnostic API.
    - The user must explicitly use a given SQL library; the macro just writes the boiler plate code.
        - In the future more general trait that removes the need for a given library may be added.
    - Rust seems to force a concrete definition at some level.
        - E.g.
            - The code cannot be abstract and general, it must become concrete at some point.
                - It's difficult to make a general interface over many SQL libs as they all use different data types, lifetimes, ownership, timing and errors.
        
           
# General ideas

- A single level Rust struct is the same as a row.
- Rows are atomic units that become more useful when inserted into a SQL engine.
    - The query engine can filter data efficiently.
        - Indexes, query planner.
        - A single struct can be inserted into 5 DB engines, each that may have 10 different query plans.
            - This is 5 different C code bases * 10 different functions = 50 possible ways to run a given query with low developer time spent.
    - Tooling.
        - Libraries and tooling allows better observation by the end user.
    - Network protocols.
        - Many tools are MySQL/Postgres network compatible.    

- Rows are more useful than tree-based data as they use a SQL engine.


# Code

- `sql_macro_test`
    - Testing out the macros for fast iteration.
    - Represents the public interface that is intended used in the wild.
    
- `sql_trait_derive`
    - Macros - Rust code that outputs Rust code.
    
- `sql_trait`
    - Traits used in the macros.


# Usage
- The intention is to keep this workspace as the atomic unit, as Rust macros must be in their own package which means a workspace of packages is required. 

- Locally:
```yaml
uc_lib = { path = "/x/unicon/uc_lib" }
uc_macro = { path = "/x/uc_macro" }
```

```rust
use serde::{Serialize, Deserialize, Deserializer};
use uc_lib::{*};
use uc_macro::{Table, ColMeta, SQLiteString, SQLiteCreateAll, SQLiteFuncRusqlite};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Table, ColMeta, SQLiteString, SQLiteFuncRusqlite)]
pub struct Payments {
    wow: u32
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Table, ColMeta, SQLiteString, SQLiteFuncRusqlite)]
pub struct Customers {
    wow: u32
}

#[derive(Debug, Clone, PartialEq)]
#[derive(SQLiteCreateAll)]
enum Db {
    Payments(Payments),
    Customers(Customers),
}


fn x() {
    let a = Payments { ..Default::default() };

    <Payments as SQLiteString>::get_create();

    let x = SQLiteFuncRusqlite::get_params_kv(&a);

    let schema = Db::get_create_all();

}
```