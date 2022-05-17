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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdFromEnumOrNone, json_key, json_string_or_none, PickOpt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Dispute {
    #[primary_key]
    pub dispute_id: Option<i64>,

    #[unique]
    pub id: String,

    pub charge: String,
    pub payment_intent: Option<String>,

    pub amount: i64,

    pub balance_transactions: Value,
    pub currency: String,


    pub evidence: Value,

    pub evidence_details: Value,

    pub is_charge_refundable: bool,
    pub reason: String,
    pub status: String,

    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetId for Dispute {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Dispute> for Dispute {
    fn from(i: &API::Dispute) -> Self {
        Dispute {
            dispute_id: None,
            id: i.id.clone(),
            charge: if let UniCharge::String(s) = &i.charge {
                s.clone()
            } else {
                // Only expandable when requested one by one (not for list or events).
                unreachable!()
            },
            payment_intent: i.payment_intent.as_ref().and_then(|x| if let UniPaymentIntent::String(s) = x {
                s.clone().into()
            } else {
                unreachable!()
            }),
            amount: i.amount,

            // Note: `balance.available` is the only balance* event, and these are Balance not BalanceTransaction
            // @todo/low BalanceTransactions have no events, so cannot be kept 100% up to date after the first dl?
            // - Save ids vs entire JSON array?
            // balance_transactions: i.balance_transactions.iter().map(|x| x.id.clone()).collect::<Vec<String>>().json(),
            balance_transactions: i.balance_transactions.json(),
            currency: i.currency.clone(),
            evidence: i.evidence.json(),
            evidence_details: i.evidence.json(),
            is_charge_refundable: false,
            reason: i.reason.clone(),
            status: i.status.to_json_key(),
            created: i.created.to_dt(),
            livemode: i.livemode,
            metadata: i.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

impl WriteTree for Dispute {
    type APIType = API::Dispute;

    /// @see https://stripe.com/docs/payments/payment-intents/migration#saved-cards
    /// - This is useful to compare the state transitions between (old:sources, new:payment intent).
    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Dispute) -> Vec<i64> {
        let mut x: Dispute = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Dispute) -> Vec<i64> {
        let mut x: Dispute = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Dispute) -> Vec<i64> {
        // Cannot delete disputes.
        unimplemented!()
    }
}
