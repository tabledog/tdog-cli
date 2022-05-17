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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string_or_none, json_string_or_none_opt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Price {
    #[primary_key]
    pub price_id: Option<i64>,

    #[unique]
    pub id: String,
    pub r#type: String,

    pub product: String,
    pub active: bool,
    pub billing_scheme: String,
    pub currency: String,
    pub lookup_key: Option<String>,
    pub nickname: Option<String>,

    pub recurring: Option<Value>,

    // pub tiers: Option<Vec<PriceTier>>,

    pub tiers: Option<Value>,

    pub tiers_mode: Option<String>,
    pub transform_quantity: Option<Value>,

    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,

    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetId for Price {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Price> for Price {
    fn from(x: &API::Price) -> Self {
        Price {
            price_id: None,
            id: x.id.clone(),
            r#type: x.type_x.to_json_key(),
            product: if let API::UniProduct297E1E::String(s) = &x.product {
                s.clone()
            } else {
                unreachable!("product should be a string")
            },
            active: x.active,
            billing_scheme: x.billing_scheme.to_json_key(),
            currency: x.currency.clone(),
            lookup_key: x.lookup_key.clone(),
            nickname: x.nickname.clone(),
            recurring: x.recurring.json_or_none(),
            tiers: x.tiers.json_or_none(),
            tiers_mode: x.tiers_mode.to_json_key_or_none(),
            transform_quantity: x.transform_quantity.json_or_none(),
            unit_amount: x.unit_amount.clone(),
            unit_amount_decimal: x.unit_amount_decimal.clone(),
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

/// @todo/low Should the price table have a `inline=bool` for prices created via `price_data`?
/// - These act differently from normal prices (cannot dl, no events), cannot re-use?
impl WriteTree for Price {
    type APIType = API::Price;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Price) -> Vec<i64> {
        let mut x: Price = data.into();
        // Inline prices (created via `price_data`) exist on many objects, and are not in the dl list, and do not have events.
        // - Upsert instead of insert so that other inline-price parent objects can be processed before this when downloading.
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Price) -> Vec<i64> {
        let mut x: Price = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Price) -> Vec<i64> {
        unimplemented!("Cannot delete prices");
    }
}
