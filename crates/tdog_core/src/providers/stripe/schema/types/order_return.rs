use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCharge, UniCustomerC00F6E, UniDefaultSource, UniPaymentIntent, UniPaymentMethod, UniPromotionCode};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, GetIdFromEnumOrNone, json_key, json_string_or_none, PickOpt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct OrderReturn {
    #[primary_key]
    pub order_return_id: Option<i64>,

    #[unique]
    pub id: String,

    // Note: avoid using `order` as it is reserved, see https://www.postgresql.org/docs/8.1/sql-keywords-appendix.html
    pub order_id: Option<String>,

    // Note: This cannot be kept up to date with just events.
    // - Only order_return.created triggered (no order_return.updated triggered after a refund completed).
    // - Fix: `order.updated` is triggered after the refund completes, refunds is always expanded in both dl and events.
    pub refund: Option<String>,


    pub amount: i64,
    pub currency: String,

    // Note: these do not have their own `id`, so are JSON arrays of full objects.
    pub items: Value,
    pub created: DT,
    pub livemode: bool,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    // Only a create event = no update?
    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetObjType for OrderReturn {
    fn get_obj_type_static() -> &'static str {
        "order_return"
    }
}

impl GetId for OrderReturn {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::OrderReturn> for OrderReturn {
    fn from(x: &API::OrderReturn) -> Self {
        let x2 = x.clone();

        OrderReturn {
            order_return_id: None,
            id: x2.id,
            order_id: x2.order.and_then(|x| x.get_id_any().into()),
            refund: x2.refund.and_then(|x| x.get_id_any().into()),
            amount: x2.amount,
            currency: x2.currency,
            items: x2.items.json(),
            created: x2.created.to_dt(),
            livemode: x2.livemode,
            insert_ts: None,
            update_ts: None,
        }
    }
}

impl WriteTree for OrderReturn {
    type APIType = API::OrderReturn;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::OrderReturn) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: OrderReturn = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));

        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::OrderReturn) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: OrderReturn = data.into();
        writes.push(x.upsert_first_level(utx, run_id));


        writes
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::OrderReturn) -> Vec<i64> {
        unimplemented!("No delete for OrderReturn.")
    }
}