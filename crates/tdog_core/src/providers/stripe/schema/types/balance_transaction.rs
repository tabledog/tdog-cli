use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCharge, UniDefaultSource, UniPaymentIntent, UniPaymentMethod, UniPromotionCode, UniSource231860};
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
pub struct BalanceTransaction {
    #[primary_key]
    pub balance_transaction_id: Option<i64>,

    #[unique]
    pub id: String,

    pub r#type: String,

    pub source: Option<String>,
    pub amount: i64,
    pub available_on: DT,
    pub currency: String,
    pub description: Option<String>,
    pub exchange_rate: Option<f64>,
    pub fee: i64,

    pub fee_details: Value,
    pub net: i64,
    pub reporting_category: String,
    pub status: String,

    pub created: DT,

    #[insert_ts]
    pub insert_ts: Option<DT3>,
}


impl GetId for BalanceTransaction {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::BalanceTransaction> for BalanceTransaction {
    fn from(i: &API::BalanceTransaction) -> Self {
        BalanceTransaction {
            balance_transaction_id: None,
            id: i.id.clone(),

            r#type: i.type_x.to_json_key(),
            source: i.source.as_ref().as_ref().and_then(|x| match x {
                UniSource231860::String(s) => s.clone().into(),
                _ => unreachable!("Expected BalanceTransaction.source to always be a string, found object")
            }),
            amount: i.amount,
            available_on: i.available_on.to_dt(),
            currency: i.currency.clone(),
            description: i.description.clone(),
            exchange_rate: i.exchange_rate.clone(),
            fee: i.fee,
            fee_details: i.fee_details.json(),
            net: i.net,
            reporting_category: i.reporting_category.clone(),
            status: i.status.clone(),
            created: i.created.to_dt(),

            insert_ts: None,
        }
    }
}


impl WriteTree for BalanceTransaction {
    type APIType = API::BalanceTransaction;

    /// @see https://stripe.com/docs/payments/payment-intents/migration#saved-cards
    /// - This is useful to compare the state transitions between (old:sources, new:payment intent).
    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::BalanceTransaction) -> Vec<i64> {
        let mut x: BalanceTransaction = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::BalanceTransaction) -> Vec<i64> {
        let mut x: BalanceTransaction = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::BalanceTransaction) -> Vec<i64> {
        // Cannot delete BalanceTransaction.
        unimplemented!()
    }
}