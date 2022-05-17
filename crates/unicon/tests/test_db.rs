use chrono::{NaiveDateTime};

// use crate::*;

use unicon_proc_macro::{*};
use lazy_static::lazy_static;



// @todo/low Macro expands to code that depends on this. `use` in macro code which does not conflict with usage scope.
use std::collections::HashMap;

use rusqlite::{params, Connection, Result, ToSql};

use unicon::{*};
use crate::table::Table;
use crate::engines::placeholder::{*};
use crate::engines::sqlite::{*};
use crate::engines::mysql::{*};
use crate::engines::postgres::{*};


use crate::traits::{*};
use crate::uc::{*};
use unicon_proc_macro::{
    Insert,
    Db,
};

use unicon_proc_macro::{
    SQLiteString,
    MySQLString,
    SQLiteFuncRusqlite,
};
// use uc_macro::{PlaceholderString, PlaceholderFuncStd};



use mysql::*;
use mysql::prelude::*;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash;
use crate::dt3::DT3;
use crate::dt::DT;
use crate::table::CreateSQLObj;


// @todo/low Is this better done with generics (that will treat each row the same via a trait (create/insert))?
// - Issue: an enum will require pattern matching but then do the same actions on each branch?
#[derive(Debug, Clone, PartialEq)]
#[derive(Db)] // @todo/low merge this with `EngineXStringSchema`?
pub enum DbTest {
    RowA(RowA),
    RowB(RowB),
    RowC(RowC),
    RowD(RowD),
}

// @todo/med Update.
// - Allow partially setting some of a structs fields and describing a "where" clause.
// - Check at compile time.

// Options:
// - A
//      - Macro creates an identical struct, `${name}Update`, which nests all fields in a Option (representing if its included in the update).
//      - From/to implemented between those types.
//      - Where is `${name}Update::where(|x| x.field = Some(x))`:
// - B
//      - `let update = vec![CustomerUpdate::FieldX(val)]`;

// #[derive(Table, ColMeta, SQLiteString, SQLiteFuncRusqlite)]
// #[derive(PlaceholderString, PlaceholderFuncStd)]


type Value = serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
#[table_name_plural(false)]
#[index("CREATE INDEX row_a_index ON self (row_a_id, some_fk)")]
#[index("CREATE INDEX some_other_index ON self (row_a_id asc, some_fk desc)")]
// Assert: `String` column that is used in an index uses a SQL dialect string type that is index-able (MySQL: VARCHAR(255), not TEXT)
#[index("CREATE INDEX test_mysql_varchar_255_not_text ON self (desc_x)")]
pub struct RowA {
    #[primary_key]
    pub row_a_id: Option<i64>,

    // @todo/maybe should this be a strongly type connection to another struct field? Like `RowB.row_b_id`?
    // - How to enforce FK constraints at the SQL engine? Use alias: `type IdRowA = i64`?
    #[unique]
    pub some_fk: Option<i64>,

    #[unique]
    pub fk_uuid: String,

    pub title: String,

    pub desc_x: String,

    pub json_opt: Option<Value>,

    // @todo/next Ensure correct type.
    pub a_bool: bool,

    pub is_nullable: Option<String>,

    pub float_x: f64,

    pub plain_dt3: Option<DT3>,
    pub plain_dt: Option<DT>,

    #[update_ts]
    pub update_ts: Option<DT3>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    // Issue: When reading/selecting, row_c will have default values. But when writing, row_c may map to a node in the JSON tree that is never Option (so it cannot always be Option).
    // - Moving from a JSON tree into rows is a lossy step - you loose the structure of JSON, and gain the flat structure of SQL that uses relations at the query level.
    //      - So going back to the JSON tree structure is not possible without many sub queries.
    //          - The purpose of TD is to go from a JSON tree into SQL as efficiently as possible.
    //              - It is not a general ORM for applications, and does not support reading tree objects from the SQL DB and writing them back.
    //                  - Writes will be via the SAAS API, which will then sync back to the SQL database, similar to how React writes are sent to and then propagate from the root node to the descendants.
    #[skip]
    pub row_c: RowC,
}


#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
// #[derive(Table, ColMeta, SQLiteString, SQLiteFuncRusqlite)]
pub struct RowB {
    #[primary_key]
    pub row_b_id: Option<i64>,
    pub title: String,
    pub desc_x: String,
}


#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
#[fk("FOREIGN KEY(row_a_id) REFERENCES RowA(row_a_id)")]
#[fk("FOREIGN KEY(row_b_id, row_b_title) REFERENCES RowB(row_b_id, title)")]
#[fk("FOREIGN KEY(type) REFERENCES RowB(title)")]
// @todo/low Allow indexes with Rust reserved keywords.
// #[index("CREATE INDEX type ON self (type)")]
pub struct RowC {
    #[primary_key]
    pub row_c_id: Option<i64>,
    pub title: String,

    // Raw identifier for restricted Rust keywords (that are valid SQL keywords).
    pub r#type: String,

    pub row_a_id: i64,

    pub row_b_id: i64,
    pub row_b_title: String,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}



#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct RowD {
    #[primary_key]
    pub row_d_id: Option<i64>,

    // Note: This allows `null`, which is the default value.
    // - Keep this in RowD so that default still works for RowA.
    // - Without `Option` means: this will always have a JSON object present.
    pub json_direct: Value,
}

// @todo/maybe; one hashmap for each Row type, stored in a single struct?
// E.g.: `struct DbRows {row_a: Hashmap<i32, RowA>, ...}`
// - This would allow representing segments of a table after insert, which the user can inspect by iterating and joining themselves.


// @todo/maybe Strongly typed foreign keys to prevent setting fk fields from the wrong type.
// - Method A.
//      - `struct Row_B_FK_FieldX(i32)`
//          - Generate with proc macro from field attribute.
//      - `From<RowA> for Row_B_FK_FieldX`
//          - Map RowA primary key to Row_B_FK_FieldX
//      - row_b.set_fk<Row_B_FK_FieldX>(row_a)
//          - row_a.into().set_fk(row_b)
//              - row_b.field_fk_x = i32


// @todo/next `trait set_pk(&mut self, i32)`.

// @see https://stackoverflow.com/questions/64406203/rust-how-to-enforce-super-trait-without-implementing-the-functions
// @see https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=9de0dd2b85aab9579a5f8481128242c7
