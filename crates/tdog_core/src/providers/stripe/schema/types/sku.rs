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
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Sku {
    #[primary_key]
    pub sku_id: Option<i64>,

    #[unique]
    pub id: String,

    pub product: String,
    pub active: bool,
    pub attributes: Option<Value>,
    pub currency: String,
    pub image: Option<String>,
    pub inventory: Value,
    pub package_dimensions: Option<Value>,
    pub price: i64,
    pub created: DT,
    pub updated: DT,
    pub livemode: bool,
    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetObjType for Sku {
    fn get_obj_type_static() -> &'static str {
        "sku"
    }
}

impl GetId for Sku {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Sku> for Sku {
    fn from(x: &API::Sku) -> Self {
        let x = x.clone();

        Sku {
            sku_id: None,
            id: x.id,
            product: x.product.get_id_any(),
            active: x.active,
            attributes: x.attributes.json_or_none(),
            currency: x.currency,
            image: x.image,
            inventory: x.inventory.json(),
            package_dimensions: x.package_dimensions.json_or_none(),
            price: x.price,
            created: x.created.to_dt(),
            updated: x.updated.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

impl WriteTree for Sku {
    type APIType = API::Sku;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Sku) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Sku = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));

        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Sku) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Sku = data.into();
        writes.push(x.upsert_first_level(utx, run_id));


        writes
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Sku) -> Vec<i64> {
        let mut x: Sku = data.into();
        let write_id = x.tx_delete_log_write(utx, run_id, "id");
        vec![write_id]
    }
}