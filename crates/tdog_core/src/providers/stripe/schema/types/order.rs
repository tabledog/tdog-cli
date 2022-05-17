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
use crate::providers::stripe::schema::types::order_return::OrderReturn;
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Order {
    #[primary_key]
    pub order_id: Option<i64>,

    #[unique]
    pub id: String,

    pub upstream_id: Option<String>,
    pub charge: Option<String>,
    pub customer: Option<String>,
    pub amount: i64,
    pub amount_returned: Option<i64>,
    pub application: Option<String>,
    pub application_fee: Option<i64>,
    pub currency: String,
    pub email: Option<String>,
    pub external_coupon_code: Option<String>,
    pub items: Value,
    pub returns: Option<Value>,
    pub selected_shipping_method: Option<String>,
    pub shipping: Option<Value>,
    pub shipping_methods: Option<Value>,
    pub status: String,
    pub status_transitions: Option<Value>,
    pub created: DT,
    pub updated: Option<DT>,
    pub livemode: bool,
    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetObjType for Order {
    fn get_obj_type_static() -> &'static str {
        "order"
    }
}

impl GetId for Order {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Order> for Order {
    fn from(x: &API::Order) -> Self {
        let x = x.clone();

        if let Some(x2) = &x.returns {
            assert!(!x2.has_more, "Found Order.returns.has_more=true, this is an issue as there is no URL to list the extra items. Order.id={}", &x.id);
        }

        Order {
            order_id: None,
            id: x.id,

            upstream_id: x.upstream_id,
            charge: x.charge.and_then(|x3| x3.get_id_any().into()),
            customer: x.customer.and_then(|x3| x3.get_id_any().into()),
            amount: x.amount,
            amount_returned: x.amount_returned,
            application: x.application,
            application_fee: x.application_fee,
            currency: x.currency,
            email: x.email,
            external_coupon_code: x.external_coupon_code,

            // Note: `items` have no `id` AND orders can be updated.
            // - Issue: The TD write log and functions expect to be able to ID rows by a unique ID (not array location).
            // - Fix: Use an array. (it is also possible to delete all items and insert all again, but it would not be possible to record the write log for those rows).
            // - Q: is `parent` an inferred unique FK (E.g. one line per parent ID)?
            // - `items` cannot be edited directly via update, but discounts can be appended to the list via `coupon` update.
            items: x.items.json(),
            returns: x.returns.and_then(|x3| x3.data.iter().map(|x4| x4.id.clone()).collect::<Vec<String>>().json().into()),
            selected_shipping_method: x.selected_shipping_method,
            shipping: x.shipping.json_or_none(),
            shipping_methods: x.shipping_methods.json_or_none(),
            status: x.status,
            status_transitions: x.status_transitions.json_or_none(),
            created: x.created.to_dt(),
            updated: x.updated.and_then(|x3| x3.to_dt().into()),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

impl WriteTree for Order {
    type APIType = API::Order;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Order) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Order = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));

        // `order_return` only triggers an `created` event - so it mush be written from the parent order to be kept up to date.
        // - Always included in both dl and events.
        if let Some(x2) = &data.returns {
            for x3 in &x2.data {
                writes.append(&mut OrderReturn::insert_tree(utx, run_id, &x3));
            }
        }


        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Order) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Order = data.into();
        writes.push(x.upsert_first_level(utx, run_id));


        if let Some(x2) = &data.returns {
            for x3 in &x2.data {
                writes.append(&mut OrderReturn::upsert_tree(utx, run_id, &x3));
            }
        }

        writes
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Order) -> Vec<i64> {
        unimplemented!("No delete for Order.")
    }
}