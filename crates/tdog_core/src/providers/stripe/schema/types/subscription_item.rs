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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string_or_none, json_string_or_none_opt, ToDT, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct SubscriptionItem {
    #[primary_key]
    pub subscription_item_id: Option<i64>,

    #[unique]
    pub id: String,

    pub subscription: String,

    pub billing_thresholds: Option<Value>,

    pub price: String,

    pub quantity: Option<i64>,


    pub tax_rates: Option<String>,

    pub created: DT,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetId for SubscriptionItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}


impl From<&API::SubscriptionItem> for SubscriptionItem {
    fn from(s: &API::SubscriptionItem) -> Self {
        // let tax_rates_fk: Option<Vec<String>> = s.tax_rates.as_ref().and_then(|x| x.iter().map(|t| t.id).collect().into());

        SubscriptionItem {
            subscription_item_id: None,
            id: s.id.clone(),
            subscription: s.subscription.clone(),
            billing_thresholds: s.billing_thresholds.json_or_none(),
            price: s.price.id.clone(),
            quantity: s.quantity.clone(),
            tax_rates: s.tax_rates.as_ref().and_then(|x| x.get_pks_json_opt()),
            created: s.created.to_dt(),
            metadata: s.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None
        }
    }
}


impl WriteTree for SubscriptionItem {
    type APIType = API::SubscriptionItem;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::SubscriptionItem) -> Vec<i64> {
        let mut write_ids = vec![];
        let mut x: SubscriptionItem = data.into();

        write_ids.push(x.tx_insert_set_pk_log_write(utx, run_id));

        // When price created via inline `price_data`: `price.created` not triggered, price not in dl list.
        write_ids.append(&mut Price::upsert_tree(utx, run_id, &data.price));

        write_ids
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::SubscriptionItem) -> Vec<i64> {
        let mut write_ids = vec![];
        let mut x: SubscriptionItem = data.into();

        write_ids.push(x.upsert_first_level(utx, run_id));

        // When price created via inline `price_data`: `price.created` not triggered, price not in dl list.
        write_ids.append(&mut Price::upsert_tree(utx, run_id, &data.price));

        write_ids
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::SubscriptionItem) -> Vec<i64> {
        // !has_direct_event - when these are `deleted` via the API they are just detached from the parent sub, but still exist for update/reattachment?
        unimplemented!("Cannot delete SubscriptionItems");
    }
}
