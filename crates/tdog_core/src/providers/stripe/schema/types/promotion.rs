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
pub struct PromotionCode {
    #[primary_key]
    pub promotion_id: Option<i64>,

    #[unique]
    pub id: String,

    pub customer: Option<String>,
    pub active: bool,
    pub code: String,
    pub coupon: String,
    pub expires_at: Option<DT>,
    pub max_redemptions: Option<i64>,
    pub restrictions: Value,
    pub times_redeemed: i64,
    pub created: DT,
    pub livemode: bool,
    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetId for PromotionCode {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}
impl GetObjType for PromotionCode {
    fn get_obj_type_static() -> &'static str {
        "promotion_code"
    }
}


impl From<&API::PromotionCode> for PromotionCode {
    fn from(x: &API::PromotionCode) -> Self {
        let x2 = x.clone();

        PromotionCode {
            promotion_id: None,
            id: x2.id,
            customer: x2.customer.and_then(|x3| match x3 {
                UniCustomerC00F6E::String(s) => s.into(),
                _ => unreachable!("Expected customer to be string on promotion code.")
            }),
            active: x2.active,
            code: x2.code,
            coupon: x2.coupon.id,
            expires_at: x.expires_at.and_then(|x3| x3.to_dt().into()),
            max_redemptions: x.max_redemptions,
            restrictions: x.restrictions.json(),
            times_redeemed: x.times_redeemed,
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None
        }
    }
}

impl WriteTree for PromotionCode {
    type APIType = API::PromotionCode;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::PromotionCode) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: PromotionCode = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));

        // Coupons = has_direct_list && has_direct_event, no need to upsert here.

        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::PromotionCode) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: PromotionCode = data.into();
        writes.push(x.upsert_first_level(utx, run_id));

        writes
    }


    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::PromotionCode) -> Vec<i64> {
        unimplemented!("No delete for PromotionCode.")
    }
}