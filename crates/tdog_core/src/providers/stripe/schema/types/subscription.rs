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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, json_key, json_string, json_string_or_none, json_string_or_none_opt, ToDT, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, unix_to_iso_wrap, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema::types::SubscriptionItem;
use crate::providers::stripe::schema_meta::{DeleteStaticLogWrite, GetInferredDeletes, LogWrite};
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Subscription {
    // pub object: UniStrObject59F834,
    #[primary_key]
    pub subscription_id: Option<i64>,

    #[unique]
    pub id: String,
    pub customer: String,

    pub default_payment_method: Option<String>,
    pub default_source: Option<String>,
    pub latest_invoice: Option<String>,
    pub pending_setup_intent: Option<String>,

    pub schedule: Option<String>,

    pub application_fee_percent: Option<f64>,
    pub billing_cycle_anchor: DT,

    pub billing_thresholds: Option<Value>,

    pub cancel_at: Option<DT>,
    pub cancel_at_period_end: bool,
    pub canceled_at: Option<DT>,
    pub collection_method: Option<String>,
    pub current_period_end: DT,
    pub current_period_start: DT,
    pub days_until_due: Option<i64>,


    pub default_tax_rates: Option<String>,

    pub discount: Option<String>,

    pub ended_at: Option<String>,

    // Blank list = `[]` (match semantics of network value).
    // - Assumption: these always contain 100% of items (current limit is 20 items per sub - all always included in both download and events).
    pub items: Value,

    pub next_pending_invoice_item_invoice: Option<String>,

    pub pause_collection: Option<Value>,

    pub pending_invoice_item_interval: Option<Value>,


    pub pending_update: Option<Value>,

    pub start_date: DT,
    pub status: String,

    pub transfer_data: Option<Value>,

    pub trial_end: Option<String>,
    pub trial_start: Option<String>,

    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetId for Subscription {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Subscription> for Subscription {
    fn from(x: &API::Subscription) -> Self {
        let customer = if let API::UniCustomerC00F6E::String(s) = &x.customer {
            s.clone()
        } else {
            // @todo/next test with deleted customer.
            unreachable!("Subscription should always have customer string.");
        };


        // @todo/high Possible issue: If more than 10 items are added in a single API call, only the last 10 are inserted.
        // - sub items do not have their own update events.
        // - If added one by one, each event is upserted - inserting all items.
        // - Fix, A: If the event is within 1 min of now, list all subscription items via the dl api.
        assert!(!x.items.has_more, "has_more=true for subscription.items property.");

        Subscription {
            subscription_id: None,
            id: x.id.clone(),
            customer,
            default_payment_method: x.default_payment_method.as_ref().and_then(|x2| x2.get_id_any().into()),
            default_source: (*x.default_source).as_ref().and_then(|x2| {
                if let UniDefaultSource::String(s) = x2 {
                    return s.clone().into();
                }
                unreachable!("default_source should always be a string");
            }),
            latest_invoice: x.latest_invoice.as_ref().and_then(|x2| {
                if let API::UniInvoice::String(s) = x2 {
                    return s.clone().into();
                }
                unreachable!("latest_invoice should always be a string");
            }),
            pending_setup_intent: x.pending_setup_intent.as_ref().and_then(|x2| {
                if let API::UniSetupIntent::String(s) = x2 {
                    return s.clone().into();
                }
                unreachable!("pending_setup_intent should always be a string");
            }),
            schedule: x.schedule.to_json_key_or_none(),
            application_fee_percent: x.application_fee_percent,
            billing_cycle_anchor: x.billing_cycle_anchor.to_dt(),
            billing_thresholds: x.billing_thresholds.json_or_none(),
            cancel_at: x.cancel_at.and_then(|x2| x2.to_dt().into()),
            cancel_at_period_end: x.cancel_at_period_end,
            canceled_at: x.canceled_at.and_then(|x2| x2.to_dt().into()),
            collection_method: x.collection_method.to_json_key_or_none(),
            current_period_end: x.current_period_end.to_dt(),
            current_period_start: x.current_period_start.to_dt(),
            days_until_due: x.days_until_due,
            default_tax_rates: x.default_tax_rates.as_ref().and_then(|x| x.get_pks_json_opt()),

            // Coupons applied -> Discount.
            // - Discounts = !has_dl_list && !has_direct_events
            discount: x.discount.as_ref().and_then(|x2| x2.id.clone().into()),
            ended_at: unix_to_iso_wrap(x.ended_at),
            items: x.items.data.iter().map(|x2| x2.id.clone()).collect::<Vec<String>>().json(),
            next_pending_invoice_item_invoice: unix_to_iso_wrap(x.next_pending_invoice_item_invoice),
            pause_collection: x.pause_collection.json_or_none(),
            pending_invoice_item_interval: x.pending_invoice_item_interval.json_or_none(),
            pending_update: x.pending_update.json_or_none(),
            start_date: x.start_date.to_dt(),
            status: json_key(&x.status),
            transfer_data: x.transfer_data.json_or_none(),
            trial_end: unix_to_iso_wrap(x.trial_end),
            trial_start: unix_to_iso_wrap(x.trial_start),
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

/// sub_items = !has_dl_list && !has_direct_event, so deletes are just "items missing from sub.items update event".
///
/// Alternative: delete all items, insert active ones.
/// - This would require less network round trips, but the write log would be less useful.
///     - E.g. All types deleted, then re-created, when actually they may have been only updated (never deleted via the Stripe API).
///         - If the write log is used to relay writes to another store (SQLite -> Redis), this would cause unnecessary deletes/re-creates.
///         - General: The write log should closely mimic the Stripe API writes.
///
/// Note: When (download, first_apply), applying sub.update events older than the download point will result in deleting and recreating the newest sub items (c,d,c,...)
fn delete_missing_items_log_writes(utx: &mut UniTx, run_id: i64, data: &API::Subscription, writes: &mut Vec<i64>) {
    let active_items = data.items.data.iter().map(|x| x.id.as_str()).collect();
    let deletes = SubscriptionItem::get_inferred_deleted_items(utx, "subscription", &data.id, active_items);
    for x in deletes {
        writes.push(SubscriptionItem::tx_delete_static_log_write(utx, run_id, &x));
    }
}


impl WriteTree for Subscription {
    type APIType = API::Subscription;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Subscription) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Subscription = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));


        assert!(!data.items.has_more, "Expected subscription to always contain 100% of items for both downloads and update events. Found has_more=true. Subscription.id={}", &data.id);

        // Upsert not needed here as the subscription node is the only path to an item.
        for x in &data.items.data {
            writes.append(&mut SubscriptionItem::insert_tree(utx, run_id, &x));
        }

        if let Some(x2) = &data.default_payment_method {
            match x2 {
                UniPaymentMethod::PaymentMethod(x3) => {
                    // Upsert as these are listed one per *active* customer at dl time via API call (custId, paymentMethodType).
                    // - These need to be expanded to insert payment methods for deleted customers.
                    writes.append(&mut PaymentMethod::upsert_tree(utx, run_id, &x3));
                }
                UniPaymentMethod::String(_) => unreachable!("Subscription.default_payment_method should always be expanded at dl time."),
            }
        }

        // Upsert discount.
        if let Some(x2) = &data.discount {
            writes.append(&mut Discount::upsert_tree(utx, run_id, &x2));
        }

        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Subscription) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Subscription = data.into();
        writes.push(x.upsert_first_level(utx, run_id));


        // Upsert sub items.
        assert!(!data.items.has_more, "Expected subscription to always contain 100% of items for both downloads and update events. Found has_more=true. Subscription.id={}", &data.id);

        delete_missing_items_log_writes(utx, run_id, &data, &mut writes);

        for x in &data.items.data {
            writes.append(&mut SubscriptionItem::upsert_tree(utx, run_id, &x));
        }


        // Upsert discount.
        if let Some(x2) = &data.discount {
            writes.append(&mut Discount::upsert_tree(utx, run_id, &x2));
        }

        writes
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Subscription) -> Vec<i64> {
        unimplemented!("Cannot delete Subscriptions");
    }
}
