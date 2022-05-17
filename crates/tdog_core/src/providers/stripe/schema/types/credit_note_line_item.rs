use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use rusqlite::params;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCustomerC00F6E, UniDefaultSource, UniPaymentMethod, UniPromotionCode};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string_or_none, Source, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema_meta::{LogWrite, TdStripeWrite};
use crate::providers::traits::{ExistsTx, ExistsTxSelf, UpsertFirstLevel};

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct CreditNoteLineItem {
    #[primary_key]
    pub credit_note_line_item_id: Option<i64>,

    #[unique]
    pub id: String,

    pub r#type: String,

    pub credit_note_id: String,

    pub amount: i64,
    pub description: Option<String>,
    pub discount_amount: i64,
    pub discount_amounts: Value,
    pub invoice_line_item: Option<String>,
    pub quantity: Option<i64>,
    pub tax_amounts: Value,
    pub tax_rates: Value,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,

    pub livemode: bool,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetObjType for CreditNoteLineItem {
    fn get_obj_type_static() -> &'static str {
        "credit_note_line_item"
    }
}

impl GetId for CreditNoteLineItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

pub struct CreditNoteLineItemWithParentId<'a> {
    pub parent: String,
    pub data: &'a API::CreditNoteLineItem,
}


impl From<&CreditNoteLineItemWithParentId<'_>> for CreditNoteLineItem {
    fn from(x: &CreditNoteLineItemWithParentId) -> Self {
        let x2 = x.data.clone();

        CreditNoteLineItem {
            credit_note_line_item_id: None,
            id: x2.id,
            r#type: x2.type_x.to_json_key(),
            credit_note_id: x.parent.clone(),
            amount: x2.amount,
            description: x2.description,
            discount_amount: x2.discount_amount,
            discount_amounts: x2.discount_amounts.json(),
            invoice_line_item: x2.invoice_line_item,
            quantity: x2.quantity,
            tax_amounts: x2.tax_amounts.json(),
            tax_rates: x2.tax_rates.iter().map(|x3| x3.id.clone()).collect::<Vec<String>>().json(),
            unit_amount: x2.unit_amount,
            unit_amount_decimal: x2.unit_amount_decimal,
            livemode: x2.livemode,
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl<'b> WriteTree for CreditNoteLineItemWithParentId<'b> {
    type APIType = CreditNoteLineItemWithParentId<'b>;


    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &CreditNoteLineItemWithParentId) -> Vec<i64> {
        let mut x: CreditNoteLineItem = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &CreditNoteLineItemWithParentId) -> Vec<i64> {
        let mut x: CreditNoteLineItem = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &CreditNoteLineItemWithParentId) -> Vec<i64> {
        let mut x: CreditNoteLineItem = data.into();
        let write_id = x.tx_delete_log_write(utx, run_id, "id");
        vec![write_id]
    }
}

