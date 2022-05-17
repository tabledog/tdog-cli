use std::collections::HashMap;

use crate::traits::{TableTr, TableStatic};
use crate::table::{CreateSQLObj, Postgres, SQLStaticStrings};
use postgres::types::ToSql;
use postgres::Row;


// Any structs that have trait TableTr can also have PostgresString.
impl<T> PostgresString for T where T: TableTr {}

pub trait PostgresString: TableTr {
    fn now_ms(&self) -> &'static str {
        // Returns second to 5 decimal places.
        // "TIMEZONE('utc', now())"
        Postgres::get_utc_now_3()
    }

    fn get_update(&self, to_update: &Vec<&'static str>, w: &'static str) -> String {
        let mut next_param = 1;
        let mut new_vals: Vec<String> = vec![];

        for x in to_update {
            new_vals.push(format!("{} = ${}", x, next_param));
            next_param += 1;
        }

        if let Some(update_ts) = self.get_key_update_ts() {
            new_vals.push(format!("{}={}", update_ts, self.now_ms()));
        }

        // Add where last (param value will be last).
        let wh = format!("{} = ${}", w, next_param);
        next_param += 1;

        // @todo/low Cache this per unique combo of (rust struct, update values, where).
        format!(
            "UPDATE {} SET {} WHERE {}",
            self.get_table_name(),
            new_vals.join(", "),
            wh
        )
    }

    fn get_delete(&self, w: &'static str) -> String {
        let wh = format!("{} = $1", w);

        // @todo/low Cache this per unique combo of (rust struct, update values, where).
        format!(
            "DELETE FROM {} WHERE {}",
            self.get_table_name(),
            wh
        )
    }
}


pub trait PostgresFuncX: PostgresString {
    // Avoid HashMap:
    // - `.iter` order is random. This results in SQL statements with random col/param names.
    //      - This will effect debug-ability, SQL diff tools, and query caching.
    // - Postgres does not natively support key based params, only 1-based indexes.
    // - Vec order is the same as the Rust struct order (created from the macro).
    fn to_kv_all(&self) -> Vec<(&'static str, &(dyn ToSql + Sync))>;
    fn to_kv_writable_only(&self) -> Vec<(&'static str, &(dyn ToSql + Sync))>;

    fn get_ins_and_params(&self) -> (&str, Vec<(&str, &(dyn ToSql + Sync))>) {
        (
            // Prevent having to write `RowX as ...`
            <Self as TableTr>::get_postgres_insert(&self),
            Self::to_kv_writable_only(self)
        )
    }

    // Remove column key from params
    // - rust-postgres crate (and the pg protocol) do not support key based params, only 1-indexed params.
    fn get_ins_and_params_vec(&self) -> (&str, Vec<&(dyn ToSql + Sync)>) {
        let (ins, params_kv) = self.get_ins_and_params();
        let params_vec = params_kv.into_iter().map(|x| x.1).collect();
        (ins, params_vec)
    }


    fn get_update_and_params(&self, w: &'static str) -> (String, Vec<(&'static str, &(dyn ToSql + Sync))>) {
        // Note: Col used in `WHERE` clause cannot also be updated (update x set col1=2 where col1=1). Assumption: its either a PK or a unique field, so it will not need updating.
        let mut new_vals = self.to_kv_writable_only();

        // Remove `where` col from update vals.
        new_vals.retain(|x| x.0 != w);

        let new_vals_keys: Vec<&'static str> = new_vals.iter().map(|x| x.0).collect();

        // params = (new_values + where)
        let mut params: Vec<(&str, &(dyn ToSql + Sync))> = new_vals.clone();

        // Add where.
        let all = self.to_kv_all();
        let w_x = all.into_iter().find(|x| x.0 == w);
        assert!(w_x.is_some(), "WHERE clause for UPDATE must match a struct field. WHERE={}", w);
        params.push(w_x.unwrap());

        (
            <Self as PostgresString>::get_update(self, &new_vals_keys, w),
            params
        )
    }

    // Params indexed via vec (not hashmap key)
    fn get_update_and_params_vec(&self, w: &'static str) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let (update, params_kv) = self.get_update_and_params(w);
        let params_vec = params_kv.into_iter().map(|x| x.1).collect();
        (update, params_vec)
    }


    fn get_delete_and_params(&self, w: &'static str) -> (String, Vec<(&'static str, &(dyn ToSql + Sync))>) {
        let mut params: Vec<(&str, &(dyn ToSql + Sync))> = vec![];
        let all = self.to_kv_all();
        let w_x = all.into_iter().find(|x| x.0 == w);
        assert!(w_x.is_some(), "WHERE clause for DELETE must match a struct field. WHERE={}", w);
        params.push(w_x.unwrap());

        (
            <Self as PostgresString>::get_delete(self, w),
            params
        )
    }

    // Params indexed via vec (not hashmap key)
    fn get_delete_and_params_vec(&self, w: &'static str) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let (delete, params_kv) = self.get_delete_and_params(w);
        let params_vec = params_kv.into_iter().map(|x| x.1).collect();
        (delete, params_vec)
    }
}


pub trait PostgresFuncXStatic: TableStatic {
    fn row_to_ins(r: &Row) -> Self;
}






