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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string_or_none, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Source {
    #[primary_key]
    pub source_id: Option<i64>,

    #[unique]
    pub id: String,
    pub r#type: String,
    pub customer: Option<String>,

    pub ach_credit_transfer: Option<Value>,
    pub ach_debit: Option<Value>,
    pub alipay: Option<Value>,
    pub amount: Option<i64>,
    pub au_becs_debit: Option<Value>,
    pub bancontact: Option<Value>,
    pub card: Option<Value>,
    pub card_present: Option<Value>,
    pub client_secret: String,
    pub code_verification: Option<Value>,
    pub currency: Option<String>,

    pub eps: Option<Value>,
    pub flow: String,
    pub giropay: Option<Value>,
    pub ideal: Option<Value>,
    pub klarna: Option<Value>,
    pub multibanco: Option<Value>,
    pub owner: Option<Value>,
    pub p24: Option<Value>,
    pub receiver: Option<Value>,
    pub redirect: Option<Value>,
    pub sepa_debit: Option<Value>,
    pub sofort: Option<Value>,
    pub source_order: Option<Value>,
    pub statement_descriptor: Option<String>,
    pub status: String,
    pub three_d_secure: Option<Value>,
    pub usage_x: Option<String>,
    pub wechat: Option<Value>,

    pub created: DT,
    pub livemode: bool,
    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetId for Source {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Source> for Source {
    fn from(x: &API::Source) -> Self {
        let x = x.clone();

        Source {
            source_id: None,
            id: x.id,
            r#type: x.type_x.to_json_key(),
            customer: x.customer,
            ach_credit_transfer: x.ach_credit_transfer.json_or_none(),
            ach_debit: x.ach_debit.json_or_none(),
            alipay: x.alipay.json_or_none(),
            amount: x.amount,
            au_becs_debit: x.au_becs_debit.json_or_none(),
            bancontact: x.bancontact.json_or_none(),
            card: x.card.json_or_none(),
            card_present: x.card_present.json_or_none(),
            client_secret: x.client_secret,
            code_verification: x.code_verification.json_or_none(),
            currency: x.currency,
            eps: x.eps.json_or_none(),
            flow: x.flow,
            giropay: x.giropay.json_or_none(),
            ideal: x.ideal.json_or_none(),
            klarna: x.klarna.json_or_none(),
            multibanco: x.multibanco.json_or_none(),
            owner: x.owner.json_or_none(),
            p24: x.p24.json_or_none(),
            receiver: x.receiver.json_or_none(),
            redirect: x.redirect.json_or_none(),
            sepa_debit: x.sepa_debit.json_or_none(),
            sofort: x.sofort.json_or_none(),
            source_order: x.source_order.json_or_none(),
            statement_descriptor: x.statement_descriptor,
            status: x.status,
            three_d_secure: x.three_d_secure.json_or_none(),
            usage_x: x.usage,
            wechat: x.wechat.json_or_none(),
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None
        }
    }
}


impl WriteTree for Source {
    type APIType = API::Source;

    /// @see https://stripe.com/docs/payments/payment-intents/migration#saved-cards
    /// - This is useful to compare the state transitions between (old:sources, new:payment intent).
    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Source) -> Vec<i64> {
        let mut x: Source = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Source) -> Vec<i64> {
        let mut x: Source = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Source) -> Vec<i64> {
        unimplemented!("Cannot delete Source")
    }
}