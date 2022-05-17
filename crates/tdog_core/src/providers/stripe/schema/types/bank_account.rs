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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, json_key, json_string_or_none, json_string_or_none_opt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct BankAccount {
    #[primary_key]
    pub bank_account_id: Option<i64>,

    #[unique]
    pub id: String,

    pub account: Option<String>,
    pub customer: Option<String>,
    pub account_holder_name: Option<String>,
    pub account_holder_type: Option<String>,
    pub available_payout_methods: Option<Value>,
    pub bank_name: Option<String>,
    pub country: String,
    pub currency: String,
    pub default_for_currency: Option<bool>,
    pub fingerprint: Option<String>,
    pub last4: String,
    pub routing_number: Option<String>,
    pub status: String,


    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetObjType for BankAccount {
    fn get_obj_type_static() -> &'static str {
        "bank_account"
    }
}

impl GetId for BankAccount {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::BankAccount> for BankAccount {
    fn from(x: &API::BankAccount) -> Self {
        let x = x.clone();

        BankAccount {
            bank_account_id: None,
            id: x.id,

            account: x.account.and_then(|x2| x2.get_id_any().into()),
            customer: x.customer.and_then(|x2| x2.get_id_any().into()),
            account_holder_name: x.account_holder_name,
            account_holder_type: x.account_holder_type,
            available_payout_methods: x.available_payout_methods.and_then(|x2| x2.iter().map(|x3| x3.to_json_key()).collect::<Vec<String>>().json().into()),
            bank_name: x.bank_name,
            country: x.country,
            currency: x.currency,
            default_for_currency: x.default_for_currency,
            fingerprint: x.fingerprint,
            last4: x.last4,
            routing_number: x.routing_number,
            status: x.status,
            insert_ts: None,
            update_ts: None,
        }
    }
}

impl WriteTree for BankAccount {
    type APIType = API::BankAccount;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::BankAccount) -> Vec<i64> {
        let mut x: BankAccount = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::BankAccount) -> Vec<i64> {
        let mut x: BankAccount = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::BankAccount) -> Vec<i64> {
        unimplemented!("Cannot delete BankAccounts");
    }
}
