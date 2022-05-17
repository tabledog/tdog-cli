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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, json_key, json_string_or_none, ToDT, ToJSONKey, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct TaxId {
    #[primary_key]
    pub tax_id_id: Option<i64>,

    #[unique]
    pub id: String,

    pub r#type: String,

    pub customer: Option<String>,
    pub country: Option<String>,
    pub value: String,
    pub verification: Option<Value>,
    pub created: DT,
    pub livemode: bool,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,

}

impl GetObjType for TaxId {
    fn get_obj_type_static() -> &'static str {
        "tax_id"
    }
}

impl GetId for TaxId {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::TaxId> for TaxId {
    fn from(x: &API::TaxId) -> Self {
        let x2 = x.clone();

        TaxId {
            tax_id_id: None,
            id: x2.id,
            r#type: x2.type_x.to_json_key(),
            customer: x2.customer.and_then(|x3| x3.get_id_any().into()),
            country: x2.country,
            value: x2.value,
            verification: x.verification.json_or_none(),
            created: x2.created.to_dt(),
            livemode: x.livemode,
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl WriteTree for TaxId {
    type APIType = API::TaxId;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::TaxId) -> Vec<i64> {
        let mut x: TaxId = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::TaxId) -> Vec<i64> {
        let mut x: TaxId = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::TaxId) -> Vec<i64> {
        let mut x: TaxId = data.into();
        let write_id = x.tx_delete_log_write(utx, run_id, "id");
        vec![write_id]
    }
}