use std::collections::HashMap;
//use unicon::dt3::{DT3, from_mysql_to_dt_3_utc_opt, from_mysql_to_dt_3_utc};
use std::hash::BuildHasherDefault;

use chrono::{DateTime, Utc};
use mysql::Params;
use mysql::prelude::Queryable;

use serde::{Deserialize, Deserializer, Serialize};
//use unicon::dt::DT;
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniDefaultSource, UniPaymentMethod, UniPromotionCode};
use stripe_client::types::types as API;
use twox_hash::XxHash;
use unicon::{*};
use unicon::dt::{*};
//use unicon::UniTx;
//use unicon::{*};
use unicon::dt::{*};
use unicon::dt3::{*};
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

use crate::fns::{get_utc_dt, get_utc_dt_from_3ms};
use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAndObject, json_key, json_string_or_none, ToDT, ToVal, ToValOrNone, unix_to_iso, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
#[index("CREATE INDEX data_object_id ON self (data_object_id)")]
#[index("CREATE INDEX data_object_object ON self (data_object_object)")]
// @todo/low Index `type` as its likely to be a target for queries. Issue: `macro-insert` uses reserved Rust keyword.
// #[index("CREATE INDEX type ON self (type)")]
#[index("CREATE INDEX resource ON self (resource)")]
#[index("CREATE INDEX action ON self (action)")]
pub struct NotificationEvent {
    #[primary_key]
    pub event_id: Option<i64>,

    #[unique]
    pub id: String,

    pub r#type: String,

    pub resource: String,
    pub action: String,

    pub account: Option<String>,
    pub api_version: Option<String>,

    // pub data: NotificationEventData,
    pub data_object_id: Option<String>,
    pub data_object_object: String,


    pub data_object: Value,


    pub data_previous_attributes: Option<Value>,

    pub pending_webhooks: i64,

    // pub request: Option<NotificationEventRequest>,
    pub request_id: Option<String>,
    pub request_idempotency_key: Option<String>,

    pub created: DT,
    pub livemode: bool,

    #[insert_ts]
    pub insert_ts: Option<DT3>,
}




impl NotificationEvent {
    // Returns the download ts to determine which events to ignore (as they were before the download, meaning any writes are included in the downloaded object).
    // Assumption: This is only called when on the first `apply_events` after the first `download`.
    // - This means that all rows in `td_stripe_writes` are from a `download` run.
    // pub fn first_events_join_first_download(utx: &mut UniTx) -> Vec<(String, String, DateTime<Utc>, Option<String>, Option<DateTime<Utc>>)> {
    //     // Note: Remove `plan` as these are just alias's to prices.
    //     // - These are almost identical to `price`, and have a `price_xyz` ID.
    //     // - Plans are being replaced by the prices API.
    //     // - Assumption: All plan events are mirrored/copied to price events (but some price events are never mirrored to plans).
    //     //      - Assumption: processing only the price events and ignoring plan events will also capture any plan data.
    //
    //     // Join on (object_type, obj_id) because some two types can have the same id (src_x can be the ID of a Source and a PaymentMethod).
    //
    //     // language=sql
    //     let std_sql = r###"
    //         select e.id, e.data_object_id, e.created, w.obj_type, w.insert_ts
    //         from notification_events e left join td_stripe_writes w on
    //             (e.data_object_object = w.obj_type AND e.data_object_id = w.obj_id)
    //         where
    //             e.data_object_id IS NOT NULL AND
    //             e.data_object_object != "plan"
    //     "###;
    //
    //     match utx {
    //         UniTx::Rusqlite(tx) => {
    //             let mut stmt = tx.prepare(std_sql).unwrap();
    //             let mut rows = stmt.query_map(NO_PARAMS, |r| {
    //                 let insert_ts_str: Option<String> = r.get(4).unwrap();
    //                 let mut insert_ts = None;
    //                 if let Some(s) = insert_ts_str {
    //                     insert_ts = Some(get_utc_dt_from_3ms(&s));
    //                 }
    //
    //                 Ok((
    //                     r.get(0).unwrap(),
    //                     r.get(1).unwrap(),
    //                     get_utc_dt(&r.get(2).unwrap()),
    //                     r.get(3).unwrap(),
    //                     insert_ts
    //                 ))
    //             }).unwrap();
    //
    //             let mut set = vec![];
    //             for r in rows {
    //                 set.push(r.unwrap());
    //             };
    //             return set;
    //         },
    //         UniTx::MySQL(tx) => {
    //             // tx.query(&std_sql).unwrap()
    //             //
    //             //
    //             // if let Some(row) = res {
    //             //     Ok((
    //             //       row[0].into(),
    //             //       row[1].into(),
    //             //       from_mysql_to_dt_3_utc(row[2]),
    //             //       row[3].into(),
    //             //       from_mysql_to_dt_3_utc_opt(row[4])
    //             //     ))
    //             // }
    //             // return None;
    //         }
    //         UniTx::PlaceholderLibA(_) => unimplemented!()
    //     }
    // }
}


impl GetId for NotificationEvent {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::NotificationEvent> for NotificationEvent {
    fn from(i: &API::NotificationEvent) -> Self {
        let type_parts = i.type_x.split(".").collect::<Vec<_>>();

        let resource = type_parts[0..type_parts.len() - 1].join(".");
        let action = type_parts.last().unwrap().to_string();

        let x = (*i.data.object).get_id_and_object_type_opt();
        let data_object_id = x.0;
        let data_object_object = x.1.unwrap();

        NotificationEvent {
            event_id: None,
            id: i.id.clone(),
            r#type: i.type_x.clone(),
            resource,
            action,
            account: i.account.clone(),
            api_version: i.api_version.clone(),
            data_object_id,
            data_object_object,
            data_object: (*i.data.object).json(),
            data_previous_attributes: i.data.previous_attributes.json_or_none(),
            pending_webhooks: i.pending_webhooks,
            request_id: i.request.as_ref().and_then(|x| x.id.clone()),
            request_idempotency_key: i.request.as_ref().and_then(|x| x.idempotency_key.clone()),
            created: i.created.to_dt(),
            livemode: i.livemode,
            insert_ts: None,
        }
    }
}
