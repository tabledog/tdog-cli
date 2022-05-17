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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string_or_none, ToDT, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct TaxRate {
    #[primary_key]
    pub tax_rate_id: Option<i64>,

    #[unique]
    pub id: String,
    pub active: bool,
    pub description: Option<String>,
    pub display_name: String,
    pub inclusive: bool,
    pub jurisdiction: Option<String>,
    pub percentage: f64,

    pub created: DT,
    pub livemode: bool,

    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetObjType for TaxRate {
    fn get_obj_type_static() -> &'static str {
        "tax_rate"
    }
}

impl GetId for TaxRate {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::TaxRate> for TaxRate {
    fn from(i: &API::TaxRate) -> Self {
        TaxRate {
            tax_rate_id: None,
            id: i.id.clone(),
            active: i.active,
            description: i.description.clone(),
            display_name: i.display_name.clone(),
            inclusive: i.inclusive,
            jurisdiction: i.jurisdiction.clone(),
            percentage: i.percentage.clone(),
            created: i.created.to_dt(),
            livemode: i.livemode,
            metadata: i.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl WriteTree for TaxRate {
    type APIType = API::TaxRate;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::TaxRate) -> Vec<i64> {
        let mut x: TaxRate = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::TaxRate) -> Vec<i64> {
        let mut x: TaxRate = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::TaxRate) -> Vec<i64> {
        // Cannot delete TaxRate.
        unimplemented!()
    }
}