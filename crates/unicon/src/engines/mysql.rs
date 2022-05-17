use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash;
use mysql::Params;
use crate::traits::{TableTr, TableStatic};
use crate::table::CreateSQLObj;


// Any structs that have trait TableTr can also have MySQLString.
impl<T> MySQLString for T where T: TableTr {}

pub trait MySQLString: TableTr {

    // These are all now readable from `<x as TableTr>::get_table().static_sql_strings.unwrap().mysql`
    // fn get_insert(&self) -> &'static str;
    // fn get_create_table(&self) -> &'static str;
    // fn get_create_indexes(&self) -> Vec<&'static str>;

    // fn get_create(&self) -> Vec<String> {
    //     let mut x = vec![];
    //     x.push(Self::get_create_table(self).to_string());
    //
    //     for x2 in Self::get_create_indexes(self) {
    //         x.push(x2.to_string());
    //     }
    //
    //     // Self::get_create_table_with_fk(self), // Not supporting FK's as API relations are typically "eventually consistent".
    //
    //     x
    // }

    fn now_ms(&self) -> &'static str {
        // Assumption: A `DATETIME` col will truncate this to the correct granularity.
        "UTC_TIMESTAMP(6)"
    }

    fn get_update(&self, to_update: &Vec<String>, w: &'static str) -> String {
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


#[derive(Debug, Clone)]
pub struct MySQLKV {
    pub key: String,
    pub val: mysql::Value
}

impl MySQLKV {
    // For use with MySQL crate API, specifically `Params::Named(x)`
    pub fn into_hm(kv: Vec<MySQLKV>) -> HashMap<String, mysql::Value, BuildHasherDefault<XxHash>> {
        let mut hm = HashMap::default();
        for x in kv {
            hm.insert(x.key, x.val);
        }
        hm
    }
}



pub trait MySQLFuncX: MySQLString {

    // Note: `Vec` used here to preserve order (as ordered in the Rust struct source code).
    // - This allows consistent SQL generation (caching, debugging).
    fn to_kv_all(&self) -> Vec<MySQLKV>;
    fn to_kv_writable_only(&self) -> Vec<MySQLKV>;


    fn get_ins_and_params(&self) -> (&str, Params) {
        (
            // Prevent having to write `RowX as ...`
            <Self as TableTr>::get_mysql_insert(self),
            Params::Named(MySQLKV::into_hm(Self::to_kv_writable_only(self)))
        )
    }


    fn get_update_and_params(&self, w: &'static str) -> (String, Params) {
        // Note: Col used in `WHERE` clause cannot also be updated (update x set col1=2 where col1=1). Assumption: its either a PK or a unique field, so it will not need updating.
        let mut new_vals = self.to_kv_writable_only();
        new_vals.retain(|x| x.key != w);

        let new_vals_keys: Vec<String> = new_vals.iter().map(|x| x.key.clone()).collect();

        // params = (new_values + where)
        let mut params = MySQLKV::into_hm(new_vals.clone());


        // Add WHERE.
        let mut all = self.to_kv_all();
        let x = all.into_iter().find(|x| x.key == w).expect(format!("WHERE clause for UPDATE must match a struct field. WHERE={}", w).as_str());
        params.insert(x.key, x.val);

        (
            <Self as MySQLString>::get_update(self, &new_vals_keys, w),
            Params::Named(params)
        )
    }


    fn get_delete_and_params(&self, w: &'static str) -> (String, Params) {
        let mut params: HashMap<String, mysql::Value, _> = HashMap::default();

        // Add WHERE.
        let mut all = self.to_kv_all();
        let x = all.into_iter().find(|x| x.key == w).expect(format!("WHERE clause for DELETE must match a struct field. WHERE={}", w).as_str());
        params.insert(x.key, x.val);

        (
            <Self as MySQLString>::get_delete(self, w),
            Params::Named(params)
        )
    }
}


pub trait MySQLFuncXStatic: TableStatic {
    // Note: `mut` needed to use `Row::take(col_name)` which partially moves some of its data.
    fn row_to_ins(r: &mut mysql::Row) -> Self;
}





