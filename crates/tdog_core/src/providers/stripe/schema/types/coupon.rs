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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdFromEnumOrNone, json_key, json_string_or_none, PickOpt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Coupon {
    #[primary_key]
    pub coupon_id: Option<i64>,

    #[unique]
    pub id: String,

    pub valid: bool,
    pub name: Option<String>,
    pub amount_off: Option<i64>,
    pub applies_to: Option<Value>,
    pub currency: Option<String>,
    pub duration: String,
    pub duration_in_months: Option<i64>,
    pub max_redemptions: Option<i64>,
    pub percent_off: Option<f64>,
    pub redeem_by: Option<i64>,
    pub times_redeemed: i64,

    pub livemode: bool,
    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetId for Coupon {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Coupon> for Coupon {
    fn from(x: &API::Coupon) -> Self {
        Coupon {
            coupon_id: None,
            id: x.id.clone(),
            valid: x.valid,
            name: x.name.clone(),
            amount_off: x.amount_off.clone(),
            applies_to: x.applies_to.json_or_none(),
            currency: x.currency.clone(),
            duration: x.duration.to_json_key(),
            duration_in_months: x.duration_in_months.clone(),
            max_redemptions: x.max_redemptions.clone(),
            percent_off: x.percent_off.clone(),
            redeem_by: x.redeem_by.clone(),
            times_redeemed: x.times_redeemed,
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

impl WriteTree for Coupon {
    type APIType = API::Coupon;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Coupon) -> Vec<i64> {
        let mut x: Coupon = data.into();

        // May be inserted via a parent object before the download; avoid unique constraint error on id.
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Coupon) -> Vec<i64> {
        let mut x: Coupon = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Coupon) -> Vec<i64> {
        unimplemented!("No delete for Coupon as it needs to be referenced from older invoices (via a discount object).")
    }
}