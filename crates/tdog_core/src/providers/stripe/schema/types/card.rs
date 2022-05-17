use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, json_key, json_string_or_none, json_string_or_none_opt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Card {
    #[primary_key]
    pub card_id: Option<i64>,

    #[unique]
    pub id: String,

    pub name: Option<String>,
    // Connect
    pub account: Option<String>,
    pub customer: Option<String>,
    pub recipient: Option<String>,
    pub address_city: Option<String>,
    pub address_country: Option<String>,
    pub address_line1: Option<String>,
    pub address_line1_check: Option<String>,
    pub address_line2: Option<String>,
    pub address_state: Option<String>,
    pub address_zip: Option<String>,
    pub address_zip_check: Option<String>,
    pub available_payout_methods: Option<Value>,
    pub brand: String,
    pub country: Option<String>,
    pub currency: Option<String>,
    pub cvc_check: Option<String>,
    pub default_for_currency: Option<bool>,
    pub dynamic_last4: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    pub fingerprint: Option<String>,
    pub funding: String,
    pub last4: String,
    pub tokenization_method: Option<String>,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetObjType for Card {
    fn get_obj_type_static() -> &'static str {
        "card"
    }
}

impl GetId for Card {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Card> for Card {
    fn from(x: &API::Card) -> Self {
        let x = x.clone();

        Card {
            card_id: None,
            id: x.id.clone(),

            name: x.name,

            account: x.account.and_then(|x2| x2.get_id_any().into()),
            customer: x.customer.and_then(|x2| x2.get_id_any().into()),
            recipient: x.recipient.and_then(|x2| x2.get_id_any().into()),

            address_city: x.address_city,
            address_country: x.address_country,
            address_line1: x.address_line1,
            address_line1_check: x.address_line1_check,
            address_line2: x.address_line2,
            address_state: x.address_state,
            address_zip: x.address_zip,
            address_zip_check: x.address_zip_check,
            available_payout_methods: x.available_payout_methods.and_then(|x2| x2.iter().map(|x3| x3.to_json_key()).collect::<Vec<String>>().json().into()),
            brand: x.brand,
            country: x.country,
            currency: x.currency,
            cvc_check: x.cvc_check,
            default_for_currency: x.default_for_currency,
            dynamic_last4: x.dynamic_last4,
            exp_month: x.exp_month,
            exp_year: x.exp_year,
            fingerprint: x.fingerprint,
            funding: x.funding,
            last4: x.last4,
            tokenization_method: x.tokenization_method,

            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl WriteTree for Card {
    type APIType = API::Card;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Card) -> Vec<i64> {
        let mut x: Card = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Card) -> Vec<i64> {
        let mut x: Card = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Card) -> Vec<i64> {
        // `customer.source.deleted` just means it is no longer attached to a customer,
        // `customer.update` removes the card_x ID from the customer.
        // card.customer is still set to the old customer (should be ok as queries can start from customer.default_source, cards cannot be moved between customers meaning card.customer never transitions between different states).
        unimplemented!("Cannot delete Cards");
    }
}
