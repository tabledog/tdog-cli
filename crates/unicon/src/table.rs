// use std::intrinsics::unreachable;
// use core::panicking::panic;
use serde::{Serialize, Deserialize};
use regex::Regex;
use lazy_static::lazy_static;

// @todo/low Separate code called from macro, and code called from am code.
// - `Table` struct not inside the `uc_macro` crate as proc-macro crates cannot export types (as they are "compiler plugins").
use std::convert::TryFrom;
use syn::export::TokenStream2;
use syn::{Field, Type, PathArguments, GenericArgument, DataStruct, Fields, DataEnum, Data};
use quote::format_ident;
use quote::quote;
use crate::uc::{Schema, TableCreate, NameCreate};
use std::collections::{HashSet, HashMap};
use crate::data::mysql::MYSQL_RESERVED_KEYWORDS;
use crate::data::postgres::POSTGRES_RESERVED_KEYWORDS;


// Supported Rust types that are allowed to be used in "row structs" for inserting.
// - Assumption: these can be mapped to/from SQL types.
// - Map directly from Rust type -> SQL engine X (with no intermediate representation of a "general abstract SQL type")
//      - The IR adds too much indirection and removes some of the semantic meaning of the types as the engines SQL types differ quite a lot.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub enum RustType {
    String,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,

    // From external libs
    // Serde
    Value,

    // Chrono
    // NaiveDateTime,
    // Wrap needed to implement trait for rusqlite::ToSql
    // DateTime with millisecond resolution.
    DT3,

    // Same as DT3, but only second resolution.
    DT,
}

impl From<String> for RustType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "String" => RustType::String,
            "i8" => RustType::I8,
            "i16" => RustType::I16,
            "i32" => RustType::I32,
            "i64" => RustType::I64,
            "u8" => RustType::U8,
            "u16" => RustType::U16,
            "u32" => RustType::U32,
            "u64" => RustType::U64,
            "f32" => RustType::F32,
            "f64" => RustType::F64,
            "bool" => RustType::Bool,
            // @todo/low Support serde_json::Value (Value conflicts with many other libs).
            "Value" => RustType::Value,
            "DT3" => RustType::DT3,
            "DT" => RustType::DT,

            // Note: panic is ok as macro runs  at compile time, not runtime.
            _ => panic!("Type not supported: {}", s)
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub struct_name: String,
    pub name: String,
    pub cols: Vec<Col>,
    pub indexes: Vec<Index>,
    pub foreign_keys: Vec<FK>,

    // `Option` as this is created async from the other Table data.
    pub static_sql_strings: Option<AllEngineStrings>,
}

// Always less than 5 engines; prefer enum/direct data over generics for clearer code.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct AllEngineStrings {
    pub sqlite: StaticSQLStrings,
    pub mysql: StaticSQLStrings,
    pub postgres: StaticSQLStrings,
}


// Use a struct to store this data instead of using functions that recompute these.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct StaticSQLStrings {
    pub create: CreateSQLObj,
    pub indexes: Vec<CreateSQLObj>,
    pub insert: String,
}


// `TableCreate` flattens and removes some of this data, and places it into a format that is easily comparable with reading the existing schema.
// - Reading the "target" and "existing" schema data into the same structure makes comparing/logging them much easier.
//      - Instead of reading the same data from two different structures in a function scope, first move the data to identical structures.
impl From<StaticSQLStrings> for TableCreate {
    fn from(x: StaticSQLStrings) -> Self {
        TableCreate {
            name: x.create.name,
            create: x.create.create.clone(),
            indexes: x.indexes.iter().map(|x2| NameCreate {
                name: x2.name.clone(),

                // Standard SQL
                create: x2.create.clone(),
            }).collect(),
        }
    }
}

impl Table {
    pub fn get_primary_key_col_name(&self) -> String {
        for col in &self.cols {
            if col.primary {
                return col.name.clone();
            }
        }
        panic!("Every struct should have a int primary key.");
    }

    /// Columns/fields that are used for inserts only.
    /// - Used when creating:
    ///     - 1. Insert SQL string
    ///     - 2. Struct fields to insert params tuple.
    /// Meaning: (pk, insert_ts, update_ts) are all like parent packet headers, the rest of the row values are the payload.
    pub fn cols_writable_only(&self) -> Vec<Col> {
        self.cols.clone().into_iter().filter(|x| !(x.skip || x.insert_ts || x.update_ts || x.primary)).collect()
    }

    pub fn cols_not_skipped(&self) -> Vec<Col> {
        self.cols.clone().into_iter().filter(|x| !(x.skip)).collect()
    }

    fn get_insert_ts(&self) -> Option<Col> {
        self.cols_not_skipped().into_iter().find(|c| c.insert_ts)
    }

    fn get_update_ts(&self) -> Option<Col> {
        self.cols_not_skipped().into_iter().find(|c| c.update_ts)
    }

    /// `FOREIGN KEY...` SQL statement as parsed data.
    // pub fn get_ts_fk_data(&self) -> Vec<TokenStream2> {
    //     self.foreign_keys.iter().map(|fk| {
    //         let from_struct = format_ident!("{}", &fk.from.struct_name.as_ref().unwrap());
    //         let from_cols = &fk.from.fields;
    //         let to_cols = &fk.to.fields;
    //
    //         // Note: default objects used as if any static functions are used, `$dyn TX` cannot be used due to not being "object safe",
    //         quote! {
    //             FK {
    //                 from_tbl: <#from_struct as TableTr>::get_table_name(&(#from_struct {..Default::default()})).to_string(),
    //                 from_cols: vec![#(#from_cols.to_string()),*],
    //                 to_tbl: <Self as TableTr>::get_table_name(&(Self {..Default::default()})).to_string(),
    //                 to_cols: vec![#(#to_cols.to_string()),*]
    //             }
    //         }
    //     }).collect()
    // }

    pub fn get_ts_fk_assert_fields_exist(&self) -> Vec<TokenStream2> {
        self.foreign_keys.iter().enumerate().map(|(i, fk)| {
            // Assert: Fields used in FK relations exist.
            let from_struct = format_ident!("{}", &fk.from.struct_name.as_ref().unwrap());
            let from_cols = &fk.from.fields;
            let to_struct = format_ident!("{}", &fk.to.struct_name.as_ref().unwrap());
            let to_cols = &fk.to.fields;

            let f_name = format_ident!("_sql_fk_assert_fields_{}_{}", &self.name, &i);
            let from_fields = from_cols.iter().map(|f| {
                let i = format_ident!("r#{}", f);
                quote! {
                from.#i
            }
            });
            let to_fields = to_cols.iter().map(|f| {
                let i = format_ident!("r#{}", f);
                quote! {
                to.#i
            }
            });

            quote! {
            // Note: Auto generated function that asserts `foreign key` statements reference valid struct fields at compile time.
            fn #f_name(from: #from_struct, to: #to_struct) {
                || {
                    (
                        ( #(#from_fields),* ),
                        ( #(#to_fields),* )
                    )
                };
            }
        }
        }).collect()
    }

    // Parse `#[table_name_plural(false)]` attribute, used on Rust structs.
    // - Some table names cannot be pluralized (E.g. metadata -> metadatas).
    //
    // @see https://stackoverflow.com/questions/56188700/how-do-i-make-my-custom-derive-macro-accept-trait-generic-parameters
    pub fn get_table_name_plural(ast: &syn::DeriveInput) -> bool {
        let attribute = ast.attrs.iter().filter(
            |a| a.path.segments.len() == 1 && a.path.segments[0].ident == "table_name_plural"
        ).nth(0);

        if let Some(x) = attribute {
            return x.tokens.to_string() == "(true)";
        }

        // Default - no attribute.
        true
    }

    // Intended to be run at macro time.
    fn create_static_strings(&mut self) {
        let sqlite = StaticSQLStrings {
            create: <Self as ToSQLString<SQLite>>::get_create_table(self),
            indexes: <Self as ToSQLString<SQLite>>::get_create_indexes(self),
            insert: <Self as ToSQLString<SQLite>>::get_insert(self),
        };

        let mysql = StaticSQLStrings {
            create: <Self as ToSQLString<MySQL>>::get_create_table(self),
            indexes: <Self as ToSQLString<MySQL>>::get_create_indexes(self),
            insert: <Self as ToSQLString<MySQL>>::get_insert(self),
        };

        let postgres = StaticSQLStrings {
            create: <Self as ToSQLString<Postgres>>::get_create_table(self),
            indexes: <Self as ToSQLString<Postgres>>::get_create_indexes(self),
            insert: <Self as ToSQLString<Postgres>>::get_insert(self),
        };

        let all = AllEngineStrings {
            sqlite,
            mysql,
            postgres,
        };

        self.static_sql_strings = Some(all)
    }

    // Returns the columns that conflict with SQL keywords, which would cause a runtime error to be thrown from the SQL engine.
    // - Taking the approach of "Rename keywords instead of quoting them".
    //      - Queries are easier to write without quoting identifiers.
    fn get_col_names_matching_reserved_sql_keywords(&self) -> HashMap<&str, Vec<Col>> {
        let mut hm = HashMap::new();

        for x in &self.cols {
            if MYSQL_RESERVED_KEYWORDS.contains(&x.name.to_uppercase().as_str()) {
                hm.entry("mysql").or_insert(vec![]).push(x.clone());
            }

            if POSTGRES_RESERVED_KEYWORDS.contains(&x.name.to_uppercase().as_str()) {
                hm.entry("postgres").or_insert(vec![]).push(x.clone());
            }

            // Other engines...
        }

        hm
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    pub sql: String,
    pub name: String,
    pub fields_used: Vec<String>,
}

impl Index {
    // Return the set of field names used in indexes (E.g. to ensure they are index-able types, like VARCHAR(X) instead of TEXT in MySQL).
    pub fn get_fields_used(indexes: &Vec<Index>) -> HashSet<String> {
        let mut x = HashSet::new();

        for x2 in indexes {
            for x3 in &x2.fields_used {
                x.insert(x3.clone());
            }
        }

        x
    }
}

/// ForeignKey
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct FK {
    pub sql_raw: String,
    pub from: Endpoint,
    pub to: Endpoint,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct Endpoint {
    pub struct_name: Option<String>,
    pub fields: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct Col {
    pub name: String,
    pub name_raw: String,
    pub t: RustType,

    // @todo/next remove this if the Rust type covers all use cases.
    // When: struct field has col_type. (E.g. SQLite has a JSON column type, but you still insert strings that contain valid JSON.
    // attr_type: Option<RustType>,
    pub nullable: bool,
    pub primary: bool,
    pub unique: bool,

    // If this column is used in a custom index.
    // - E.g. only includes "CREATE INDEX X on Y (Z)"
    //      - Excludes index #[primary] or #[unique].
    // Place this on the column so Rust function scopes that take a column and map it to a SQL dialect can choose the correct type without needing the context of the Table being passed (MySQL will map to VARCHAR(255) instead of TEXT as TEXT has a variable length so cannot be used in an index).
    pub is_used_in_index: bool,

    pub skip: bool,
    pub insert_ts: bool,
    pub update_ts: bool,
    pub attrs: Vec<AttrKey>,
}

impl Col {
    fn to_sqlite_type(&self) -> &str {
        // Note: `INT` != `INTEGER` in the context of `PRIMARY KEY`
        // @see https://stackoverflow.com/questions/20289410/difference-between-int-primary-key-and-integer-primary-key-sqlite/20289487#:~:text=Yes%2C%20there%20is%20a%20difference,separate%20primary%20key%20is%20created.
        let t = match self.t {
            RustType::String => "TEXT",
            RustType::I8 => "INTEGER",
            RustType::I16 => "INTEGER",
            RustType::I32 => "INTEGER",
            RustType::I64 => "INTEGER",
            RustType::U8 => "INTEGER",
            RustType::U16 => "INTEGER",
            RustType::U32 => "INTEGER",
            // Note: SQLite `INTEGER` is a 64 bit *signed*, which means this will silently reduce precision.
            RustType::U64 => unreachable!("Rust type u64 not supported for SQL as its INTEGER column has a max range of i64. This prevents silently reducing the precision of numbers > i64 on insert. Fix: use i64 if suitable."),
            RustType::F32 => "REAL",
            RustType::F64 => "REAL",
            RustType::Bool => "INTEGER",
            // Note: `JSON` is not a valid SQLite type.
            RustType::Value => "TEXT",

            // Date is an ISO date time string.
            RustType::DT3 => "TEXT",
            RustType::DT => "TEXT",
        };

        t
    }

    fn to_mysql_type(&self) -> &str {
        // Note: `INT` != `INTEGER` in the context of `PRIMARY KEY`
        // @see https://stackoverflow.com/questions/20289410/difference-between-int-primary-key-and-integer-primary-key-sqlite/20289487#:~:text=Yes%2C%20there%20is%20a%20difference,separate%20primary%20key%20is%20created.

        let t = match self.t {
            // VARCHAR(>255) used 1 more byte to storage than VARCHAR(<256)
            // @see https://stackoverflow.com/a/5899369/4949386
            // - VARCHAR(255) = 1 byte (length) + string bytes
            // - VARCHAR(>255) = 2 bytes (length) + string bytes.

            // TEXT vs VARCHAR
            // @see https://stackoverflow.com/a/60310946/4949386
            // - Cannot index TEXT (stored as binary data with char set outside of table row).
            RustType::String => if self.unique || self.is_used_in_index {
                // Note: Cannot index TEXT.
                // Expecting UUID-like strings in this field.
                // 255 is supported by < MySQL 5.0.3
                // 255 uses 1 less byte, may improve index/query performance.
                "VARCHAR(255)"
            } else {
                // Need to allow end users to dynamically add indexes (which cannot be done for TEXT).
                // Maybe: In the future, add a #[max_str_size=xMB] for cols that need large TEXT storage (not currently used for Stripe)
                //
                // VARCHAR max = (21840=utf8, 64KB=latin1?)
                // "VARCHAR(21840)" = `ERROR 1118 (42000): Row size too large`
                // - This is the limit for a single column, but the entire row must be under 65535
                //
                // 20,000 / 500 = 40
                // Assumptions:
                // - Each row is < 40 cols.
                // - Transactions will throw errors when the row limit is exceeded, so runtime state should not be incorrect if any schemas go over this.
                "VARCHAR(500)"
            }
            // @see https://dev.mysql.com/doc/refman/8.0/en/integer-types.html
            RustType::I8 => "TINYINT",
            RustType::I16 => "SMALLINT",
            RustType::I32 => "INT",
            RustType::I64 => "BIGINT",
            RustType::U8 => "TINYINT UNSIGNED",
            RustType::U16 => "SMALLINT UNSIGNED",
            RustType::U32 => "INT UNSIGNED",
            RustType::U64 => "BIGINT UNSIGNED",
            // @see https://stackoverflow.com/questions/60070417/mysql-8-should-i-be-able-to-write-a-valid-ieee-754-floating-point-number-and-re
            // - Assumption: This is IEEE Standard 754
            RustType::F32 => "FLOAT",
            RustType::F64 => "DOUBLE",
            // BOOLEAN is an alias of TINYINT in MySQL.
            RustType::Bool => "TINYINT(1)",
            // Note: `JSON` is not a valid SQLite type.
            RustType::Value => "JSON",

            // Date is an ISO date time string.
            // 3 == millisecond precision timestamps.
            // @todo/low infer precision needed from Rust chrono type?
            // @todo/low does this cause `UTC_TIMESTAMP()` to store ms?
            RustType::DT3 => "DATETIME(3)",
            RustType::DT => "DATETIME",
        };

        t
    }

    fn to_postgres_type(&self) -> &str {
        // Note: `INT` != `INTEGER` in the context of `PRIMARY KEY`
        // @see https://stackoverflow.com/questions/20289410/difference-between-int-primary-key-and-integer-primary-key-sqlite/20289487#:~:text=Yes%2C%20there%20is%20a%20difference,separate%20primary%20key%20is%20created.

        let t = match self.t {
            // See MySQL comments.
            // - Assumption: A tx will fail if a string exceeds this size.
            RustType::String => if self.unique || self.is_used_in_index {
                "VARCHAR(255)"
            } else {
                "VARCHAR(500)"
            }
            // @see https://dev.mysql.com/doc/refman/8.0/en/integer-types.html
            RustType::I8 => "SMALLINT",
            RustType::I16 => "SMALLINT",
            RustType::I32 => "INT",
            RustType::I64 => "BIGINT",

            // Postgres does not have `UNSIGNED` numeric types.
            RustType::U8 => "INT",
            RustType::U16 => "INT",
            RustType::U32 => "BIGINT",
            RustType::U64 => "NUMERIC(20)",
            // Both of these are IEEE Standard 754
            RustType::F32 => "REAL",
            RustType::F64 => "DOUBLE PRECISION",
            // BOOLEAN is an alias of TINYINT in MySQL.
            RustType::Bool => "BOOLEAN",
            // JSONB = Binary representation that takes slightly longer to insert, but allows faster query operations.
            RustType::Value => "JSONB",


            // `TIMESTAMP` stores the highest possible precision by default (no way to specify precision).
            RustType::DT3 => "TIMESTAMP",
            RustType::DT => "TIMESTAMP",
        };

        t
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Rows {
    pub struct_names: Vec<String>,
}

// These zero sized structs are used to make a generic type param a concrete type representing a SQL engine (SQL syntax dialect/server side process).
// - It allows implementing the same trait for different engines.
pub struct SQLite;
pub struct MySQL;
pub struct Postgres;

// @see https://stackoverflow.com/questions/64406203/rust-how-to-enforce-super-trait-without-implementing-the-functions
pub trait ToSQLString<T> {
    fn get_create_table(&self) -> CreateSQLObj;
    fn get_create_indexes(&self) -> Vec<CreateSQLObj>;
    fn get_insert(&self) -> String;
}

// Strings that do not require state from a given Rust struct (that represents a row).
// - E.g. SQL functions.
pub trait SQLStaticStrings {
    fn get_utc_now_3() -> &'static str;
}

impl SQLStaticStrings for Postgres {
    // @see https://www.postgresql.org/docs/10/functions-datetime.html#FUNCTIONS-DATETIME-TRUNC
    // Note: 5 digits max returned from `now()`.
    // - Postgres datetime cols do not specify precision so must be truncated to the desired precision before insert.
    fn get_utc_now_3() -> &'static str {
        "date_trunc('milliseconds', timezone('utc', now()))"
    }
}


// - Pass `name` to enable object ref in logs (a list of names is easier to read that the whole SQL schema).
// - Each SQL dialect will use this same object type - but the create string will be correct for that particular engine.
//      - The parent context will distinguish which SQL dialect it is.
// - Enable ordering many objects into one Vec to create the whole database.
// - Create if not exists will be used with the DB changes API to determine if the object was created.
// - Intended for schemas, tables and indexes.
// @todo/maybe remove this?
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct CreateSQLObj {
    pub obj_type: ObjType,
    pub name: String,
    pub create: String,
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub enum ObjType {
    // "Database" = MySQL, SQLite = whole file
    Schema,
    Table,
    Index,
}


impl ToSQLString<SQLite> for Table {
    fn get_create_table(&self) -> CreateSQLObj {
        let cols = self.cols_not_skipped().iter().map(|c| {
            let t = &c.to_sqlite_type();

            let mut v = vec![
                c.name.to_string(),
                t.to_string()
            ];

            if !c.nullable { v.push("NOT NULL".to_string()) }

            // `AUTOINCREMENT` =  `prevent the reuse of ROWIDs from previously deleted rows`
            // - This seems a good default as some systems use max(id) to mean "last event timestamp".
            // @see https://sqlite.org/autoinc.html
            if c.primary { v.push("PRIMARY KEY AUTOINCREMENT".to_string()) }
            if c.unique { v.push("UNIQUE".to_string()) }

            v.join(" ")
        }).collect::<Vec<String>>().join(",\n");

        CreateSQLObj {
            obj_type: ObjType::Table,
            name: self.name.clone(),
            create: format!("CREATE TABLE IF NOT EXISTS {} (\n{}\n)", self.name, cols),
        }
    }

    fn get_create_indexes(&self) -> Vec<CreateSQLObj> {
        let mut v = vec![];
        for x in &self.indexes {
            v.push(CreateSQLObj {
                obj_type: ObjType::Index,
                name: x.name.clone(),
                create: x.sql.clone(),
            });
        }

        v
    }

    fn get_insert(&self) -> String {
        let mut kv: Vec<(String, String)> = vec![];

        /// Set PK via SQL engine auto increment.
        kv.push((
            self.get_primary_key_col_name(),
            "NULL".into()
        ));

        for c in self.cols_writable_only() {
            kv.push((
                c.name.clone(),
                format!(":{}", &c.name)
            ));
        }

        if let Some(c) = self.get_insert_ts() {
            // Column will truncate ms/microseconds depending on the type.
            kv.push((
                c.name.clone(),
                "STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')".into()
            ));
        }

        if let Some(c) = self.get_update_ts() {
            kv.push((
                c.name.clone(),
                "STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')".into()
            ));
        }

        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.name,
            to_csv_keys(&kv),
            to_csv_vals(&kv)
        )
    }
}

impl ToSQLString<MySQL> for Table {
    fn get_create_table(&self) -> CreateSQLObj {
        let cols = self.cols_not_skipped().iter().map(|c| {
            let t = c.to_mysql_type();

            let mut v = vec![
                c.name.to_string(),
                t.to_string()
            ];

            if !c.nullable { v.push("NOT NULL".to_string()) }

            if c.primary { v.push("PRIMARY KEY AUTO_INCREMENT".to_string()) }
            if c.unique { v.push("UNIQUE".to_string()) }

            v.join(" ")
        }).collect::<Vec<String>>().join(",\n");

        CreateSQLObj {
            obj_type: ObjType::Table,
            name: self.name.clone(),
            create: format!("CREATE TABLE IF NOT EXISTS {} (\n{}\n)", self.name, cols),
        }
    }

    fn get_create_indexes(&self) -> Vec<CreateSQLObj> {
        let mut v = vec![];
        for x in &self.indexes {
            v.push(CreateSQLObj {
                obj_type: ObjType::Index,
                name: x.name.clone(),
                create: x.sql.clone(),
            });
        }

        v
    }

    // Note: `mysql crate, src/engines`: MySql itself doesn't have named parameters support, so it's implemented on the client side. One should use `:name` as a placeholder syntax for a named parameter.
    // - https://docs.rs/mysql/20.1.0/mysql/params/enum.Params.html
    //      - Named takes a `String` for every `Value`, does this double the memory needed? Is it better to use Vec<>?
    fn get_insert(&self) -> String {
        let mut kv: Vec<(String, String)> = vec![];

        /// Set PK via SQL engine auto increment.
        kv.push((
            self.get_primary_key_col_name(),
            "NULL".into()
        ));

        for c in self.cols_writable_only() {
            kv.push((
                c.name.clone(),
                format!(":{}", &c.name)
            ));
        }


        if let Some(c) = self.get_insert_ts() {
            // Column will truncate ms/microseconds depending on the type.
            kv.push((
                c.name.clone(),
                "UTC_TIMESTAMP(6)".into()
            ));
        }

        if let Some(c) = self.get_update_ts() {
            kv.push((
                c.name.clone(),
                "UTC_TIMESTAMP(6)".into()
            ));
        }

        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.name,
            to_csv_keys(&kv),
            to_csv_vals(&kv)
        )
    }
}

fn to_csv_keys(kv: &Vec<(String, String)>) -> String {
    kv.iter().map(|x| x.0.clone()).collect::<Vec<String>>().join(", ")
}

fn to_csv_vals(kv: &Vec<(String, String)>) -> String {
    kv.iter().map(|x| x.1.clone()).collect::<Vec<String>>().join(", ")
}


impl ToSQLString<Postgres> for Table {
    fn get_create_table(&self) -> CreateSQLObj {
        let cols = self.cols_not_skipped().iter().map(|c| {
            let t = c.to_postgres_type();

            let mut v = vec![
                c.name.to_string(),
                t.to_string()
            ];

            if !c.nullable { v.push("NOT NULL".to_string()) }
            if c.primary {
                assert_eq!(c.t, RustType::I64, "Expected Postgres bigserial primary key to be of type i64.");
                v.pop(); // Remove type.
                v.push("BIGSERIAL PRIMARY KEY".to_string())
            }
            if c.unique { v.push("UNIQUE".to_string()) }

            v.join(" ")
        }).collect::<Vec<String>>().join(",\n");

        CreateSQLObj {
            obj_type: ObjType::Table,
            name: self.name.clone(),
            create: format!("CREATE TABLE IF NOT EXISTS {} (\n{}\n)", self.name, cols),
        }
    }

    fn get_create_indexes(&self) -> Vec<CreateSQLObj> {
        let mut v = vec![];
        for x in &self.indexes {
            v.push(CreateSQLObj {
                obj_type: ObjType::Index,
                name: x.name.clone(),
                create: x.sql.clone(),
            });
        }

        v
    }

    // Postgres does not have named parameters support (libraries may emulate it).
    fn get_insert(&self) -> String {
        let mut kv: Vec<(String, String)> = vec![];

        // Set PK via SQL engine auto increment.
        // - Postgres throws an error when NULL is passed to an auto incrementing PK field (unlike MySQL that assumes NULL means "generate a new ID")
        // kv.push((
        //     self.get_primary_key_col_name(),
        //     "NULL".into()
        // ));

        // 1-indexed: Postgres params start at 1.
        // Note: params must be given in the same order to this insert statement.
        let mut next_index = 1;
        for c in self.cols_writable_only() {
            kv.push((
                c.name.clone(),
                format!("${}", next_index)
            ));
            next_index += 1;
        }

        if let Some(c) = self.get_insert_ts() {
            kv.push((
                c.name.clone(),
                Postgres::get_utc_now_3().into()
            ));
        }

        if let Some(c) = self.get_update_ts() {
            kv.push((
                c.name.clone(),
                Postgres::get_utc_now_3().into()
            ));
        }

        // Note: `RETURNING id` is needed as Postgres does not support "get_last_id()"
        format!(
            "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
            self.name,
            to_csv_keys(&kv),
            to_csv_vals(&kv),
            self.get_primary_key_col_name()
        )
    }
}


impl From<&Field> for Col {
    fn from(f: &Field) -> Self {
        let name_raw = f.ident.as_ref().unwrap().clone().to_string();

        // Some keywords are invalid Rust field names, but valid SQL columns. Reduce renaming as much as possible.
        // E.g. `r#type` => `type`
        // @see https://www.postgresql.org/docs/current/sql-keywords-appendix.html
        // @todo/low `panic!` if a SQL reserved keyword is used here.
        // @todo/low Alternative: field `rename` attr (like in Serde).
        let name = str::replace(name_raw.as_str(), "r#", "");

        let (mut t, nullable): (RustType, bool) = match &f.ty {
            Type::Path(p) if p.path.segments.len() == 1 => {
                let f = &p.path.segments.first().unwrap();

                let mut t = f.ident.to_string();
                let mut nullable = false;

                // @todo/low Test this with a wider range of possible struct syntax.
                if t.as_str() == "Option" {
                    nullable = true;

                    match &f.arguments {
                        PathArguments::AngleBracketed(x) => {
                            match x.args.first().unwrap() {
                                GenericArgument::Type(x) => {
                                    match x {
                                        Type::Path(x) => {
                                            t = x.path.segments.first().unwrap().ident.to_string();
                                        }
                                        _ => panic!("Expected Path.")
                                    }
                                }
                                _ => panic!("Expected Type.")
                            }
                        }
                        _ => panic!("Expected AngleBracketed.")
                    };
                }

                (t.into(), nullable)
            }
            _ => panic!("Expecting only Path. {:?}", &f.ty)
        };

        // @todo/low https://stackoverflow.com/questions/61169932/how-do-i-get-the-value-and-type-of-a-literal-in-a-rust-proc-macro
        // Can `parse_macro_input` be used? (Is there a way to get a more specific type for attributes than TokenStream?)


        // When: `#[unique]` on struct field.
        let mut primary = false;
        let mut unique = false;
        let mut skip = false;
        let mut insert_ts = false;
        let mut update_ts = false;
        // let mut attr_type = None;
        let mut attrs = vec![];

        f.attrs.iter().for_each(|a| {
            let attr = a.path.segments.first().unwrap().ident.to_string();

            let k: AttrKey = attr.into();
            attrs.push(k.clone());

            match k {
                AttrKey::PrimaryKey => primary = true,
                AttrKey::Unique => unique = true,
                AttrKey::Skip => skip = true,
                AttrKey::InsertTs => {
                    assert_eq!(t, RustType::DT3, "Rust type for DateTime should be chrono `NaiveDateTime` wrapped with DT3");
                    insert_ts = true
                }
                AttrKey::UpdateTs => {
                    assert_eq!(t, RustType::DT3, "Rust type for DateTime should be chrono `NaiveDateTime wrapped with DT3`");
                    update_ts = true
                }
                AttrKey::ColType => {
                    // May use in future.
                    // // E.g. `#[col_type = "json"]`
                    // for t in a.tokens.to_token_stream().into_iter() {
                    //     match t {
                    //         TokenTree::Literal(x) => {
                    //             // @todo/low How do you get the symbol/kind fields that are used in the debug for Literal?
                    //             let s = x.to_string();
                    //             match s.as_str().trim_matches('"') {
                    //                 "json" => {
                    //                     attr_type = Some(SQLType::JSON)
                    //                 }
                    //                 _ => panic!("Unknown `col_type` type")
                    //             }
                    //         }
                    //         _ => {
                    //             // Ignore Punct
                    //         }
                    //     }
                    // }
                }
            }
        });

        Col {
            name,
            name_raw,
            t,
            // attr_type,
            nullable,
            primary,
            unique,
            // Parent scope will write this based on Table-level indexes.
            is_used_in_index: false,
            skip,
            insert_ts,
            update_ts,
            attrs,
        }
    }
}

/// Single keyed macro attributes on struct fields.
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, Debug, Clone)]
pub enum AttrKey {
    PrimaryKey,
    Unique,
    Skip,
    InsertTs,
    UpdateTs,
    ColType,
    // @todo/low `ColType(DataStruct)`. Is there a better way to convert the attribute AST into strongly typed structs?
}

impl From<String> for AttrKey {
    fn from(s: String) -> Self {
        match s.as_str() {
            "primary_key" => Self::PrimaryKey,
            "unique" => Self::Unique,
            "skip" => Self::Skip,
            "col_type" => Self::ColType,
            "insert_ts" => Self::InsertTs,
            "update_ts" => Self::UpdateTs,
            x => panic!("AttrKey does not have variant for key {}", x)
        }
    }
}


/// Get all macro attribute's on struct field (key only, no value).
fn get_attrs(f: &Field) -> Vec<AttrKey> {
    f.attrs.iter().map(|a| {
        let attr = a.path.segments.first().unwrap().ident.to_string();
        attr.into()
    }).collect()
}


// Issue: The visitor pattern would require a state machine to understand which `Ident` are struct names, types, options etc.
// - Fix A: Use Rust pattern matching against the AST instead.
// - Fix B: Tree path based visitor pattern (tells you depth of current node, ancestor path to root).
//      - This would enable pattern matching against the path: `match x {(pathA, pathB, PathC) => }`. Use generics in last node?

// struct IdentVisitor;
//
// // @see https://docs.rs/syn/1.0.45/syn/visit/index.html
// impl<'ast> Visit<'ast> for IdentVisitor {
//     // fn visit_item_fn(&mut self, node: &'ast ItemFn) {
//     //     println!("Function with name={}", node.sig.ident);
//     //
//     //     // Delegate to the default impl to visit any nested functions.
//     //     visit::visit_item_fn(self, node);
//     // }
//
//     fn visit_ident(&mut self, node: &'ast Ident) {
//         dbg!(node);
//         // println!("Function with name={}", node.sig.ident);
//
//         // Delegate to the default impl to visit any nested functions.
//         // visit::visit_item_fn(self, node);
//     }
//
// }


fn get_cols(ds: &DataStruct, indexes: &Vec<Index>) -> Vec<Col> {
    let indexed_fields = Index::get_fields_used(indexes);

    match &ds.fields {
        Fields::Named(n) => {
            // Remove skipped fields early as their types can be deeply nested and not valid SQL types.
            let not_skipped = n.named.iter().filter(|f| !(get_attrs(&f).iter().any(|i| *i == AttrKey::Skip)));
            not_skipped.map(|f| {
                let mut x: Col = f.into();

                x.is_used_in_index = indexed_fields.contains(&x.name);

                x
            }).collect::<Vec<Col>>()
        }
        Fields::Unnamed(_) |
        Fields::Unit => {
            unreachable!("Only use derive on named structs.")
        }
    }
}

// Converts `#[index("CREATE INDEX idx_example ON self (col_a asc, col_b desc)")]` into `Index`
impl TryFrom<&syn::Attribute> for Index {
    type Error = &'static str;

    fn try_from(a: &syn::Attribute) -> Result<Self, Self::Error> {
        if a.path.segments.first().unwrap().ident == "index" {

            // @todo/low Parse attribute properly:
            // let ts = a.tokens.to_token_stream();
            // let meta = &a.parse_meta().unwrap();

            let attr = a.tokens.to_string();
            let sql = attr.as_str().trim_matches('(').trim_matches(')').trim_matches('"');

            lazy_static! {
                static ref RE: Regex = Regex::new(r"CREATE INDEX (?P<name>\w+) ON self \((?P<cols>[\w,\s]+)\)").unwrap();
            }

            let caps = RE.captures(sql);
            if caps.is_none() {
                panic!("`index` attribute should be a simple normalised 'create index...' string.");
            }

            let c = caps.unwrap();

            let mut fields_used = vec![];

            // E.g. cols = "row_a_id asc, some_fk desc, single, one_more desc"
            for k in c["cols"].split(",").map(|x| x.trim()) {
                let x: Vec<&str> = k.split(" ").collect();
                fields_used.push(x[0].to_string());
            }

            return Ok(Index {
                sql: sql.to_string(),
                name: c["name"].to_string(),
                fields_used,
            });
        }
        Err("Not an 'index' attribute")
    }
}

impl TryFrom<&syn::Attribute> for FK {
    type Error = &'static str;

    fn try_from(a: &syn::Attribute) -> Result<Self, Self::Error> {
        if a.path.segments.first().unwrap().ident == "fk" {

            // @todo/low Parse attribute properly:
            // let ts = a.tokens.to_token_stream();
            // let meta = &a.parse_meta().unwrap();

            let attr = a.tokens.to_string();
            let sql = attr.as_str().trim_matches('(').trim_matches(')').trim_matches('"').trim();

            lazy_static! {
                static ref RE: Regex = Regex::new(r"^FOREIGN KEY\((?P<to>.+?)\) REFERENCES (?P<from_struct>\w+?)\((?P<from>.+?)\)$").unwrap();
            }

            let caps = RE.captures(sql);
            if caps.is_none() {
                panic!("`fk` attribute should match a pattern like `FOREIGN KEY(row_b_id, row_b_title) REFERENCES RowB(row_b_id, row_b_title)`. Note: RowB is a Rust struct.");
            }

            let c = caps.unwrap();
            let get_fields = |k: &str| remove_whitespace(&c[k]).split(",").map(|s| s.to_string()).collect::<Vec<String>>();

            return Ok(FK {
                sql_raw: sql.to_string(),
                from: Endpoint {
                    struct_name: Some(c["from_struct"].into()),
                    fields: get_fields("from").into(),
                },
                to: Endpoint {
                    struct_name: None,
                    fields: get_fields("to").into(),
                },
            });
        }
        Err("Not an 'fk' attribute")
    }
}


fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}


// Converts a `struct Row { col_a, col_b }` into a `Table`.
impl From<&syn::DeriveInput> for Table {
    fn from(ast: &syn::DeriveInput) -> Self {
        let name = &ast.ident;
        match &ast.data {
            Data::Struct(s) => {
                // IdentVisitor.visit_data_struct(s);
                let struct_name = name.to_string();

                // Optionally pluralize name (attribute `#[table_name_plural(false)]` can be used on struct)
                let mut table_name = to_snake_case(name.to_string().as_str());
                if Table::get_table_name_plural(ast) {
                    table_name = format!("{}s", table_name);
                }

                let mut indexes = ast.attrs.iter().filter_map(|x| { Index::try_from(x).ok() }).collect::<Vec<Index>>();

                // Use `self` to denote the struct that the attribute is applied to.
                // - Enforce: struct attribute index should only be used for indexes on that struct/table.
                indexes.iter_mut().for_each(|i| {
                    i.sql = i.sql.replace("ON self", format!("ON {}", table_name.as_str()).as_str());
                });

                let mut foreign_keys = ast.attrs.iter().filter_map(|x| { FK::try_from(x).ok() }).collect::<Vec<FK>>();
                for mut fk in &mut foreign_keys {
                    fk.to.struct_name = Some(struct_name.clone());
                }

                let mut t = Table {
                    struct_name,
                    name: table_name,
                    cols: get_cols(&s, &indexes),
                    indexes,
                    foreign_keys,
                    static_sql_strings: None,
                };

                t.create_static_strings();

                let name_conflicts = t.get_col_names_matching_reserved_sql_keywords();

                // Assumption: SQL writers prefer to quote all or no columns (so never allow them to use a reserved keyword that requires quoting in queries).
                // @see https://stackoverflow.com/questions/10891368/postgres-table-column-name-restrictions#comment14208886_10891404
                assert_eq!(name_conflicts.len(), 0, "Rust struct field names that map to SQL columns conflict with reserved SQL keywords. Change the name so that query writers do not need to quote every identifier defensively. {:?}", name_conflicts);

                t
            }
            _ => panic!("Expected struct.")
        }
    }
}

// Converts an `Enum W { A(X}, B(Y) }` into `vec!["X", "Y"]`
// Issue: Only handles basic cases (E.g. may not handle name resolution of types).
impl From<&DataEnum> for Rows {
    fn from(de: &DataEnum) -> Self {
        let struct_names = de.variants.iter().map(|v| {
            match &v.fields {
                Fields::Unnamed(x) => {
                    match &x.unnamed.first().unwrap().ty {
                        Type::Path(x) => x.path.segments.first().unwrap().ident.to_string(),
                        _ => panic!("Expected Enum Type::Path")
                    }
                }
                _ => panic!("Expected Enum Fields::Unnamed.")
            }
        }).collect();

        Rows {
            struct_names
        }
    }
}


// "SomeCamelCase" => "some_camel_case"
fn to_snake_case(camel_case: &str) -> String {
    let re = Regex::new(r"(?P<a>[a-z])(?P<b>[A-Z])").unwrap();
    re.replace_all(camel_case, "${a}_${b}").to_lowercase()
}