use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniAccount, UniApplication, UniCustomerC00F6E, UniDefaultSource, UniLatestAttempt, UniMandate, UniPaymentMethod, UniPromotionCode};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, GetIdFromEnum, GetIdFromEnumOrNone, json_key, json_string_or_none, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

// @todo/low Use Serde::Value for JSON instead/in addition to `#[col_type = "json"]` to enforce connecting JSON types to row values (so they cannot accidentally be strings; useful when a type is either a string ID or an object).
#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct SetupIntent {
    #[primary_key]
    pub setup_intent_id: Option<i64>,

    #[unique]
    pub id: String,

    pub application: Option<String>,
    pub customer: Option<String>,

    pub latest_attempt: Option<String>,

    pub mandate: Option<String>,
    pub on_behalf_of: Option<String>,
    pub payment_method: Option<String>,
    pub single_use_mandate: Option<String>,
    pub cancellation_reason: Option<String>,
    pub client_secret: Option<String>,
    pub description: Option<String>,

    pub last_setup_error: Option<Value>,

    pub next_action: Option<Value>,

    pub payment_method_options: Option<Value>,

    pub payment_method_types: Value,

    pub status: String,
    pub usage_x: String,
    pub created: DT,
    pub livemode: bool,

    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetObjType for SetupIntent {
    fn get_obj_type_static() -> &'static str {
        "setup_intent"
    }
}

impl GetId for SetupIntent {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::SetupIntent> for SetupIntent {
    fn from(i: &API::SetupIntent) -> Self {
        SetupIntent {
            setup_intent_id: None,
            id: i.id.clone(),

            // Connect only, expandable
            application: i.application.as_ref().and_then(|x| if let UniApplication::String(s) = x { s.clone().into() } else { unreachable!("Expected application to be string not object.") }),

            customer: i.customer.as_ref().and_then(|x| if let UniCustomerC00F6E::String(c) = x { c.clone().into() } else { unreachable!("Expected customer string.") }),
            latest_attempt: i.latest_attempt.as_ref().and_then(|x| if let UniLatestAttempt::String(s) = x { s.clone().into() } else { unreachable!("Expected latest attempt to be a string.") }),
            mandate: i.mandate.as_ref().and_then(|x| if let UniMandate::String(s) = x { s.clone().into() } else { unreachable!("Expected mandate to be string.") }),
            on_behalf_of: i.on_behalf_of.as_ref().and_then(|x| if let UniAccount::String(s) = x { s.clone().into() } else { unreachable!("Expected account to be string.") }),
            payment_method: i.payment_method.as_ref().and_then(|x| x.get_id_any().into()),
            single_use_mandate: i.single_use_mandate.as_ref().and_then(|x| if let UniMandate::String(s) = x { s.clone().into() } else { unreachable!("Expected mandate to be string.") }),

            cancellation_reason: i.cancellation_reason.as_ref().and_then(|x| x.to_json_key().into()),
            client_secret: i.client_secret.clone(),
            description: i.description.clone(),
            last_setup_error: (*i.last_setup_error).as_ref().and_then(|x| x.json_or_none()),
            next_action: i.next_action.clone().json_or_none(),
            payment_method_options: i.payment_method_options.as_ref().and_then(|x| x.json_or_none()),
            payment_method_types: i.payment_method_types.json(),
            status: i.status.to_json_key(),
            usage_x: i.usage.clone(),
            created: i.created.to_dt(),
            livemode: i.livemode,
            metadata: i.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl WriteTree for SetupIntent {
    type APIType = API::SetupIntent;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::SetupIntent) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: SetupIntent = data.into();

        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));

        if let Some(x2) = &data.payment_method {
            match x2 {
                UniPaymentMethod::PaymentMethod(x3) => {
                    // Upsert as these are listed one per *active* customer at dl time via API call (custId, paymentMethodType).
                    // - These need to be expanded to insert payment methods for deleted customers.
                    writes.append(&mut PaymentMethod::upsert_tree(utx, run_id, &x3));
                }
                UniPaymentMethod::String(id) => {
                    // Issue: This can be a string even when expand=true.
                    // - E.g. `stripe setup_intents list --expand data.payment_method` can return a string `payment_method`.
                    // - This happens when a SetupIntent is created without explicitly attaching a payment method.
                    // - The payment_method ID seems to be a placeholder - it cannot be expanded or fetched directly.
                    // Fix: Ignore for now - no queryable data, customer should be referencing the SetupIntent.id.

                    // unreachable!("SetupIntent.payment_method should always be expanded at dl time.");

                    debug!("Ignoring non-data placeholder SetupIntent.payment_method ID {:?}", id);
                }
            }
        }

        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::SetupIntent) -> Vec<i64> {
        let mut x: SetupIntent = data.into();
        vec![x.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::SetupIntent) -> Vec<i64> {
        unimplemented!("Cannot delete SetupIntent")
    }
}