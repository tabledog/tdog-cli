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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string, json_string_or_none, ToDT, ToJSONKey, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

/// Note: The Price API replaces the Plans API.
/// - @see https://stripe.com/docs/billing/subscriptions/subscription-schedules/use-cases
/// - @todo/next Download price data.
/// - @todo/low Check all fields are populated
///     - Did not check completely as this API is in the process of being deprecated; focus on the new paths first).
///     - Its not included in Sigmas tables.
#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct SubscriptionSchedule {
    #[primary_key]
    pub subscription_schedule_id: Option<i64>,

    #[unique]
    pub id: String,

    pub customer: String,
    pub subscription: Option<String>,
    pub canceled_at: Option<DT>,
    pub completed_at: Option<DT>,
    pub current_phase: Option<Value>,


    pub default_settings: Value,
    pub end_behavior: String,

    pub phases: Value,
    pub released_at: Option<DT>,
    pub released_subscription: Option<String>,
    pub status: String,
    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetId for SubscriptionSchedule {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::SubscriptionSchedule> for SubscriptionSchedule {
    fn from(x: &API::SubscriptionSchedule) -> Self {
        SubscriptionSchedule {
            subscription_schedule_id: None,
            id: x.id.clone(),
            customer: if let API::UniCustomerC00F6E::String(s2) = &x.customer {
                s2.clone().into()
            } else {
                unreachable!("customer should always be a string.");
            },
            subscription: x.subscription.as_ref().and_then(|x2| {
                if let API::UniSubscription::String(s) = x2 {
                    return s.clone().into();
                }
                unreachable!("Subscription should always be a string");
            }),
            canceled_at: x.canceled_at.and_then(|x| x.to_dt().into()),
            completed_at: x.completed_at.and_then(|x| x.to_dt().into()),
            current_phase: x.current_phase.json_or_none(),
            default_settings: x.default_settings.json(),
            end_behavior: x.end_behavior.to_json_key(),
            phases: x.phases.json(),
            released_at: x.released_at.and_then(|x| x.to_dt().into()),
            released_subscription: x.released_subscription.clone(),
            status: x.status.to_json_key(),
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl WriteTree for SubscriptionSchedule {
    type APIType = API::SubscriptionSchedule;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::SubscriptionSchedule) -> Vec<i64> {
        let mut x: SubscriptionSchedule = data.into();
        vec![x.tx_insert_set_pk_log_write(utx, run_id)]
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::SubscriptionSchedule) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: SubscriptionSchedule = data.into();
        writes.push(x.upsert_first_level(utx, run_id));
        writes
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::SubscriptionSchedule) -> Vec<i64> {
        unimplemented!("Cannot delete SubscriptionSchedules");
    }
}
