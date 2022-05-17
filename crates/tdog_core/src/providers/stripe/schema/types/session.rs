use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use rusqlite::params;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCustomerC00F6E, UniDefaultSource, UniPaymentMethod, UniPromotionCode};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, json_key, json_string_or_none, Source, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema_meta::{LogWrite, TdStripeWrite};
use crate::providers::traits::{ExistsTx, ExistsTxSelf, UpsertFirstLevel};

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Session {
    #[primary_key]
    pub session_id: Option<i64>,

    #[unique]
    pub id: String,

    pub client_reference_id: Option<String>,

    pub customer: Option<String>,
    pub payment_intent: Option<String>,
    pub setup_intent: Option<String>,
    pub subscription: Option<String>,
    pub allow_promotion_codes: Option<bool>,
    pub amount_subtotal: Option<i64>,
    pub amount_total: Option<i64>,
    pub billing_address_collection: Option<String>,
    pub cancel_url: String,
    pub currency: Option<String>,
    pub customer_email: Option<String>,


    // Issues:
    // - A. No events for `session` create, so events cannot keep them up to date.
    // - B. `line_items` JSON key is never included in downloads OR events (and there are no events for the singular items).
    // - C. Cannot complete a session without a web UI (no API).

    // Temp fix:
    // - Ignore line items as they are converted to invoice/sub items on success.
    // - Only keep the parent first level `session` keys up to date. Events:
    //      - `checkout.session.async_payment_failed`
    //      - `checkout.session.async_payment_succeeded`
    //      - `checkout.session.completed`
    // pub line_items: Option<String>,


    pub locale: Option<String>,
    pub mode: String,
    pub payment_method_types: Value,
    pub payment_status: String,
    pub shipping: Option<Value>,
    pub shipping_address_collection: Option<Value>,
    pub submit_type: Option<String>,
    pub success_url: String,
    pub total_details: Option<Value>,

    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetObjType for Session {
    fn get_obj_type_static() -> &'static str {
        "checkout.session"
    }
}


impl GetId for Session {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Session> for Session {
    fn from(x: &API::Session) -> Self {
        let x2 = x.clone();

        if let Some(x3) = &x.line_items {
            // Note: this key seems to never be set.
            assert!(!x3.has_more, "Session.has_more=true but there is no URL to fetch the missing items. Session.id={}", &x2.id);
        }

        Session {
            session_id: None,
            id: x2.id,
            client_reference_id: x2.client_reference_id,
            customer: x2.customer.and_then(|x3| x3.get_id_any().into()),
            payment_intent: x2.payment_intent.and_then(|x3| x3.get_id_any().into()),
            setup_intent: x2.setup_intent.and_then(|x3| x3.get_id_any().into()),
            subscription: x2.subscription.and_then(|x3| x3.get_id_any().into()),
            allow_promotion_codes: x2.allow_promotion_codes,
            amount_subtotal: x.amount_subtotal,
            amount_total: x.amount_total,
            billing_address_collection: x2.billing_address_collection.to_json_key_or_none(),
            cancel_url: x2.cancel_url,
            currency: x2.currency,
            customer_email: x2.customer_email,
            // line_items: x2.line_items.and_then(|x3| x3.data.json().into()),
            locale: x2.locale.to_json_key_or_none(),
            mode: x2.mode.to_json_key(),
            payment_method_types: x2.payment_method_types.json(),
            payment_status: x2.payment_status.to_json_key(),
            shipping: x2.shipping.json_or_none(),
            shipping_address_collection: x2.shipping_address_collection.json_or_none(),
            submit_type: x2.submit_type.to_json_key_or_none(),
            success_url: x2.success_url,
            total_details: x2.total_details.json_or_none(),
            livemode: x2.livemode,
            metadata: x2.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl WriteTree for Session {
    type APIType = API::Session;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Session) -> Vec<i64> {
        let mut write_ids = vec![];
        let mut x: Session = data.into();

        write_ids.push(x.tx_insert_set_pk_log_write(utx, run_id));

        write_ids
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Session) -> Vec<i64> {
        let mut write_ids = vec![];
        let mut x: Session = data.into();

        write_ids.push(x.upsert_first_level(utx, run_id));


        write_ids
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Session) -> Vec<i64> {
        unimplemented!("Cannot delete Stripe Session.")
    }
}

