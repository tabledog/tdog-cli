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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, GetIdFromEnum, GetIdFromEnumOrNone, json_key, json_string_or_none, ToDT, ToJSONKey, ToJSONKeyOrNone, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct PaymentIntent {
    #[primary_key]
    pub payment_intent_id: Option<i64>,

    #[unique]
    pub id: String,

    // @todo/next download all of these, assign FKs.
    pub application: Option<String>,
    pub customer: Option<String>,
    pub invoice: Option<String>,
    pub on_behalf_of: Option<String>,
    pub payment_method: Option<String>,
    pub review: Option<String>,
    pub amount: i64,
    pub amount_capturable: Option<i64>,
    pub amount_received: Option<i64>,
    pub application_fee_amount: Option<i64>,
    pub canceled_at: Option<DT>,
    pub cancellation_reason: Option<String>,
    pub capture_method: String,


    // This is a list limited to just one item (has_more=true when the length is >1).
    // - Users can query via `charge.payment_intent`
    // pub charges: Option<String>,

    pub client_secret: Option<String>,

    pub confirmation_method: String,
    pub currency: String,
    pub description: Option<String>,


    pub last_payment_error: Option<Value>,


    pub next_action: Option<Value>,


    pub payment_method_options: Option<Value>,


    pub payment_method_types: Option<Value>,
    pub receipt_email: Option<String>,
    pub setup_future_usage: Option<String>,


    pub shipping: Option<Value>,

    pub statement_descriptor: Option<String>,
    pub statement_descriptor_suffix: Option<String>,
    pub status: String,

    // pub transfer_data: Option<TransferData>,
    pub transfer_data_destination: Option<String>,
    pub transfer_data_amount: Option<i64>,

    pub transfer_group: Option<String>,

    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetId for PaymentIntent {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::PaymentIntent> for PaymentIntent {
    fn from(x: &API::PaymentIntent) -> Self {

        PaymentIntent {
            payment_intent_id: None,
            id: x.id.clone(),
            application: x.application.get_id_or_none(),
            customer: x.customer.get_id_or_none(),
            invoice: x.invoice.get_id_or_none(),
            on_behalf_of: x.on_behalf_of.get_id_or_none(),
            payment_method: x.payment_method.as_ref().and_then(|x| x.get_id_any().into()),
            review: x.review.get_id_or_none(),
            amount: x.amount,
            amount_capturable: x.amount_capturable.clone(),
            amount_received: x.amount_received.clone(),
            application_fee_amount: x.application_fee_amount.clone(),
            canceled_at: x.canceled_at.and_then(|x| x.to_dt().into()),
            cancellation_reason: x.cancellation_reason.to_json_key_or_none(),
            capture_method: x.capture_method.to_json_key(),
            // x.charges - only contains max 1 item before has_more=true.
            // charges: x.charges.as_ref().and_then(|x| x.data.get_pks_json_opt().into()),
            client_secret: x.client_secret.clone(),
            confirmation_method: x.confirmation_method.to_json_key(),
            currency: x.currency.clone(),
            description: x.description.clone(),
            last_payment_error: x.last_payment_error.json_or_none(),
            next_action: x.next_action.json_or_none(),
            payment_method_options: x.payment_method_options.json_or_none(),
            payment_method_types: x.payment_method_types.json_or_none(),
            receipt_email: x.receipt_email.clone(),
            setup_future_usage: x.setup_future_usage.to_json_key_or_none(),
            shipping: x.shipping.json_or_none(),
            statement_descriptor: x.statement_descriptor.clone(),
            statement_descriptor_suffix: x.statement_descriptor_suffix.clone(),
            status: x.status.to_json_key(),
            transfer_data_destination: x.transfer_data.as_ref().and_then(|x| x.destination.get_id().into()),
            transfer_data_amount: x.transfer_data.as_ref().and_then(|x| x.amount.into()),
            transfer_group: x.transfer_group.clone(),
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}


impl WriteTree for PaymentIntent {
    type APIType = API::PaymentIntent;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::PaymentIntent) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: PaymentIntent = data.into();

        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));


        if let Some(x2) = &data.payment_method {
            match x2 {
                UniPaymentMethod::PaymentMethod(x3) => {
                    // Upsert as these are listed one per *active* customer at dl time via API call (custId, paymentMethodType).
                    // - These need to be expanded to insert payment methods for deleted customers.
                    writes.append(&mut PaymentMethod::upsert_tree(utx, run_id, &x3));
                }
                UniPaymentMethod::String(_) => unreachable!("PaymentIntent.payment_method should always be expanded at dl time."),
            }
        }


        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::PaymentIntent) -> Vec<i64> {
        let mut u: PaymentIntent = data.into();
        vec![u.upsert_first_level(utx, run_id)]
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::PaymentIntent) -> Vec<i64> {
        unimplemented!("No delete for PaymentIntent");
    }
}
