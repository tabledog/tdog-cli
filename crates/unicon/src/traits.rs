#![allow(warnings)]

use log::error;

use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::hash::BuildHasherDefault;

use mysql::*;
use mysql::chrono::NaiveDateTime;
use mysql::prelude::*;
use regex::Regex;
use rusqlite::{Connection, NO_PARAMS, params, Result, Row, ToSql, Transaction, TransactionBehavior};
use rusqlite::ffi::Error;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef};
use serde::{Deserialize, Deserializer, Serialize};
use twox_hash::XxHash;
use crate::engines::placeholder::{PlaceholderString, PlaceholderFuncStd, PlaceholderFuncStdStatic};
use crate::engines::mysql::{MySQLFuncX, MySQLFuncXStatic};
use crate::engines::sqlite::{SQLiteFuncRusqlite, SQLiteFuncRusqliteStatic};
use crate::table::{Table, ObjType, CreateSQLObj, StaticSQLStrings, ToSQLString};
use crate::utx::UniTx;
use crate::uc::{UniCon, Schema};
use crate::uc::UniCon::Postgres;
use crate::engines::postgres::{PostgresFuncX, PostgresFuncXStatic};
use crate::params::{ToVecParamsMySQL, ToVecParamsSQLite, ToWhere, ToWhereIndexedSQLite, ToVecParamsPostgres, ToWhereIndexedPostgres};


// type MySQLTransaction<'a> = mysql::conn::transaction::Transaction<'a>;


// @todo/low
// Use as a base trait to enable casting to SQLString?
// - E.g. `trait SQLiteString: SQLString {}`
//      - Issue: Must `impl SQLiteString` AND `impl SQLString`; its not inheritance.
//      - Issue: Some SQL dialects/engines may require different SQL strings/funcs.
//          - Using `SQLiteString` is more or less the same as `SQLString<SQLite>`, but allows engine specific functions.
// - Why?
//      - This would enforce descendant traits to implement the same functions (instead of just convention).
// @see https://stackoverflow.com/questions/28632968/why-doesnt-rust-support-trait-object-upcasting
// @see https://stackoverflow.com/questions/47965967/subclassing-traits-in-rust
// @see https://doc.rust-lang.org/stable/rust-by-example/trait/dyn.html
//      - Return any struct that implements a trait using `Box<dyn Animal>` (runtime pointer to a struct on the heap).
trait SQLString {
    fn get_create() -> String;
    fn get_insert() -> String;
}


// @todo/low Add a general SQL meta data trait for table, column, index meta data.

// General meta data about the table/rows/types.
// - Not SQL dialect/engine specific.
// - Used for: Writing custom SQL where data is needed from the parsed AST of a struct.
//      - E.g. the "table_name" may be set via a struct field attribute, and need to be used at runtime as well.
pub trait TableTr {
    fn get_table(&self) -> &Table;


    fn get_sqlite_insert(&self) -> &str {
        self.get_table().static_sql_strings.as_ref().unwrap().sqlite.insert.as_str()
    }

    fn get_mysql_insert(&self) -> &str {
        self.get_table().static_sql_strings.as_ref().unwrap().mysql.insert.as_str()
    }

    fn get_postgres_insert(&self) -> &str {
        self.get_table().static_sql_strings.as_ref().unwrap().postgres.insert.as_str()
    }

    fn get_table_name(&self) -> &'static str;

    /// Note: The SQL string for `FOREIGN KEY...` cannot be completed generated at macro expansion time.
    /// - The `from` endpoint of the edge (parent table) could have any table name, and this needs to be retrieved via `s1.get_table_name()`
    /// - Macros cannot store state to communicate, *so each row-struct has its own private macro state* and cannot see the name of other tables that may be renamed.
    // fn get_fk_data(&self) -> Vec<FK>;

    // fn get_cols()
    // fn set_primary_key(&self, new_id: i64)

    fn get_key_pk(&self) -> &'static str;
    // fn get_key_insert_ts(&self) ->  Option<&'static str>;
    fn get_key_update_ts(&self) -> Option<&'static str>;
}


/// Separate static and instance functions so that `&dyn Table` still works.
/// - Static functions cause "trait not object safe" compile errors.
///     - This includes any trait or its ancestors (to have &dyn TraitLevel5, Traits0-4 must all contain only instance methods (take &self as arg 0)).
/// - Static is used for going from no data to row instances, instance functions are typically for moving data from Rust to the SQL engine.
/// @see https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6e6aae6da60ce95340b4a3c882d48382
pub trait TableStatic {
    fn get_table() -> &'static Table;
    fn get_table_name_static() -> &'static str;
}


pub trait DbStatic {
    fn get_tables() -> Vec<&'static Table>;

    fn get_table_names() -> Vec<&'static str>;


    fn get_target_schema_sqlite() -> Schema {
        Self::get_tables().iter().map(|x| x.static_sql_strings.as_ref().unwrap().sqlite.clone()).collect::<Vec<_>>().into()
    }

    fn get_target_schema_mysql() -> Schema {
        Self::get_tables().iter().map(|x| x.static_sql_strings.as_ref().unwrap().mysql.clone()).collect::<Vec<_>>().into()
    }

    fn get_target_schema_postgres() -> Schema {
        Self::get_tables().iter().map(|x| x.static_sql_strings.as_ref().unwrap().postgres.clone()).collect::<Vec<_>>().into()
    }


    fn drop_all(uc: &mut UniCon) {
        let table_names = Self::get_table_names();

        let mut utx = uc.tx_open().unwrap();

        for t in table_names {
            let drop = format!("DROP TABLE {};", t);
            utx.exec_one(&drop);
        }

        utx.tx_close();
    }
}

// Use `RowData` when:
// - `&dyn RowData`
//      - `&dyn X` can only take a single trait (but the compiler can use the `T1 + TN` form for marker traits like `Send`.

// Use `T1 + TN1 when:
// - A subset of TX needed in a trait impl.
//      - Generic trait signatures can list all possible traits (T1 + TX ...), but an `impl` block can choose only a subset of possible ones.
pub trait RowData: SQLiteFuncRusqlite + PlaceholderFuncStd {}

impl<T> RowData for T where T: SQLiteFuncRusqlite + PlaceholderFuncStd {}

pub trait Insert: SQLiteFuncRusqlite + MySQLFuncX + PostgresFuncX + PlaceholderFuncStd + std::fmt::Debug {
    // This returns the last insert ID because Postgres can only get the last insert ID consistently via `INSERT INTO ... RETURNING id`
    // - @todo/maybe Make last_insert_id optional via bool fn arg.
    fn insert(&self, uc: &mut UniCon) -> i64 {
        match uc {
            UniCon::Rusqlite(x) => {
                // @todo/maybe Use InserterConcrete trait: `conn.insert()`
                {
                    let (ins, params) = SQLiteFuncRusqlite::get_ins_and_params(self);
                    let mut stmt = x.c.prepare_cached(ins).unwrap();
                    stmt.execute_named(&params).unwrap();
                }
                uc.get_last_id()
            }
            UniCon::MySQL(x) => {
                let (ins, params) = MySQLFuncX::get_ins_and_params(self);
                x.c.exec_drop(ins, params).unwrap();
                uc.get_last_id()
            }
            UniCon::Postgres(x) => {
                // @see https://stackoverflow.com/questions/2944297/postgresql-function-for-last-inserted-id
                // @see https://github.com/sfackler/rust-postgres/issues/128
                let (ins, params) = PostgresFuncX::get_ins_and_params_vec(self);
                // `INSERT ... RETURNING id`
                x.c.query(ins, &params[..]).unwrap().first().unwrap().get(0)
            }
            UniCon::PlaceholderLibA(x) => {
                // T4::get_ins_data(self);
                dbg!(self.get_vals());
                0
            }
        }
    }

    fn insert_set_pk(&mut self, uc: &mut UniCon) -> i64 {
        let last_id = self.insert(uc);
        self.set_pk(last_id);
        last_id
    }


    fn tx_insert(&self, utx: &mut UniTx) -> i64 {
        match utx {
            UniTx::Rusqlite(tx) => {
                {
                    let (ins, params) = SQLiteFuncRusqlite::get_ins_and_params(self);
                    let mut stmt = tx.prepare_cached(ins).unwrap();
                    stmt.execute_named(&params).unwrap();
                }
                utx.get_last_id()
            }
            UniTx::MySQL(tx) => {
                let (ins, params) = MySQLFuncX::get_ins_and_params(self);
                tx.exec_drop(ins, params).unwrap();
                utx.get_last_id()
            }
            UniTx::Postgres(tx) => {
                let (ins, params) = PostgresFuncX::get_ins_and_params_vec(self);
                // `INSERT ... RETURNING id`
                tx.query(ins, &params[..]).unwrap().first().unwrap().get(0)
            }
            UniTx::PlaceholderLibA(x) => {
                // T4::get_ins_data(self);
                dbg!(self.get_vals());
                unimplemented!();
            }
        }
    }

    fn tx_insert_set_pk(&mut self, utx: &mut UniTx) -> i64 {
        let last_id = self.tx_insert(utx);
        self.set_pk(last_id);
        last_id
    }


    // @todo/maybe fn `insert_then_select_insert_ts` - When inserting the insert ts is set by the SQL server, but the Rust struct field is always `None`.

    // Implementors map pk values to a specific struct field.
    // Note: this panics if the pk has not been set (the code should have obvious scopes where it is and isnt set).
    // @todo/high Issue: APIs have their own primary keys, should both the row and the API data have its own primary key?
    // - E.g. Stripes string based `"id": "cus_EwLCaDjylU0IAm"`
    // - Int ids are useful for determining insert order. Should rows have an insert/update timestamp?
    // Note: i65 used as SQLite cannot store u64 (MySQL can as BIGINT UNSIGNED).
    fn get_pk(&self) -> i64;
    fn set_pk(&mut self, id: i64);
    // `fn get_pk_option`?

    // @todo/next `insert_tx`

    /// @todo/low Strongly type where clause.
    /// - A: `|x| vec![x.f1, x.f4]`
    ///         - Input arg to closure is self - ensures fields are checked.
    ///         - Vec<&dyn ToSql> returned, use `std::ptr_eq` to convert a pointer to a param string (`:f1`).
    fn update(&self, uc: &mut UniCon, w: &'static str) -> u64 {
        match uc {
            UniCon::Rusqlite(x) => {
                let (update, params) = SQLiteFuncRusqlite::get_update_and_params(self, w);
                let mut stmt = x.c.prepare_cached(update.as_str()).unwrap();
                let changes = stmt.execute_named(&params).unwrap();
                // @todo/high Function variant to enforce changes==1 so updates cannot fail silently.
                // - Ensure logic in tests.
                changes as u64
            }
            UniCon::MySQL(x) => {
                let (ins, params) = MySQLFuncX::get_update_and_params(self, w);
                x.c.exec_drop(ins, params).unwrap();
                x.c.affected_rows()
            }
            UniCon::Postgres(x) => {
                let (update, params) = PostgresFuncX::get_update_and_params_vec(self, w);
                x.c.execute(update.as_str(), &params[..]).unwrap()
            }
            UniCon::PlaceholderLibA(x) => {
                // T4::get_ins_data(self);
                dbg!(self.get_vals());
                0
            }
        }
    }

    /// @todo/low Use statement cache of Rusqlite, MySQL.
    fn tx_update(&self, utx: &mut UniTx, w: &'static str) -> u64 {
        match utx {
            UniTx::Rusqlite(tx) => {
                let (update, params) = SQLiteFuncRusqlite::get_update_and_params(self, w);
                let mut stmt = tx.prepare_cached(update.as_str()).unwrap();
                let changes = stmt.execute_named(&params).unwrap();
                changes as u64
            }
            UniTx::MySQL(tx) => {
                let (ins, params) = MySQLFuncX::get_update_and_params(self, w);
                tx.exec_drop(ins, params).unwrap();

                // https://dev.mysql.com/doc/c-api/5.7/en/mysql-affected-rows.html
                // Assumption: CLIENT_FOUND_ROWS is set on the connection so that an `affected_rows()` means "rows matched by the where clause".
                tx.affected_rows()
            }
            UniTx::Postgres(tx) => {
                let (update, params) = PostgresFuncX::get_update_and_params_vec(self, w);
                tx.execute(update.as_str(), &params[..]).unwrap()
            }
            UniTx::PlaceholderLibA(x) => {
                // T4::get_ins_data(self);
                dbg!(self.get_vals());
                0
            }
        }
    }

    fn tx_update_pk(&self, utx: &mut UniTx) -> u64 {
        self.tx_update(utx, self.get_key_pk())
    }


    fn delete(&self, uc: &mut UniCon, w: &'static str) -> u64 {
        match uc {
            UniCon::Rusqlite(x) => {
                let (delete, params) = SQLiteFuncRusqlite::get_delete_and_params(self, w);
                let mut stmt = x.c.prepare_cached(delete.as_str()).unwrap();
                let changes = stmt.execute_named(&params).unwrap();
                changes as u64
            }
            UniCon::MySQL(x) => {
                let (delete, params) = MySQLFuncX::get_delete_and_params(self, w);
                x.c.exec_drop(delete, params).unwrap();
                x.c.affected_rows()
            }
            UniCon::Postgres(x) => {
                let (delete, params) = PostgresFuncX::get_delete_and_params_vec(self, w);
                x.c.execute(delete.as_str(), &params[..]).unwrap()
            }
            UniCon::PlaceholderLibA(x) => {
                // T4::get_ins_data(self);
                dbg!(self.get_vals());
                0
            }
        }
    }

    fn tx_delete(&self, utx: &mut UniTx, w: &'static str) -> u64 {
        match utx {
            UniTx::Rusqlite(tx) => {
                let (delete, params) = SQLiteFuncRusqlite::get_delete_and_params(self, w);
                let mut stmt = tx.prepare_cached(delete.as_str()).unwrap();
                let changes = stmt.execute_named(&params).unwrap();
                changes as u64
            }
            UniTx::MySQL(tx) => {
                let (delete, params) = MySQLFuncX::get_delete_and_params(self, w);
                tx.exec_drop(delete, params).unwrap();
                tx.affected_rows()
            }
            UniTx::Postgres(tx) => {
                let (delete, params) = PostgresFuncX::get_delete_and_params_vec(self, w);
                tx.execute(delete.as_str(), &params[..]).unwrap()
            }
            UniTx::PlaceholderLibA(x) => {
                // T4::get_ins_data(self);
                dbg!(self.get_vals());
                unimplemented!();
            }
        }
    }
}



// Functions that can be called via `T::x(uc)`, where `T` is a row-struct.
// - E.g. `RowA::get_last(uc)`
// - Treats the RowA as Vec.
pub trait QueryByStatic: SQLiteFuncRusqliteStatic + MySQLFuncXStatic + PostgresFuncXStatic + PlaceholderFuncStdStatic {
    // @todo/low Generate an enum of field names, use enum instance instead of `&str`;
    // Get the last row from a table.
    // E.g. When getting the last `run` row to determine how much time has passed since the last run.
    fn get_last(uc: &mut UniCon, w: &'static str) -> Option<Self> where Self: Sized {
        // @todo/low Check `w` is a field of the struct at compile time.
        let std_sql = format!("SELECT * FROM {} ORDER BY {} DESC LIMIT 1", Self::get_table_name_static(), w);
        uc.get_vec_from_sql(std_sql.as_str()).into_iter().next()
    }

    // Reads all rows from SQL table to Rust Vec.
    fn get_all(uc: &mut UniCon) -> Vec<Self> where Self: Sized {
        let (table, pk_col) = (Self::get_table_name_static(), Self::get_table().get_primary_key_col_name());
        let std_sql = format!("SELECT * FROM {} ORDER BY {} ASC", table, pk_col);
        uc.get_vec_from_sql(std_sql.as_str())
    }


    // Match one or more keys for a single table.
    fn get_where<T>(uc: &mut UniCon, p: T) -> Vec<Self>
        where Self: Sized,
              T: ToWhere {
        let mut v = vec![];
        let table = Self::get_table_name_static();
        let q = |placeholders| {
            let x = format!("SELECT * FROM {} WHERE {}", table, placeholders);
            x
        };

        match uc {
            UniCon::Rusqlite(x) => {
                let mut stmt = x.c.prepare_cached(q(ToWhereIndexedSQLite::to_where(&p)).as_str()).unwrap();
                let params = ToVecParamsSQLite::to_vec(&p);
                let mut rows = stmt.query(&*params).unwrap();
                while let Some(x2) = rows.next().unwrap() {
                    v.push(<Self as SQLiteFuncRusqliteStatic>::row_to_ins(&x2));
                }
            }
            UniCon::MySQL(x) => {
                let vals = ToVecParamsMySQL::to_vec(&p);


                // Prevent trying to match JSON (as this will result in no rows returned for matching rows).
                for x in &vals {
                    match x {
                        mysql::Value::Bytes(b) => {
                            let s = std::str::from_utf8(&b).unwrap();
                            let is_json = s.starts_with("{") && s.ends_with("}") || s.starts_with("[") && s.ends_with("]");
                            if is_json {
                                error!("Cannot use JSON in a WHERE clause when using MySQL as `cast('json_str' as JSON) needs to be used and it is not yet implemented. This error is thrown to prevent invalid logic as rows will be returned when matching JSON with SQLite and Postgres.");
                                error!("{}", s);
                                panic!("JSON used in MySQL WHERE clause but this is not yet implemented.");
                            }
                        }
                        _ => {}
                    }
                }


                let params = Params::Positional(vals);
                let res = x.c.exec(q(ToWhereIndexedSQLite::to_where(&p)).as_str(), params).unwrap();
                for mut x2 in res {
                    v.push(<Self as MySQLFuncXStatic>::row_to_ins(&mut x2))
                }
            }
            UniCon::Postgres(x) => {
                let params = ToVecParamsPostgres::to_vec(&p);
                let res = x.c.query(q(ToWhereIndexedPostgres::to_where(&p)).as_str(), &*params).unwrap();
                for mut x2 in res {
                    v.push(<Self as PostgresFuncXStatic>::row_to_ins(&mut x2))
                }
            }
            UniCon::PlaceholderLibA(x) => {
                unreachable!()
            }
        }

        v
    }

    // Usage:
    // - `let mut x: Vec<License> = License::get_where_tx(utx, ("sub_id", &sub_id));`
    // - `let mut x: Vec<License> = License::get_where_tx(utx, (("sub_id", &sub_id), ("x", "example_val"));`
    // - etc.
    fn get_where_tx<T>(utx: &mut UniTx, p: T) -> Vec<Self>
        where Self: Sized,
              T: ToWhere {
        let mut v = vec![];
        let table = Self::get_table_name_static();
        let q = |placeholders| {
            let x = format!("SELECT * FROM {} WHERE {}", table, placeholders);
            x
        };

        match utx {
            UniTx::Rusqlite(tx) => {
                let mut stmt = tx.prepare_cached(q(ToWhereIndexedSQLite::to_where(&p)).as_str()).unwrap();
                let params = ToVecParamsSQLite::to_vec(&p);
                let mut rows = stmt.query(&*params).unwrap();
                while let Some(x2) = rows.next().unwrap() {
                    v.push(<Self as SQLiteFuncRusqliteStatic>::row_to_ins(&x2));
                }
            }
            UniTx::MySQL(tx) => {
                let vals = ToVecParamsMySQL::to_vec(&p);


                // Prevent trying to match JSON (as this will result in no rows returned for matching rows).
                for x in &vals {
                    match x {
                        mysql::Value::Bytes(b) => {
                            let s = std::str::from_utf8(&b).unwrap();
                            let is_json = s.starts_with("{") && s.ends_with("}") || s.starts_with("[") && s.ends_with("]");
                            if is_json {
                                error!("Cannot use JSON in a WHERE clause when using MySQL as `cast('json_str' as JSON) needs to be used and it is not yet implemented. This error is thrown to prevent invalid logic as rows will be returned when matching JSON with SQLite and Postgres.");
                                error!("{}", s);
                                panic!("JSON used in MySQL WHERE clause but this is not yet implemented.");
                            }
                        }
                        _ => {}
                    }
                }


                let params = Params::Positional(vals);
                let res = tx.exec(q(ToWhereIndexedSQLite::to_where(&p)).as_str(), params).unwrap();
                for mut x2 in res {
                    v.push(<Self as MySQLFuncXStatic>::row_to_ins(&mut x2))
                }
            }
            UniTx::Postgres(tx) => {
                let params = ToVecParamsPostgres::to_vec(&p);
                let res = tx.query(q(ToWhereIndexedPostgres::to_where(&p)).as_str(), &*params).unwrap();
                for mut x2 in res {
                    v.push(<Self as PostgresFuncXStatic>::row_to_ins(&mut x2))
                }
            }
            UniTx::PlaceholderLibA(x) => {
                unreachable!()
            }
        }

        v
    }


    // Experiment: Using `Vec<(key, value)>` instead of tuple with generics.
    // Note: Results in `error[E0038]: the trait `AnyParam` cannot be made into an object`.
    // - MySQL uses a enum concrete type which requires Clone, which requires Sized, which prevents using as an trait object.
    // - `Vec<T: AnyParam>` will not work as the T generic param will need to resolve to a single concrete type.
    // fn get_where_x<T>(uc: &mut UniCon, p: Vec<(&str, &dyn AnyParam)>) -> Vec<Self>
    //     where Self: Sized,
    //           T: AnyParam
    // {}


    // @todo/low Impl `utx.get_vec_from_sql` like above for transactions.
    // @todo/low Search for all `match utx`, `match uc` in this code and in the Stripe code, and extract general patterns (there is probably some amount of duplication that can be standardised into a re-usable trait interface).
    fn tx_get_last(utx: &mut UniTx, w: &'static str) -> Option<Self> where Self: Sized {
        // @todo/low Check `w` is a field of the struct at compile time.
        let std_sql = format!("SELECT * FROM {} ORDER BY {} DESC LIMIT 1", Self::get_table_name_static(), w);

        match utx {
            UniTx::Rusqlite(tx) => {
                let mut stmt = tx.prepare_cached(std_sql.as_str()).unwrap();
                let mut rows = stmt.query(NO_PARAMS).unwrap();
                if let Some(r) = rows.next().unwrap() {
                    return <Self as SQLiteFuncRusqliteStatic>::row_to_ins(&r).into();
                }
            }
            UniTx::MySQL(tx) => {
                if let Some(mut r) = tx.exec_first(&std_sql, Params::Empty).unwrap() {
                    return <Self as MySQLFuncXStatic>::row_to_ins(&mut r).into();
                }
            }
            UniTx::Postgres(tx) => {
                if let Some(mut r) = tx.query(std_sql.as_str(), &[]).unwrap().first() {
                    return <Self as PostgresFuncXStatic>::row_to_ins(&mut r).into();
                }
            }
            UniTx::PlaceholderLibA(x) => {
                return None;
            }
        }
        None
    }
}


pub trait OracleString {
    fn get_create(&self) -> String;
    fn get_insert(&self) -> String;
}


// @todo/next test this, remove `impl Insert for X {}` from macro.
// impl<T> Insert for T where T: SQLiteFuncRusqlite + PlaceholderFuncStd {}

// These traits are intended to be implemented on each sql-libraries connection and tx types.
// - They will enable two types of function arg:
//      - `a(x: impl InserterConcrete)` OR `a<T>(x: T) where T: InserterConcrete`
//      - `b(y: &dyn InserterDyn)` OR `Vec<&dyn InserterDyn>`
// - They enable passing the concrete connection struct to functions (instead of using enums).
// - enums will be preferred to start with, but this approach may be used instead of/in addition to enums.

// @see evernote:///view/14186947/s134/2c8ff859-f8a0-4ad6-8b7f-b4772ff9e1d0/2c8ff859-f8a0-4ad6-8b7f-b4772ff9e1d0/
// - (enum * enum) vs (dyn * dyn).

// Use generics to allow:
// - Keep functions calls static/compile time by avoiding dyn.
// - Enable both generic/dyn APIs so each one can be used when needed.
//      - Prefer generic when possible (the types are concrete).
pub trait InserterConcrete {
    // @todo/low When reading/writing JSON serde_json::Value, ensure that only the Value is only an object or array (not scalar).
    fn insert<T: SQLiteFuncRusqlite + PlaceholderFuncStd>(&self, r: &T) -> bool;
}

// Avoid generics to allow:
// - `Vec<&dyn InserterDyn>`
//      - Using itself as a trait object to avoid E0038.
// - Accepting `&dyn RowData`
//      - This would enable processing every combination of list A * B, where A is any connection, and B is any struct type.
pub trait InserterDyn {
    fn insert(&self, r: &dyn RowData) -> bool;
}

impl InserterConcrete for Connection {
    // Note: `T` is a subset of the trait signature.
    fn insert<T: SQLiteFuncRusqlite>(&self, r: &T) -> bool {
        let (ins, params) = SQLiteFuncRusqlite::get_ins_and_params(r);
        let mut stmt = self.prepare_cached(ins).unwrap();
        stmt.execute_named(&params).unwrap();
        true
    }
}
