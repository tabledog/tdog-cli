use std::collections::HashMap;
use std::collections::HashSet;
use std::{mem, cmp};
use std::time::{Duration, Instant};

use chrono::{DateTime, NaiveDateTime, Utc, SubsecRound};
use futures::future::join_all;
use futures::future::TryFutureExt;
use futures::join;
use futures::stream::{self, Stream, StreamExt, TryStream, TryStreamExt};
use futures::try_join;
use futures_util::pin_mut;
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{info, trace, warn, error};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use humantime::format_duration;
use stripe_client::http::http::{Config, StripeClient, UniErr};
use stripe_client::types::req_params::{GetCharges, GetCustomers, GetEvents, GetPaymentIntents, GetPrices, GetProducts, GetSubscriptionItems, GetSubscriptions, GetSubscriptionSchedules, GetTaxRates, UniStrStatus3EB683, UniCreated, RangeQuerySpecs};
use stripe_client::types::responses::{UniPolymorphic646C3F, UniPolymorphic70BAFA};
use stripe_client::types::types::{GetId, UniCharge, UniCustomerC00F6E, UniNotificationEventDataObject};
use stripe_client::types::types as API;
use tokio::time;
use unicon::{*};
use unicon::dt::{*};
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

use crate::fns::now_3;
use crate::providers::stripe::schema::{Db, GetIdAndObject, ToISODate, ToJSONOrNone, ToJSONString, ToVal, WriteTree};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::balance_transaction::BalanceTransaction;
use crate::providers::stripe::schema::types::bank_account::BankAccount;
use crate::providers::stripe::schema::types::card::Card;
use crate::providers::stripe::schema::types::coupon::Coupon;
use crate::providers::stripe::schema::types::credit_note::CreditNote;
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema::types::dispute::Dispute;
use crate::providers::stripe::schema::types::invoice::Invoice;
use crate::providers::stripe::schema::types::invoiceitem::Invoiceitem;
use crate::providers::stripe::schema::types::order::Order;
use crate::providers::stripe::schema::types::order_return::OrderReturn;
use crate::providers::stripe::schema::types::payment_method::PaymentMethod;
use crate::providers::stripe::schema::types::plan::Plan;
use crate::providers::stripe::schema::types::promotion::PromotionCode;
use crate::providers::stripe::schema::types::refund::Refund;
use crate::providers::stripe::schema::types::setup_intent::SetupIntent;
use crate::providers::stripe::schema::types::sku::Sku;
use crate::providers::stripe::schema::types::tax_id::TaxId;
use crate::providers::traits::{ExistsTx, GetInsertTs};
use crate::Stripe;

use super::schema_meta::{*};
use stripe_client::types::types::UniStrObject6D0693::Event;

// //use unicon::{UniCon, UniTx};
// //use unicon::{
//     Insert,
//     DbStatic,
// };
//use unicon::{
//     *
// };

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
enum Action {
    /// These are all `write_id`'s that occurred from a single event.
    /// - single event_id -> [write_ids] (as child rows may be inserted).
    /// - This would be useful to determine the result (insert/update) of child rows.
    ///
    /// - @todo/low Possibly map (download_json_obj_id -> write_ids) so that the break down of any JSON tree into writes is observable.
    ///     - May come in useful when determining different paths to child writes (e.g. a download with expanded children objects vs an event with just string ids).
    Write(Vec<i64>),
    Skip(Skip),
}


impl Action {
    /// Returns a `x.y` string for inserting into a row.
    /// E.g. `skip.not_data_write`
    fn get_key(&self) -> String {
        match self {
            Action::Write(x) => {
                // @todo/low Add write_ids to row.
                return "write".into();
            }
            Action::Skip(x) => {
                match serde_json::to_value(x).unwrap() {
                    Value::String(s) => s,
                    _ => unreachable!()
                }
            }
        }
    }

    fn log(&self, utx: &mut UniTx, run_id: i64, event_id: String) {
        let mut write_ids = None;
        match &self {
            Action::Write(x) => {
                write_ids = x.json().into();
            }
            _ => {}
        }

        let mut apply_row = TdStripeApplyEvent {
            apply_id: None,
            run_id,
            event_id,
            action: self.get_key(),
            write_ids,
            insert_ts: None,
        };

        apply_row.tx_insert_set_pk(utx);
    }
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
enum Skip {
    /// Either:
    /// A. This event is a general notification/never a row write.
    /// B. This is a data write, but not currently implemented by TD yet (and may be in the future).
    #[serde(rename = "skip.not_data_write")]
    NotDataWrite,

    // When: first_dl, first_apply
    // - If the event_ts is before the row insert_ts, and that insert_ts was the download time (indicating the current version timestamp).
    // #[serde(rename = "skip.old_data")]
    // OldData,
}


/// Make polling once per second very cheap when there are no new events.
/// - E.g. do not download a page of 100 events using `starting_after` when all the contained events have already been applied.
async fn has_events_after(c: &StripeClient, last_event: &String) -> bool {
    let p = Some(GetEvents {
        type_x: None,
        created: None,
        delivery_success: None,
        ending_before: Some(last_event.clone()),
        expand: None,
        limit: Some(1),
        starting_after: None,
        types: None,
    });


    // Assumption: This `unwrap` will exit the process if:
    // - The user has deleted all data for the Stripe account (they should drop the DB and start fresh).
    // - Two TD processes with different Stripe API keys pointing to the same DB.
    // - Running the CLI against a DB that is > 30 days old, so has missing events.
    let res = c.v1_events_get(&p).await.unwrap();
    res.data.len() > 0
}

/// @todo/low Detect if (db, account) have been mixed up (a db for one account used with another account).
/// - This is an operator error, but it should still be protected against.
async fn get_all_unapplied_events(c: &StripeClient, uc: &mut UniCon, created_gte: Option<i64>) -> Vec<API::NotificationEvent> {
    let mut o = vec![];

    let mut last_event = None;
    let last = TdStripeApplyEvent::get_last(uc, "apply_id");
    if let Some(x) = last {
        last_event = Some(x.event_id.clone());
        debug!("Last event that was applied to db: {}.", &x.event_id);

        if !has_events_after(&c, &x.event_id).await {
            return o;
        }
    }

    // When: Last run was first download.
    let mut created = None;
    if let Some(x) = created_gte {
        created = Some(UniCreated::RangeQuerySpecs(RangeQuerySpecs {
            gt: None,
            gte: Some(x),
            lt: None,
            lte: None,
        }))
    }

    /// @todo/low https://stripe.com/docs/api/events/types
    /// - Should this be limited to types (created, updated, deleted), or do other events still indicate an update that needs to be written?
    let p = Some(GetEvents {
        type_x: None,
        created,
        delivery_success: None,
        ending_before: None,
        expand: None,
        limit: Some(100),
        starting_after: None,
        types: None,
    });


    /// Walk from now back in time.
    /// - Events API result always ordered by `created` DESC.
    /// - `starting_after` = walk back in time (implicitly starting from `now` by getting the `now` page by passing `starting_after=null`
    /// - `ending_before` = walk forward in time.
    ///     - This is not used as the loop will never reach the end of the stream if events happen 1/sec (as there will always be new pages to download by the time the previous request has completed).
    ///         - Manually looking for the `last_event_id` so that the direction is consistent (instead of a `starting_after` for the initial events dl and then `ending_before` for all subsequent event downloads.
    /// - Not using `created.gt` because many events happen on the same second; possible potential to miss events.
    let mut st = c.v1_events_get_st(&p.unwrap());
    pin_mut!(st);
    'walk: while let Some(val) = st.next().await {
        let events = val.unwrap().data;

        trace!("Downloaded a page of {} events.", events.len());
        for e in events {
            if let Some(last_event_id) = &last_event {
                if e.id == *last_event_id {
                    break 'walk;
                }
            }

            o.push(e);
        }
    }

    // Note: events coming from the API are listed by most recent first.
    // - Order by created asc to make loops loop from event list start->end.
    o.reverse();

    if o.len() > 1 {
        info!("Downloaded {} new events from {} to {} inclusive.", o.len(), o.first().unwrap().id, o.last().unwrap().id);
    }
    if o.len() == 1 {
        info!("Downloaded 1 new event: {}.", o.first().unwrap().id);
    }

    // Ensure sorted by created ASC.
    // Docs = `fast in cases where the slice is nearly sorted`
    // o.sort_by_key(|k| k.created); // Do not sort. Keep Stripe response order in case two events have the same `created` value, but one event must always become before another (E.g. child updates before parent).
    o
}


fn insert_all(utx: &mut UniTx<'_>, run_id: i64, e: &Vec<API::NotificationEvent>) {
    for i in e {
        let mut i2: NotificationEvent = (i).into();
        i2.tx_insert_set_pk_log_write(utx, run_id);
    }
}


fn log_skip_not_data_write(utx: &mut UniTx, run_id: i64, event_id: String) {
    (Action::Skip(Skip::NotDataWrite)).log(utx, run_id, event_id);
}


pub fn apply_events_body(utx: &mut UniTx<'_>, run_id: i64, events: &Vec<API::NotificationEvent>) {
    if events.len() > 1 {
        // order created asc
        assert!(events.first().unwrap().created <= events.last().unwrap().created);
    }

    insert_all(utx, run_id, &events);

    for e in events {
        if !Db::event_is_table_write(&e) {
            log_skip_not_data_write(utx, run_id, e.id.clone());
            continue;
        }

        let action = write_one_event(utx, run_id, &e);
        action.log(utx, run_id, e.id.clone());
    }
}


fn write_one_event(utx: &mut UniTx, run_id: i64, e: &API::NotificationEvent) -> Action {
    use UniNotificationEventDataObject as Obj;

    /// Match represents current set of implemented `UpsertTree` (over time more will be implemented).
    let action = match &(*e.data.object) {
        // Account(x) => write_one(&utx, run_id,  &e, x),
        // PlatformFee(x) => write_one(&utx, run_id,  &e, x),
        // No need to insert this as it is just the "current value" of balance? Use can just `select json from events where type=balance.available order by id desc limit 1`.
        // Balance(x) => write_one::<_, Balance>(&utx, run_id,  &e, x)
        // AccountCapability(x) => write_one(&utx, run_id,  &e, x),
        Obj::Charge(x) => write_one::<_, Charge>(utx, run_id, &e, x),
        // Session(x) => write_one(&utx, run_id,  &e, x),
        Obj::Coupon(x) => write_one::<_, Coupon>(utx, run_id, &e, x),
        Obj::CreditNote(x) => {
            let x2 = x.as_ref();
            assert!(!x2.lines.has_more, "Found an `credit_note.x` event with `lines.has_more=true`. The Stripe events list is lossy - some object lists need to be downloaded outside of the event stream. This is an issue as the download list is always the latest version, where as the event object is a snapshot of data at a point in time (this could lead to the parent credit_note being inconsistent with the child credit note lines). A crash early approach has been taken to prevent possibly invalid SQL query results. A full download will always contain all `credit_note.lines`. Support for applying credit_note update events with >10 lines can be added on request, please contact table.dog.hq@gmail.com. CreditNote.id={}", &x.id);
            write_one::<_, CreditNote>(utx, run_id, &e, x2)
        }
        Obj::Customer(x) => write_one::<_, Customer>(utx, run_id, &e, x),
        Obj::Discount(x) => {
            let mut a = write_one::<_, Discount>(utx, run_id, &e, x);

            match &mut a {
                Action::Write(write_ids) => {
                    if e.type_x.starts_with("customer.discount.") {
                        match (&x.customer, &x.subscription, &x.invoice, &x.invoice_item) {
                            (Some(_), None, None, None) => {
                                // Customer owns discount - no `customer.updated` triggered on changes, manually update `discount` field.
                                write_ids.push(Customer::update_discount_id(utx, run_id, e.type_x.as_str(), &x));
                            }
                            _ => {
                                // Discount owned by non-customer - these have their own update events which updates the `discount`.
                            }
                        }
                    }
                }
                _ => {}
            }

            a
        }
        Obj::Dispute(x) => write_one::<_, Dispute>(utx, run_id, &e, x),
        // RadarEarlyFraudWarning(x) => write_one(utx, run_id,  &e, x),
        Obj::UniPolymorphic70BAFA(x) => {
            // Note: `customer.source.x` events can contain these (not just Source).
            match x {
                UniPolymorphic70BAFA::BankAccount(x) => write_one::<_, BankAccount>(utx, run_id, &e, x),
                UniPolymorphic70BAFA::Card(x) => write_one::<_, Card>(utx, run_id, &e, x),
            }
        }
        // FeeRefund(x) => write_one(utx, run_id,  &e, x),
        // File(x) => write_one(utx, run_id,  &e, x),
        Obj::Invoice(x) => {
            // @see Paper 2021-04-24
            assert!(!x.lines.has_more, "Found an `invoice.x` event with `lines.has_more=true`. The Stripe events list is lossy - some object lists need to be downloaded outside of the event stream. This is an issue as the download list is always the latest version, where as the event object is a snapshot of data at a point in time (this could lead to the parent invoice being inconsistent with the child invoice lines). A crash early approach has been taken to prevent possibly invalid SQL query results. A full download will always contain all `invoice.lines`. Support for applying invoice update events with >10 lines can be added on request, please contact table.dog.hq@gmail.com. Invoice.id={}", &x.id.as_ref().unwrap());
            write_one::<_, Invoice>(utx, run_id, &e, x)
        }
        Obj::InvoiceItem(x) => write_one::<_, Invoiceitem>(utx, run_id, &e, x),
        // IssuingAuthorization(x) => write_one(utx, run_id,  &e, x),
        // IssuingCard(x) => write_one(utx, run_id,  &e, x),
        // IssuingCardholder(x) => write_one(utx, run_id,  &e, x),
        // IssuingDispute(x) => write_one(utx, run_id,  &e, x),
        // IssuingTransaction(x) => write_one(utx, run_id,  &e, x),
        // Mandate(x) => write_one(utx, run_id,  &e, x),
        Obj::Order(x) => write_one::<_, Order>(utx, run_id, &e, x),
        Obj::OrderReturn(x) => write_one::<_, OrderReturn>(utx, run_id, &e, x),
        Obj::PaymentIntent(x) => write_one::<_, PaymentIntent>(utx, run_id, &e, x),
        Obj::PaymentMethod(x) => write_one::<_, PaymentMethod>(utx, run_id, &e, x),
        // Payout(x) => write_one(utx, run_id,  &e, x),
        // Person(x) => write_one(utx, run_id,  &e, x),

        // Prices replace plans (plan writes are mirrored to price list/events; users should query prices instead).
        // Obj::Plan(x) => write_one::<_, Plan>(utx, run_id,  &e, x),

        Obj::Price(x) => write_one::<_, Price>(utx, run_id, &e, x),
        Obj::Product(x) => write_one::<_, Product>(utx, run_id, &e, x),
        Obj::PromotionCode(x) => write_one::<_, PromotionCode>(utx, run_id, &e, x),

        // TransferRecipient(x) => write_one(utx, run_id,  &e, x),
        Obj::Refund(x) => write_one::<_, Refund>(utx, run_id, &e, x),
        // ReportingReportRun(x) => write_one(utx, run_id,  &e, x),
        // ReportingReportType(x) => write_one(utx, run_id,  &e, x),
        // RadarReview(x) => write_one(utx, run_id,  &e, x),
        // ScheduledQueryRun(x) => write_one(utx, run_id,  &e, x),
        Obj::SetupIntent(x) => write_one::<_, SetupIntent>(utx, run_id, &e, x),
        Obj::Sku(x) => write_one::<_, Sku>(utx, run_id, &e, x),
        Obj::Source(x) => write_one::<_, Source>(utx, run_id, &e, x),
        // SourceTransaction(x) => write_one(utx, run_id,  &e, x),
        Obj::Subscription(x) => write_one::<_, Subscription>(utx, run_id, &e, x),
        Obj::SubscriptionSchedule(x) => write_one::<_, SubscriptionSchedule>(utx, run_id, &e, x),
        Obj::TaxId(x) => write_one::<_, TaxId>(utx, run_id, &e, x),
        Obj::TaxRate(x) => write_one::<_, TaxRate>(utx, run_id, &e, x),
        // Topup(x) => write_one(utx, run_id,  &e, x),
        // Transfer(x) => write_one(utx, run_id,  &e, x),
        Obj::UnknownEvent(_) => unreachable!("UnknownEvent matched."),
        _ => {
            unimplemented!("Known event, but not implemented. {}", &e.type_x)
        }
    };

    action
}

fn write_one<A, B>(utx: &mut UniTx, run_id: i64, e: &API::NotificationEvent, a: &A) -> Action where
    A: GetId,
    B: WriteTree<APIType=A> + GetInsertTs {
    /// Detecting events that are older that the downloaded row is not needed because:
    /// - There are only two possibilities:
    ///     - 1. The last applied event is the same data as the download.
    ///     - 2. The last applied event is newer than the download.
    ///
    /// - In both cases, writing all events up to the last one results in the latest data being applied.
    ///     - And when applied inside of a DB tx, *the external world will only see the latest data* (and not intermediate/old data as older events are applied).
    ///
    /// - Questions:
    ///     - What about child types?
    ///         - Because deletes do not cascade, there could be detached child types (payment method) that are written from old events that are no longer reachable from the download API.
    ///
    // if last_run_was_dl {
    //     if let Some(insert_ts) = B::get_insert_ts(&utx, a.get_id().as_str()) {
    //         let e_ts = e.created.to_dt();
    //         if e_ts < insert_ts {
    //             return Action::Skip(Skip::OldData);
    //         }
    //     }
    // }


    let write_ids = if is_delete(&e) {
        B::delete_tree(utx, run_id, &a)
    } else {
        B::upsert_tree(utx, run_id, &a)
    };

    Action::Write(write_ids)
}


/// Deliberately limit the influence of the event type on the apply_events process.
/// - Issue: Stripes event list is inconsistent (esp for parent->child relations and c,u,d lifetime events).
///     - Fix: Just see the event list as "new data for type x" and ignore the event type entirely.
///         - The only time the event type is used is for deletes - these happen rarely, most types can only be created and updated.
///             - They may be detached, but this is not a delete as the metadata can still be updated.
pub fn is_delete(e: &API::NotificationEvent) -> bool {
    let except = vec![
        /// This means "the source is no longer attached to the customer" (NOT "the source has been deleted").
        /// - `payment_method` has a similar/mirrored lifetime, but uses `attached` instead.
        /// - `customer.updated` is fired before this, so the customer object default_source=null is applied.
        "customer.source.deleted",
        /// Docs: `Occurs whenever a customer's subscription ends.`
        /// - Does not make sense to delete the subscription as a admin UI may want to show subscription history.
        /// - This event updates `subscription.status=cancelled`.
        "customer.subscription.deleted",
        /// @see `delete_tree` comment of `Discount`
        "customer.discount.deleted",
        /// Actually delete the tax_id as it is no longer attached to the parent customer making the data not join-able.
        // "customer.tax_id.deleted",
        /// Coupons have a valid=bool which determines if it still can be applied to new customers. Monetary values are immutable.
        "coupon.deleted",
    ];


    if e.type_x.ends_with(".deleted") && !except.contains(&e.type_x.as_str()) {
        if e.type_x.matches(".").count() > 1 {
            #[cfg(test)]
            println!("Note: An `x.y.deleted` event type with many paths may actually semantically mean `detached`. Add it to the exceptions list if this is the case. {:?}", e);
        }

        return true;
    }

    false
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct ApplySummary {
    // When there is one event just set `to`
    pub run_id: i64,
    pub from: Option<EventSummary>,
    pub to: EventSummary,
    pub event_type_count: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct EventSummary {
    pub id: String,
    pub created: String,
    pub created_rel: String,
}

impl From<&API::NotificationEvent> for EventSummary {
    fn from(x: &API::NotificationEvent) -> Self {
        let created = NaiveDateTime::from_timestamp(x.created, 0);
        let now = Utc::now().naive_utc();

        // Sometimes now < created (2021-07-11T18:13:32.868322 < 2021-07-11T18:13:33), E.g. Stripe server rounds current second up but it has not yet passed. This causes `OutOfRangeError` for chrono::Duration.to_std().
        let now = cmp::max(Utc::now().naive_utc().round_subsecs(0), created);

        let mut diff = now.signed_duration_since(created);
        let human_duration = format_duration(diff.to_std().unwrap());

        EventSummary {
            id: x.id.clone(),
            created: x.created.to_iso(),
            created_rel: format!("{} ago", human_duration),
        }
    }
}


impl ApplySummary {
    pub fn from_event_list(run_id: i64, e: &Vec<API::NotificationEvent>) -> Self {
        assert!(e.len() > 0, "Cannot generate summary for 0 events.");

        let mut hm = HashMap::new();
        for x in e {
            *hm.entry(x.type_x.to_string()).or_insert(0) += 1;
        }


        ApplySummary {
            run_id,
            from: if e.len() > 1 {
                Some(e.first().unwrap().into())
            } else { None },
            to: e.last().unwrap().into(),
            event_type_count: hm,
        }
    }
}


// Get all events that do not match the Stripe Client version.
// - Grouped by Stripe version, sorted created ASC.
//
// - If the version does not match, this could cause SQL queries to operate on datasets that are not 100% complete.
//      - A. Data can be moved to different JSON paths, meaning it is not written to SQL.
//      - B. Different Stripe versions set on the account trigger different events. Missing events == missing SQL writes == incorrect queries.
pub fn get_incorrect_versions(e: &Vec<API::NotificationEvent>) -> HashMap<String, Vec<(String, String)>> {
    let mut hm = HashMap::new();
    let target_version = StripeClient::get_api_version();

    for x in e {
        let v = x.api_version.as_ref().unwrap();
        if v != target_version {
            let mut grp = hm.entry(v.clone()).or_insert(vec![]);
            grp.push((x.id.clone(), x.created.to_iso()));
        }
    }

    hm
}

// Issue: users cannot set the default Stripe version as:
// - They use /events for other processes.
// Fix: Eventually add webhook functionality which would enable pining that particular webhooks version (but would require incoming connections, 100% uptime, per account config, no ability to batch apply events in a single transaction in order).
// Fix: Allow flag to ignore Stripe version and potentially have incorrect query results.
fn assert_correct_version(e: &Vec<API::NotificationEvent>) {
    let incorrect_versions = get_incorrect_versions(&e);
    if incorrect_versions.len() > 0 {
        error!("Events NOT applied. Events found that do not match this CLI's Stripe version ({}): {}", StripeClient::get_api_version(), incorrect_versions.to_json());
        error!("Please set your Stripe account default version to {} at https://dashboard.stripe.com/developers and try again.", StripeClient::get_api_version());
        error!("Event versions must match the one used by this CLI to ensure SQL queries are correct and operate on 100% of the dataset. A new Stripe version implies different JSON structures and event types - if these differences are ignored it could result in incomplete query results.");
        error!("You can version-pin existing webhook endpoints at https://dashboard.stripe.com/webhooks.");
        error!("You can version-pin existing Stripe API HTTP clients by using the Stripe-Version: xxxx-xx-xx HTTP header.");
        error!("Events retrieved from `/v1/events` have a version that matches the Stripe account version at the time of event creation.");
        assert_eq!(incorrect_versions.len(), 0);
    }
}

/// @todo/next
/// - `customer.tax_id.created` == `skip.object_type_not_written_to_table`?
pub async fn apply_events(c: &StripeClient, uc: &mut UniCon, events: Option<Vec<API::NotificationEvent>>) {
    info!("Checking for new events.");

    // If last run was a download, limit events to 2 min's prior (in case user upgrades their Stripe version and re-downloads, and old-version events cannot be deleted - no need to wait 30 days for the events to delete, just wait 2 minutes).
    let last_run = TdRun::get_last_run(uc).expect("Cannot apply events without a prior full download.");
    let mut since = None;
    if last_run.is_download() {
        let dl_start = last_run.start_ts.unwrap().dt.timestamp();
        let created_gte = dl_start - (2 * 60);
        // info!("First time applying events. Limiting events to WHERE created >= {} (2 minutes before the first download start).", created_gte.to_iso());
        // Applying events always occurs after the first download so that writes that occur during the download can be applied. This ensures SQL queries are correct and operate on 100% of the dataset (and are not missing items due to different object lists being mutated during download. There is no read transaction support for Stripe's API, but the events stream serializes writes allowing read transaction emulation. This is why the events are applied immediately after the first download). The 2 min limit also users to upgrade their Stripe account version up to the one used by TD without having to delete prior events that are published in an older version.
        since = Some(created_gte)
    }


    // Allow passing in events for testing.
    let e = match events {
        None => get_all_unapplied_events(&c, uc, since).await,
        Some(e) => e
    };

    // Note: at this stage events have been strongly typed OR Serde will stop the process (so Serde may of been able to parse another versions JSON format as the structure changes only slightly version to version).
    assert_correct_version(&e);

    // Only get tx if there are events to apply (Postgres creates a new auto-increment ID for every rolled back tx).
    if e.len() > 0 {
        let mut utx_o = uc.tx_open().unwrap();
        let start = Instant::now();
        let utx = &mut utx_o;

        // Do not hold DB tx open for the duration of the HTTP request (this can take 200ms, when polled once per second this locks the database for 20% of the time).
        let last_run_tx = TdRun::get_last_run_tx(utx).expect("Cannot apply events without a prior full download.");
        if last_run.run_id.unwrap() != last_run_tx.run_id.unwrap() {
            warn!("Ignoring events downloaded. Another process has inserted the same events. start_run_id={}, end_run_id={}", last_run.run_id.unwrap(), last_run_tx.run_id.unwrap());
            return;
        }

        let mut run = TdRun {
            run_id: None,
            // from_api: "stripe".into(),
            r#type: "apply_events".into(),
            start_ts: now_3().into(),
            end_ts: None,
        };

        run.tx_insert_set_pk(utx);
        let run_id = run.run_id.unwrap();

        apply_events_body(utx, run_id, &e);

        run.end_ts = Some(now_3());
        run.tx_update_pk(utx);

        if e.len() > 0 {
            let write_quota_used = TdStripeWrite::get_write_count_excluding_deletes(&mut utx_o, run_id);
            utx_o.tx_close().unwrap();
            let summary = ApplySummary::from_event_list(run_id, &e);
            info!("Applied {} events: {}", &e.len(), summary.to_json());
        } else {
            // Remove this run_id as it has no events; reduce disk usage.
            utx_o.tx_rollback();
        }

        let duration = start.elapsed();
        debug!("DB TX lock held for {:?}", duration);
    }

    TdMetadata::set_heartbeat_now(uc);


    #[cfg(test)]
        {
            // @todo/low ensure this is compiled out in release build.
            // Note: This is ok during testing as a read tx can be emulated.
            //      - Stripe API writes are not occurring during the download.
            let missing = Db::get_missing_owner_all(&uc);
            if missing.len() > 0 {
                dbg!(&missing);
            }
            assert_eq!(missing.len(), 0, "Foreign key constraints violated (note: not native SQL FK constraint).");
        }
}


