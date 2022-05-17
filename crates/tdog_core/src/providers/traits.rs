use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use mysql::{Params, Value};
use mysql::prelude::Queryable;
//use unicon::{TableStatic, UniTx};
use stripe_client::types::types::GetId;
use unicon::{*};
use unicon::dt::{*};
use unicon::dt3::{*};
use unicon::engines::mysql::{*};
use unicon::engines::postgres::{*};
use unicon::engines::placeholder::{*};
use unicon::engines::sqlite::{*};
use unicon::table::{*};
use unicon::traits::{*};
use unicon::uc::{*};
use unicon::utx::{*};
use unicon_proc_macro::{*};

use crate::providers::stripe::schema_meta::LogWrite;

//use unicon::dt3::DT3;


/// Note: May only work with Stripe as Stripe tables use implicit string `id` column (which is not the primary key, but is used as the ID comes from the network).
pub trait ExistsTx where Self: TableStatic {
    /// @todo/low cache stmt
    fn exists_tx(utx: &mut UniTx, id: &str) -> bool {
        let table = Self::get_table_name_static();
        let std_sql = format!("SELECT 1 FROM {} WHERE id = :id LIMIT 1", table);

        match utx {
            UniTx::Rusqlite(tx) => {
                let mut stmt = tx.prepare_cached(&std_sql).unwrap();
                let mut rows = stmt.query_named(&[(":id", &id)]).unwrap();
                return rows.next().unwrap().is_some()
            }
            UniTx::MySQL(tx) => {
                use mysql::params;
                let params = params! {
                    "id" => &id
                };
                let row: Option<mysql::Row> = tx.exec_first(&std_sql, params).unwrap();
                // @todo/next test this
                return row.is_some();
            }
            UniTx::Postgres(tx) => {
                let sql = format!("SELECT 1 FROM {} WHERE id = $1 LIMIT 1", table);
                return tx.query(sql.as_str(), &[&id]).unwrap().len() == 1
            }
            UniTx::PlaceholderLibA(_) => {}
        }
        unreachable!()
    }
}


pub trait ExistsTxSelf where Self: ExistsTx + GetId {
    fn exists_tx_self(&self, mut utx: &mut UniTx) -> bool {
        Self::exists_tx(&mut utx, self.get_id().as_str())
    }
}


pub trait GetInsertTs where Self: TableStatic {
    fn get_insert_ts(utx: &mut UniTx, id: &str) -> Option<DateTime<Utc>> {
        let table = Self::get_table_name_static();
        let std_sql = format!("SELECT insert_ts FROM {} WHERE id = :id LIMIT 1", table);

        match utx {
            UniTx::Rusqlite(tx) => {
                let mut stmt = tx.prepare_cached(&std_sql).unwrap();
                let mut rows = stmt.query_named(&[(":id", &id)]).unwrap();
                if let Some(row) = rows.next().unwrap() {
                    let s: String = row.get(0).unwrap();
                    // @todo/low does this parse milliseconds?
                    let dt = NaiveDateTime::parse_from_str(s.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();
                    return Some(Utc.from_utc_datetime(&dt));
                }
                return None;
            }
            UniTx::MySQL(tx) => {
                use mysql::params;
                let params = params! {
                    "id" => &id
                };
                let res: Option<mysql::Row> = tx.exec_first(&std_sql, params).unwrap();
                if let Some(row) = res {
                    let x: DT3 = row[0].clone().into();
                    // @todo/next test this
                    return Some(x.into());
                }
                return None;
            }
            UniTx::Postgres(tx) => {
                let sql = format!("SELECT insert_ts FROM {} WHERE id = $1 LIMIT 1", table);
                let dt = tx.query(sql.as_str(), &[&id]).unwrap().first()?.get(0);
                return Some(dt)
            }
            UniTx::PlaceholderLibA(_) => {}
        }
        unreachable!()
    }
}


/// Upsert just the first row/level of this object.
/// - If there are child rows that need to be inserted in other tables they are ignored.
pub trait UpsertFirstLevel where Self: ExistsTxSelf + LogWrite {
    /// Returns write_id (NOT the new/updated pk).
    fn upsert_first_level(&mut self, utx: &mut UniTx, run_id: i64) -> i64 {
        if self.exists_tx_self(utx) {
            self.tx_update_log_write(utx, run_id, "id")
        } else {
            self.tx_insert_set_pk_log_write(utx, run_id)
        }
    }
}


impl<T> ExistsTx for T where T: TableStatic {}

impl<T> GetInsertTs for T where T: TableStatic {}

impl<T> ExistsTxSelf for T where T: ExistsTx + GetId {}

impl<T> UpsertFirstLevel for T where T: ExistsTxSelf + LogWrite {}
