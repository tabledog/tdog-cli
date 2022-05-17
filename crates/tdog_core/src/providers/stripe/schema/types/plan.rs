use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniDefaultSource, UniPaymentMethod, UniProduct297E1E, UniPromotionCode};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string_or_none, json_string_or_none_opt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Plan {
    #[primary_key]
    pub plan_id: Option<i64>,

    #[unique]
    pub id: String,

    pub product: Option<String>,
    pub active: bool,
    pub aggregate_usage: Option<String>,
    pub amount: Option<i64>,
    pub amount_decimal: Option<String>,
    pub billing_scheme: String,
    pub currency: String,
    pub interval_x: String,
    pub interval_count: i64,
    pub nickname: Option<String>,
    pub tiers: Option<Value>,
    pub tiers_mode: Option<String>,
    pub transform_usage: Option<Value>,
    pub trial_period_days: Option<i64>,
    pub usage_type: String,

    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetId for Plan {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Plan> for Plan {
    fn from(x: &API::Plan) -> Self {
        Plan {
            plan_id: None,
            id: x.id.clone(),
            product: x.product.as_ref().and_then(|x2| {
                if let UniProduct297E1E::String(p_id) = x2 {
                    return p_id.clone().into()
                }
                unreachable!("Plan product must be string")
            }),
            active: x.active,
            aggregate_usage: x.aggregate_usage.to_json_key_or_none(),
            amount: x.amount,
            amount_decimal: x.amount_decimal.clone(),
            billing_scheme: x.billing_scheme.to_json_key(),
            currency: x.currency.clone(),
            interval_x: x.interval.to_json_key(),
            interval_count: x.interval_count,
            nickname: x.nickname.clone(),
            tiers: x.tiers.json_or_none(),
            tiers_mode: x.tiers_mode.to_json_key_or_none(),
            transform_usage: x.transform_usage.json_or_none(),
            trial_period_days: x.trial_period_days.clone(),
            usage_type: x.usage_type.to_json_key(),
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl WriteTree for Plan {
    type APIType = API::Plan;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Plan) -> Vec<i64> {
        let mut x: Plan = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Plan) -> Vec<i64> {
        let mut x: Plan = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Plan) -> Vec<i64> {
        unimplemented!("Cannot delete Plans");
    }
}
