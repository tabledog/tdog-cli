use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCharge, UniDefaultSource, UniPaymentIntent, UniPaymentMethod, UniPromotionCode};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdFromEnum, GetIdFromEnumOrNone, json_key, json_string_or_none, PickOpt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Refund {
    #[primary_key]
    pub refund_id: Option<i64>,

    #[unique]
    pub id: String,

    pub balance_transaction: Option<String>,
    pub charge: Option<String>,
    pub failure_balance_transaction: Option<String>,
    pub payment_intent: Option<String>,
    pub source_transfer_reversal: Option<String>,
    pub transfer_reversal: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub description: Option<String>,
    pub failure_reason: Option<String>,
    pub reason: Option<String>,
    pub receipt_number: Option<String>,
    pub status: Option<String>,

    pub created: DT,
    // pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetId for Refund {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Refund> for Refund {
    fn from(i: &API::Refund) -> Self {
        Refund {
            refund_id: None,
            id: i.id.clone(),
            balance_transaction: i.balance_transaction.get_id_or_none(),
            charge: i.charge.get_id_or_none(),
            failure_balance_transaction: i.failure_balance_transaction.get_id_or_none(),
            payment_intent: i.payment_intent.get_id_or_none(),
            source_transfer_reversal: i.source_transfer_reversal.get_id_or_none(),
            transfer_reversal: i.transfer_reversal.get_id_or_none(),
            amount: i.amount,
            currency: i.currency.clone(),
            description: i.description.clone(),
            failure_reason: i.failure_reason.clone(),
            reason: i.reason.clone(),
            receipt_number: i.receipt_number.clone(),
            status: i.status.clone(),
            created: i.created.to_dt(),
            // livemode: i.livemode,
            metadata: i.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}



impl WriteTree for Refund {
    type APIType = API::Refund;

    /// @see https://stripe.com/docs/payments/payment-intents/migration#saved-cards
    /// - This is useful to compare the state transitions between (old:sources, new:payment intent).
    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Refund) -> Vec<i64> {
        let mut x: Refund = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Refund) -> Vec<i64> {
        let mut x: Refund = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Refund) -> Vec<i64> {
        // Cannot delete refunds.
        unimplemented!()
    }
}