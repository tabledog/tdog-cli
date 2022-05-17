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
pub struct Product {
    #[primary_key]
    pub product_id: Option<i64>,

    #[unique]
    pub id: String,

    // @todo/low/maybe This is missing from the spec, add it for legacy users?
    pub r#type: Option<String>,

    pub name: String,
    pub active: bool,


    pub attributes: Option<Value>,
    pub caption: Option<String>,


    pub deactivate_on: Option<Value>,
    pub description: Option<String>,


    pub images: Option<Value>,


    pub package_dimensions: Option<Value>,
    pub shippable: Option<bool>,
    pub statement_descriptor: Option<String>,
    pub unit_label: Option<String>,
    pub url: Option<String>,
    pub updated: DT,
    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetId for Product {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Product> for Product {
    fn from(i: &API::Product) -> Self {
        Product {
            product_id: None,
            id: i.id.clone(),
            // Note: this is missing from the spec and the notes because it is deprecated, replaced with Prices.
            r#type: None,
            name: i.name.clone(),
            active: i.active,
            attributes: i.attributes.json_or_none(),
            caption: i.caption.clone(),
            deactivate_on: i.deactivate_on.json_or_none(),
            description: i.description.clone(),
            images: i.images.json_or_none(),
            package_dimensions: i.package_dimensions.json_or_none(),
            shippable: i.shippable.clone(),
            statement_descriptor: i.statement_descriptor.clone(),
            unit_label: i.unit_label.clone(),
            url: i.url.clone(),
            updated: i.updated.to_dt(),
            created: i.created.to_dt(),
            livemode: i.livemode,
            metadata: i.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None
        }
    }
}


impl WriteTree for Product {
    type APIType = API::Product;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Product) -> Vec<i64> {
        let mut x: Product = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Product) -> Vec<i64> {
        let mut x: Product = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Product) -> Vec<i64> {
        let mut x: Product = data.into();
        vec![x.tx_delete_log_write(utx, run_id, "id")]
    }
}
