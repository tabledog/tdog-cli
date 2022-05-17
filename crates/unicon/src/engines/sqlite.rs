use std::collections::HashMap;
use rusqlite::{ToSql, Row};
use regex::Regex;
use crate::traits::{TableTr, TableStatic};
use crate::table::CreateSQLObj;


// Any structs that have trait TableTr can also have SQLiteString.
impl<T> SQLiteString for T where T: TableTr {}


/// Alternative: Use serde `Serialize` trait, convert a concrete type to `Value`, and get the field names at runtime that way.
/// - Issue: This will not be type checked.
///
/// Requirements: When converting from the possibly generated API Rust struct into SQL insert, the code must:
/// - Allow subsets of fields to insert.
/// - Allow transforming data (E.g. dates, monetery values to decimal).
/// - Generated fields (e.g JSON strings).
/// - PK and FK storage alongside the row data.
/// - Show db changes on API struct re-gen (e.g auto-generate migrations, determine what will break running code against an older db structure).
/// - Allow for different network APIs to the same data (REST, GRPC, GraphQL etc).
///
/// Because of these needs, I think just converting copying the network API structs, modifying them for SQL insert, and then mapping the data 1:1 could be a better option than placing `Insert` onto the network API struct.
///
/// Note: `SQLiteString: Table`.
/// - Ensure all `EngineString` traits are attached to a single root trait.
/// - Allow storing meta data that can then be converted into engine specific SQL strings (E.g. complex foreign key relations/constraints).
pub trait SQLiteString: TableTr {
    // These are all now readable from `<x as TableTr>::get_table().static_sql_strings.unwrap().sqlite`
    // fn get_create(&self) -> Vec<String> {
    //     let mut x = vec![
    //         Self::get_create_table(self).to_string()
    //     ];
    //
    //     x.append(&mut Self::get_create_indexes(self).iter().map(|x| x.to_string()).collect());
    //
    //     x
    //
    //     // format!(
    //     //     "{};\n{}",
    //     //     // Self::get_create_table(self),
    //     //     Self::get_create_table_with_fk(self),
    //     //     Self::get_create_indexes(self).join(";\n")
    //     // )
    // }


    // These are all now readable from `<x as TableTr>::get_table().static_sql_strings.unwrap().sqlite`
    // fn get_insert(&self) -> &'static str;
    // fn get_create_table(&self) -> &'static str;
    // fn get_create_indexes(&self) -> Vec<&'static str>;


    // Re-add later if needed (probably should be in static_sql_strings or in the database schema with macro time checks that both endpoints exist).
    // fn get_create_table_with_fk(&self) -> String {
    //     let mut create = Self::get_create_table(self).to_string();
    //     let fks = self.get_fk_data();
    //     if fks.len() > 0 {
    //         let statements = fks.into_iter().map(|fk| fk.to_std_sql()).collect::<Vec<String>>().join(",");
    //         let fk_sql = format!(", {})", statements);
    //
    //         let re = Regex::new(r"\)$").unwrap();
    //         create = re.replace(&create, fk_sql.as_str()).to_string();
    //     }
    //
    //     create
    // }


    fn now_ms(&self) -> &'static str {
        "STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')"
    }

    fn get_update(&self, to_update: &Vec<&'static str>, w: &'static str) -> String {
        let mut new_vals: Vec<String> = to_update.iter().map(|i| format!("{}=:{}", i, i)).collect();
        let wh = format!("{}=:{}", w, w);

        if let Some(update_ts) = self.get_key_update_ts() {
            new_vals.push(format!("{}={}", update_ts, self.now_ms()));
        }

        // @todo/low Cache this per unique combo of (rust struct, update values, where).
        format!(
            "UPDATE {} SET {} WHERE {}",
            self.get_table_name(),
            new_vals.join(", "),
            wh
        )
    }

    fn get_delete(&self, w: &'static str) -> String {
        let wh = format!("{}=:{}", w, w);

        // @todo/low Cache this per unique combo of (rust struct, update values, where).
        format!(
            "DELETE FROM {} WHERE {}",
            self.get_table_name(),
            wh
        )
    }
}

// Intermediate state for Rust struct -> X -> SQLite crate API.
// - HashMap not used as the ordering of columns matter for caching SQL statements (and makes debugging easier).
// - Created at macro time, but may be used for updates/deletes at runtime to dynamically generate SQL.
pub struct SQLiteKV<'a> {
    pub key: &'static str,

    // `:${key}`
    // Created at macro time, prevents recreating a String on every operation.
    pub key_param: &'static str,

    pub val: &'a dyn ToSql
}


pub trait SQLiteFuncRusqlite: SQLiteString {
    

    // https://stackoverflow.com/questions/38459139/implementing-nested-traits
    // type T: SQLiteString;

    // All struct fields
    // - Excludes skipped.
    // - Includes pk, insert_ts, update_ts.
    // - Used for getting pk value as part of a `UPDATE WHERE px=x`
    fn to_kv_all(&self) -> Vec<SQLiteKV>;


    // Fields that are writable via insert or update.
    // Excludes (skipped, pk, insert_ts, update_ts)
    fn to_kv_writable_only(&self) -> Vec<SQLiteKV>;

    fn get_ins_and_params(&self) -> (&str, Vec<(&str, &dyn ToSql)>) {
        (
            // Prevent having to write `RowX as ...`
            <Self as TableTr>::get_sqlite_insert(&self),
            Self::to_kv_writable_only(self).into_iter().map(|x| (x.key_param, x.val)).collect()
        )
    }

    fn get_update_and_params(&self, w: &'static str) -> (String, Vec<(&'static str, &dyn ToSql)>) {
        // Note: Col used in `WHERE` clause cannot also be updated (update x set col1=2 where col1=1). Assumption: its either a PK or a unique field, so it will not need updating.
        let mut new_vals = self.to_kv_writable_only();

        // Remove WHERE value.
        new_vals.retain(|x| x.key != w);

        let new_vals_keys: Vec<&'static str> = new_vals.iter().map(|x| x.key).collect();

        // params = (new_values + where)
        let mut params: Vec<(&str, &dyn ToSql)> = new_vals.iter().map(|x| (x.key_param, x.val)).collect();


        // Add WHERE.
        let all = self.to_kv_all();
        let x = all.iter().find(|x| x.key == w).expect(format!("WHERE clause for UPDATE must match a struct field. WHERE={}", w).as_str());
        params.push((x.key_param, x.val));

        let x = <Self as SQLiteString>::get_update(self, &new_vals_keys, w);

        (
            x,
            params
        )
    }


    fn get_delete_and_params(&self, w: &'static str) -> (String, Vec<(&'static str, &dyn ToSql)>) {
        let mut params: Vec<(&str, &dyn ToSql)> = vec![];

        // Add WHERE.
        let all = self.to_kv_all();
        let x = all.iter().find(|x| x.key == w).expect(format!("WHERE clause for DELETE must match a struct field. WHERE={}", w).as_str());
        params.push((x.key_param, x.val));

        (
            <Self as SQLiteString>::get_delete(self, w),
            params
        )
    }
}


pub trait SQLiteFuncRusqliteStatic: TableStatic {
    fn row_to_ins(r: &Row) -> Self;
}
