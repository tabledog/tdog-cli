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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdFromEnumOrNone, json_key, json_string_or_none, PickOpt, ToDT, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::refund::Refund;
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::{ExistsTx, UpsertFirstLevel};

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Charge {
    #[primary_key]
    pub charge_id: Option<i64>,

    #[unique]
    pub id: String,

    pub paid: bool,
    pub application: Option<String>,
    pub application_fee: Option<String>,
    pub balance_transaction: Option<String>,
    pub customer: Option<String>,
    pub invoice: Option<String>,
    pub on_behalf_of: Option<String>,
    // @todo/high Restricted SQL keywords.
    // - Adding `_x` suffix seems to be easiest.
    //      - No need to escape columns in x different SQL dialects.
    //      - Can easily detect it with regex `_x$`
    pub order_id: Option<String>,
    pub payment_intent: Option<String>,
    pub review: Option<String>,
    pub source_transfer: Option<String>,
    pub transfer: Option<String>,
    pub amount: i64,
    pub amount_captured: i64,
    pub amount_refunded: i64,
    pub application_fee_amount: Option<i64>,

    pub billing_details: Value,
    pub calculated_statement_descriptor: Option<String>,
    pub captured: bool,
    pub currency: String,
    pub description: Option<String>,
    pub disputed: bool,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,

    pub fraud_details: Option<Value>,

    pub outcome: Option<Value>,

    pub payment_method: Option<String>,


    pub payment_method_details: Option<Value>,
    pub payment_method_details_type: Option<String>,

    pub receipt_email: Option<String>,
    pub receipt_number: Option<String>,
    pub receipt_url: Option<String>,
    pub refunded: bool,


    pub refunds: String,

    // #[skip]
    // pub refunds_2: Vec<Refund>,


    pub shipping: Option<Value>,
    pub statement_descriptor: Option<String>,
    pub statement_descriptor_suffix: Option<String>,
    pub status: String,

    pub transfer_data: Option<String>,
    pub transfer_group: Option<String>,

    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetId for Charge {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Charge> for Charge {
    fn from(i: &API::Charge) -> Self {
        Charge {
            charge_id: None,
            id: i.id.clone(),
            paid: i.paid,
            application: i.application.get_id_or_none(),
            application_fee: i.application_fee.get_id_or_none(),
            balance_transaction: i.balance_transaction.get_id_or_none(),
            customer: i.customer.get_id_or_none(),
            invoice: i.invoice.get_id_or_none(),
            on_behalf_of: i.on_behalf_of.get_id_or_none(),
            order_id: i.order.get_id_or_none(),
            payment_intent: i.payment_intent.get_id_or_none(),
            review: i.review.get_id_or_none(),
            source_transfer: i.source_transfer.get_id_or_none(),
            transfer: i.transfer.get_id_or_none(),
            amount: i.amount,
            amount_captured: i.amount_captured,
            amount_refunded: i.amount_refunded,
            application_fee_amount: i.application_fee_amount.clone(),
            billing_details: i.billing_details.json(),
            calculated_statement_descriptor: i.calculated_statement_descriptor.clone(),
            captured: i.captured,
            currency: i.currency.clone(),
            description: i.description.clone(),
            disputed: i.disputed,
            failure_code: i.failure_code.clone(),
            failure_message: i.failure_message.clone(),
            fraud_details: i.fraud_details.json_or_none(),
            outcome: i.outcome.json_or_none(),
            payment_method: i.payment_method.clone(),

            // @todo/next Serde will add `key: null` when key was not in the original JSON.
            payment_method_details: i.payment_method_details.json_or_none(),
            payment_method_details_type: i.payment_method_details.pick_opt(|x| &x.type_x),
            receipt_email: i.receipt_email.clone(),
            receipt_number: i.receipt_number.clone(),
            receipt_url: i.receipt_url.clone(),
            refunded: i.refunded,
            refunds: i.refunds.data.get_pks_json(),
            // refunds_2: i.refunds.data.iter().map(|x| x.into()).collect(),


            shipping: i.shipping.json_or_none(),
            statement_descriptor: i.statement_descriptor.clone(),
            statement_descriptor_suffix: i.statement_descriptor_suffix.clone(),
            status: i.status.clone(),
            transfer_data: i.transfer_data.to_json_key_or_none(),
            transfer_group: i.transfer_group.clone(),
            created: i.created.to_dt(),
            livemode: i.livemode,
            metadata: i.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

impl WriteTree for Charge {
    type APIType = API::Charge;

    /// @see https://stripe.com/docs/payments/payment-intents/migration#saved-cards
    /// - This is useful to compare the state transitions between (old:sources, new:payment intent).
    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Charge) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Charge = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));

        // Refunds = has_direct_list && !has_direct_event
        // Refunds exist as children on (OrderReturn, TransferReversal, CreditNote), so can be inserted already from any of these depending on download order.
        for r in &data.refunds.data {
            writes.append(&mut Refund::upsert_tree(utx, run_id, &r));
        }

        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Charge) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Charge = data.into();
        writes.push(x.upsert_first_level(utx, run_id));

        /// It is not possible to know at this point if this is a c or u.
        /// - There is no `refund.created` event, only `charge.refund.updated` (refund creates are communicated via parent Charge events).
        /// - Refunds are always included in Charges (both dl and events), with no `expand` key needed.
        for r in &data.refunds.data {
            writes.append(&mut Refund::upsert_tree(utx, run_id, &r));
        }

        writes
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Charge) -> Vec<i64> {
        // Cannot delete charges.
        unimplemented!()
    }
}