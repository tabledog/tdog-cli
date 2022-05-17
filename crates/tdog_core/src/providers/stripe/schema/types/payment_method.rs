use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCustomerEDC00A, UniDefaultSource, UniPaymentMethod, UniPromotionCode};
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
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct PaymentMethod {
    #[primary_key]
    pub payment_method_id: Option<i64>,

    pub r#type: String,
    pub id: String,
    pub customer: Option<String>,

    pub alipay: Option<Value>,

    pub au_becs_debit: Option<Value>,

    pub bacs_debit: Option<Value>,

    pub bancontact: Option<Value>,

    pub billing_details: Value,

    pub card: Option<Value>,

    pub card_present: Option<Value>,

    pub eps: Option<Value>,

    pub fpx: Option<Value>,

    pub giropay: Option<Value>,

    pub grabpay: Option<Value>,

    pub ideal: Option<Value>,

    pub interac_present: Option<Value>,

    pub oxxo: Option<Value>,

    pub p24: Option<Value>,

    pub sepa_debit: Option<Value>,

    pub sofort: Option<Value>,

    pub created: DT,
    pub livemode: bool,

    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,

}

impl GetId for PaymentMethod {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::PaymentMethod> for PaymentMethod {
    fn from(i: &API::PaymentMethod) -> Self {
        PaymentMethod {
            payment_method_id: None,
            r#type: i.type_x.to_json_key(),
            id: i.id.clone(),
            customer: match &i.customer {
                Some(x) => match x {
                    UniCustomerEDC00A::String(s) => Some(s.clone()),
                    UniCustomerEDC00A::Customer(_) => unreachable!("Not expecting a full customer object on a PaymentMethod, should just be a customer id string.")
                },
                None => None,
            },
            alipay: i.alipay.json_or_none(),
            au_becs_debit: i.au_becs_debit.json_or_none(),
            bacs_debit: i.bacs_debit.json_or_none(),
            bancontact: i.bancontact.json_or_none(),
            billing_details: i.billing_details.json(),
            card: i.card.json_or_none(),
            card_present: i.card_present.json_or_none(),
            eps: i.eps.json_or_none(),
            fpx: i.fpx.json_or_none(),
            giropay: i.giropay.json_or_none(),
            grabpay: i.grabpay.json_or_none(),
            ideal: i.ideal.json_or_none(),
            interac_present: i.interac_present.json_or_none(),
            oxxo: i.oxxo.json_or_none(),
            p24: i.p24.json_or_none(),
            sepa_debit: i.sepa_debit.json_or_none(),
            sofort: i.sofort.json_or_none(),
            created: i.created.to_dt(),
            livemode: i.livemode,
            metadata: i.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl WriteTree for PaymentMethod {
    type APIType = API::PaymentMethod;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::PaymentMethod) -> Vec<i64> {
        let mut x: PaymentMethod = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::PaymentMethod) -> Vec<i64> {
        let mut x: PaymentMethod = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::PaymentMethod) -> Vec<i64> {
        unimplemented!("PaymentMethod is never deleted")
    }
}