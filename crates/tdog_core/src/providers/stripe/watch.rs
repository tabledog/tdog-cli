use std::collections::HashMap;
use std::time::Duration;
use futures::FutureExt;
use chrono::{DateTime, Utc};
use futures::future::{join_all, LocalBoxFuture, AbortHandle, Abortable};
use futures::future::TryFutureExt;
use futures::join;
use futures::stream::{self, Stream, StreamExt, TryStream, TryStreamExt};
use futures::try_join;
use futures_util::pin_mut;
use log::{info, trace, warn};
use serde::{Deserialize, Serialize};
use stripe_client::http::http::{Config, StripeClient, UniErr, StripeAccount};
use stripe_client::types::req_params::{GetBalanceHistory, GetCharges, GetCountrySpecs, GetCoupons, GetCustomers, GetDisputes, GetEvents, GetInvoiceitems, GetInvoices, GetPaymentIntents, GetPaymentMethods, GetPrices, GetProducts, GetPromotionCodes, GetRefunds, GetSetupIntents, GetSubscriptionItems, GetSubscriptions, GetSubscriptionSchedules, GetTaxRates, UniStrStatus3EB683, UniStrTypeBAE85E};
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{
    Address as APIAddress,
    Customer as APICustomer,
    Price as APIPrice,
    Product as APIProduct,
    Source as APISource,
    Subscription as APISubscription,
    SubscriptionItem as APISubscriptionItem,
    SubscriptionSchedule as APISubscriptionSchedule,
    TaxRate as APITaxRate,
};
use tokio::sync::Mutex;
use tokio::time;
use tokio::time::delay_for;
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
use crate::providers::stripe::apply_events::apply_events;
use crate::providers::stripe::dl::{*};
use crate::providers::stripe::schema::{Db, WriteTree};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::balance_transaction::BalanceTransaction;
use crate::providers::stripe::schema::types::coupon::Coupon;
use crate::providers::stripe::schema::types::dispute::Dispute;
use crate::providers::stripe::schema::types::invoice::Invoice;
use crate::providers::stripe::schema::types::invoice_line_item::{InvoiceLineItem, InvoiceLineItemWithParentId};
use crate::providers::stripe::schema::types::invoiceitem::Invoiceitem;
use crate::providers::stripe::schema::types::promotion::PromotionCode;
use crate::providers::stripe::schema::types::refund::Refund;
use crate::providers::stripe::schema::types::setup_intent::SetupIntent;
//use unicon::{UniCon, UniTx};
//use unicon::{
//     Insert,
//     DbStatic,
//     TableStatic,
// };s
use crate::{Stripe, Download};

use super::schema_meta::{*};

use crate::providers::stripe::schema::util::ToJSONString;
use crate::util::is_debug_build;
use serde_json::{Value, Map};
use crate::providers::stripe::queue::Queue;
use std::sync::Arc;
use std::ops::Deref;
use tokio::task::JoinHandle;
use std::env;

impl From<&Stripe> for Config {
    fn from(x: &Stripe) -> Self {
        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert("Authorization".into(), format!("Bearer {}", x.secret_key.clone()).into());

        let mut proxy = None;
        if let Some(x2) = &x.http {
            if let Some(x3) = &x2.proxy {
                if let Some(x4) = &x3.url {
                    proxy = Some(x4.clone());
                }
            }
        }

        // Force using local proxy whilst developing.
        if is_debug_build() {
            proxy = Some("http://localhost:8888".into());
        }

        Config {
            secret_key: x.secret_key.clone(),
            is_test: Config::is_test_secret_key(&x.secret_key),
            base: "https://api.stripe.com".into(),
            headers: Some(hm),
            proxy,

            // Issue: Sometimes when polling /v1/events, after around 20 min the Stripe server will not respond. crate::reqwest defaults to no timeout - wait forever. This prevents new events being applied.
            // Fix: Kill TD process.
            // - In production, TD should be managed by a process scheduler that will restart it for any crashes.
            // - In dev, the user can use docker restart, or write a simple bash loop.
            timeout_ms: Some(40_000),
            retry: true,
            log_requests: true,
        }
    }
}

// let c = Customer {
//     wow: 123
// };
//
// let p = Payment {
//     wow: 123
// };
//
//
// p.insert(&uc);
// p.insert(&uc);
// c.insert(&uc);
// c.insert(&uc);
//
// let diff_types: Vec<&dyn Insert> = vec![&p, &c, &p, &c];
// let same_types = vec![&p, &p, &p];
//
//
// ins_all_concrete(&same_types, &uc);
// ins_all_dyn(&diff_types, &uc);


/// Issue, possible: FK relations can be broken when the underlying data is being mutated.
/// - E.g. Download timeline: (customers, pause, subscriptions).
///     - During `pause` new customers + subs can be created, causing a SQL insert issue as the new sub with no customer will be inserted.
/// Fix, A: Option to remove FK constraints (enable only for low throughput accounts).
/// Fix, B: Download each dep individually if it is missing.
/// Fix, C: Use the incremental download logic.
pub async fn download_all(c: &StripeClient, uc: &mut UniCon, dl: &Download) {
    info!("Download all started.");
    info!("HTTP requests are logged at https://dashboard.stripe.com/test/logs.");
    info!("HTTP requests can also be observed by setting an HTTP proxy via the JSON config.");

    // let c = StripeClient::new(s.into());
    let mut utx = uc.tx_open().unwrap();


    let mut run = TdRun {
        run_id: None,
        // from_api: "stripe".into(),
        r#type: "download".into(),
        start_ts: now_3().into(),
        end_ts: None,
    };

    run.tx_insert_set_pk(&mut utx);
    let run_id = run.run_id.unwrap();


    // @todo/low Handle errors.
    // @todo/low Does the order of these downloads matter for foreign keys?
    // - Is it possible to enforce FK constraints on commit (instead of inside the tx after each statement)?
    // - Should downloads happen from the deepest levels to the root of the tree? What about for a graph of deps (E.g. discounts)?


    // Cue: `mutex.lock()` is like a dynamic/on-demand `&mut x`.
    // - Even though there is a single thread with an event loop that activates/sleeps async functions (so only one function will ever be mutating utx), *a `&mut utx` cannot be given to each async function as the Rust compiler only expects one `&mut` to exist*.
    //      - Fix: Use a mutex, pass a new read only `&mutex` to each async function.
    // let utx_mt = Arc::new(Mutex::new(utx));
    let utx_mt = Mutex::new(utx);


    dl_all(&c, &utx_mt, run_id, dl).await;

    let mut utx = utx_mt.into_inner();

    run.end_ts = Some(now_3());
    run.tx_update_pk(&mut utx);

    let inserts = TdStripeWrite::get_insert_count_by_obj_type(&mut utx);
    let write_quota_used = TdStripeWrite::get_write_count_excluding_deletes(&mut utx, run_id);

    utx.tx_close().unwrap();

    info!("Download all completed.");
    info!("Inserted objects: {}",  inserts.to_json());


    // Note: this check is done after starting from a `dl` SQLite file.
    // - Maybe be an CLI option to debug relations in the future.
    // #[cfg(test)]
    //     {
    //         // Note: This should only apply for test database's where a read-tx can be emulated (by ensuring no writes are happening whilst the download is occurring).
    //         // let missing = Db::get_missing_owner_all(&uc);
    //         // if missing.len() > 0 {
    //         //     dbg!(&missing);
    //         // }
    //         // assert_eq!(missing.len(), 0, "Foreign key constraints violated (note: not native SQL FK constraint).");
    //     }
}


// When a Stripe HTTP 429 occurs, stop starting new requests until all of the current ones have resolved.
pub fn on_429_pause_queue_until_resolved(q_mt_a: Arc<Mutex<Queue>>, c: &StripeClient, exit_on_429: bool) -> AbortHandle {
    let (a_h, a_r) = AbortHandle::new_pair();
    let stats = c.stats.clone();

    let h = tokio::spawn(Abortable::new(async move {
        loop {
            {
                let stats = stats.read().await;
                let mut q = q_mt_a.lock().await;
                if stats.cur_429_reqs_retrying > 0 {
                    if exit_on_429 {
                        error!("The Stripe API responded with HTTP code 429 (too many requests).");
                        error!("Exited the process to avoid continually rate locking the Stripe account (this prevents impacting other systems read/writing to the same Stripe account).");
                        // Note: panic! does not kill main process.
                        std::process::exit(1);
                        // Implicit: Current DB tx fails and is rolled back.
                    }

                    if !q.paused {
                        error!("The Stripe API responded with HTTP code 429 (too many requests).");
                        error!("Ensure you have set the correct request concurrency rate-per-second in the config. You can also contact Stripe to increase your rate limits.");
                        error!("A 429 response indicates that the Stripe account is temporarily locked. This may impact other processes using the Stripe API for the same account.");
                        error!("Waiting until 429 lock is lifted to continue with download.");
                        q.pause();
                    }
                } else {
                    if q.paused {
                        info!("Stripe account HTTP 429 lock has been lifted. Continuing with download.");
                        q.resume();
                    }
                }
            }


            delay_for(Duration::from_millis(1000)).await;
        }
    }, a_r));

    a_h
}

// Issues:
// - When downloading a large account with the CLI set to `info`, no output is seen for a long time.
//      - To the end user, it could seem like the CLI has crashed.
// - 429 responses may go hidden as the CLI will wait and recover from them, but the overall requests/sec will be reduced.
//      - There is a "pendulum swing" between the two states of "fast requests under the 429 limit" and "10x slower request rate due to 429 lock".
// - It is not obvious how quickly the download is progressing, which makes it impossible to determine an ETA for completion.
// - TD aims to download the account as fast as possible.
//      - To compare with itself and other tools,s stats are required.
//
// Fix: Output stats regularly.
// - It should be possible to grep the logs, parse the data and create a chart relatively easily.
pub fn log_stats_every(q_mt_a: Arc<Mutex<Queue>>, c: &StripeClient) -> AbortHandle {
    let (a_h, a_r) = AbortHandle::new_pair();
    let stats = c.stats.clone();

    let h = tokio::spawn(Abortable::new(async move {
        loop {
            let ms = 30_000;
            delay_for(Duration::from_millis(ms)).await;

            let x = {
                let queue_len = {
                    let mut q = q_mt_a.lock().await;
                    q.len()
                };

                let mut stats = stats.write().await;

                let mut total = stats.req_log.len() as u32;
                let mut total_ok = 0;

                let mut total_ms = 0;
                let mut code = HashMap::new();
                for line in &stats.req_log {
                    let code_str = match line.code {
                        None => "net_error".to_string(),
                        Some(x) => {
                            if x == 200 {
                                total_ok += 1;
                                total_ms += line.duration_ms;
                            }

                            format!("{}", x)
                        }
                    };
                    let c = code.entry(code_str.clone()).or_insert(0);
                    *c += 1;
                }

                stats.req_log = vec![];

                ReqSummary {
                    complete: SummaryInterval {
                        total,
                        code,
                        avg_req_ms: if total_ok > 0 { total_ms / total_ok } else { 0 },
                        avg_reqs_per_sec: if total_ok > 0 { total_ok / (ms / 1000) as u32 } else { 0 },
                    },
                    // running: stats.running,
                    queue_len,
                }
            };

            info!("HTTP request summary, last 30 seconds:  {}", x.to_json());
        }
    }, a_r));

    a_h
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ReqSummary {
    pub complete: SummaryInterval,

    // This is confusing as it can be 0 (when the interval is at the end of the second when the requests have completed).
    // pub running: u32,
    pub queue_len: u64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct SummaryInterval {
    pub total: u32,
    pub code: HashMap<String, u32>,
    pub avg_req_ms: u32,
    pub avg_reqs_per_sec: u32,
}


pub async fn dl_all(c: &StripeClient, utx_mt: &Mutex<UniTx<'_>>, run_id: i64, dl: &Download) {
    let from_stripe = dl.get_stripe_from();
    let exit_on_429 = from_stripe.exit_on_429;
    let max_requests_per_second = match from_stripe.max_requests_per_second {
        None => {
            // Limits are 25 for test, 100 for live (but have hit 429 in dev at much lower concurrent requests).
            if c.config.is_test {
                10
            } else {
                // See comment on `fn tx_insert_set_pk_log_write`.
                // - (1ms per DB (insert + log) * 100 list items * 10 concurrent list loops) = Around 1 second of blocking calls per second.
                //      - This will cause HTTP requests to timeout due to not being processed by the blocked event loop.
                20
            }
        }
        Some(x) => x
    };

    info!("Using `max_requests_per_second`={}", max_requests_per_second);
    // dbg!(env::var("RUST_MIN_STACK"));

    let q_mt_a = Arc::new(Mutex::new(Queue::new(max_requests_per_second)));

    let a_queue = Queue::run_scheduler(q_mt_a.clone());
    let a_on_429_pause_queue = on_429_pause_queue_until_resolved(q_mt_a.clone(), c, exit_on_429);
    let a_log_stats = log_stats_every(q_mt_a.clone(), c);


    // @todo/low Handle errors.
    // @todo/low Does the order of these downloads matter for foreign keys?
    // - Is it possible to enforce FK constraints on commit (instead of inside the tx after each statement)?
    // - Should downloads happen from the deepest levels to the root of the tree? What about for a graph of deps (E.g. discounts)?

    // @todo/high Issue "Segmentation fault/Stack overflow".
    // - Without `boxed_local()` on each future, a segmentation fault occurs.
    //      - Sometimes a stack overflow also occurs.
    //      - I think these two are related.
    //      - Does not happen with `cargo --release`.
    //      - Possible causes:
    //          - Serde's generated functions/Stripes optionally expandable types.
    //          - Incorrect `unsafe` of a library (Tokio's async locking types).
    //          - SQLite, using it via Mutex/async functions on the same thread.


    // A
    let f1 = dl_customers(c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f2 = dl_payment_intents(&c, &q_mt_a, &utx_mt, run_id).boxed_local();

    let f3 = dl_setup_intents(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f4 = dl_refunds(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f5 = dl_charges(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f6 = dl_subscriptions(&c, &q_mt_a, &utx_mt, run_id).boxed_local();

    // B
    let f7 = dl_products(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f8 = dl_prices(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f9 = dl_skus(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f10 = dl_orders(&c, &q_mt_a, &utx_mt, run_id).boxed_local();

    // dl_order_returns(&c, &utx, run_id).await;

    // D
    let f11 = dl_invoices(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f12 = dl_invoiceitems(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f13 = dl_credit_notes(&c, &q_mt_a, &utx_mt, run_id).boxed_local();

    // Z
    // Issue: tax rate is a parent of (sub, inv, inv line item, inv items), but the list could be downloaded before, then a new tax rate created (which is missing), and then the sub/inv list with a link to the missing tax_rate.
    // @todo/low Fix, A: apply events immediately after dl to avoid ordering issues with dl?
    // - Why do the events from `28 days ago to now` have to be applied? Would a 2 minute window make a difference? for webhook replacements?

    // Z
    let f14 = dl_coupons(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f15 = dl_promotion_codes(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f16 = dl_tax_rates(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f17 = dl_disputes(&c, &q_mt_a, &utx_mt, run_id).boxed_local();
    let f18 = dl_balance_transactions(&c, &q_mt_a, &utx_mt, run_id).boxed_local();

    // dl_sessions(&c, &utx, run_id)

    join!(
        // A
        f1,
        f2,
        f3,
        f4,
        f5,
        f6,

        // B
        f7,
        f8,
        f9,
        f10,

        // D
        f11,
        f12,
        f13,

        // Z
        f14,
        f15,
        f16,
        f17,
        f18
    );


    // Stop related Tokio background tasks that loop.
    a_on_429_pause_queue.abort();
    a_queue.abort();
    a_log_stats.abort();
}

// @todo/med Issue: A Stripe event stream can be a subset of all possible events. If the type of events subscribed to is changed for a given API key, new events will start to be published against an incomplete DB which would lead to incorrect queries (as some of the object types are missing due being created between (download, x, apply_partial_events)).
// - Fix: Detect API key subscription change, drop DB and re-download data.
static DL_NOT_NEEDED_MSG: &str = "No per-object downloads needed: The DB is less than 30 days old so can be brought up to date by applying new events since the last apply_events process ran.";

// Represents the account (and the dataset) of the Stripe Account accessible for the currently provided Stripe secret key.
pub struct StripeAccountOrig {
    pub id: String,
    pub is_test: bool,
    pub account: Map<String, Value>,
}


fn log_using_sa(sa: &StripeAccount) {
    let live = if sa.is_test { "test" } else { "live" };

    let url = if sa.is_test {
        format!("https://dashboard.stripe.com/{}/test", &sa.id)
    } else {
        format!("https://dashboard.stripe.com/{}", &sa.id)
    };

    info!("Using Stripe account {} ({})", &url, live);
}

pub async fn once(sc: &StripeClient, uc: &mut UniCon, dl: &Download) {
    let tx = Some(1);


    let sa = sc.stripe_account.as_ref().unwrap();

    // @todo/low Issue: Create a tx here to prevent multiple TD processes from interacting (when more than one TD process is run accidentally it should not corrupt the DB or affect dependant processes query results).
    // - Not too much of an issue as each fn starts its own tx.
    //      - Conflicts should cause the tx to fail (E.g. unique constraints for dl/apply_events, table/index already exists for create/drop tables).
    create_schema_if_not_exists_and_log::<Db>(uc, &sa);
    log_using_sa(&sa);


    if let Some(possible) = TdRun::is_apply_events_possible(uc) {
        if possible {
            info!("{}", DL_NOT_NEEDED_MSG);
            apply_events(sc, uc, None).await;
            return;
        }

        drop_all_and_recreate_tables(uc);
    }
    /// else {Fresh empty DB}.


    download_all(sc, uc, dl).await;

    if dl.options.apply_events_after_one_shot_dl {
        apply_events(sc, uc, None).await;
    }
}

pub async fn poll(sc: &StripeClient, mut uc: &mut UniCon, poll_freq_ms: u64, dl: &Download) {
    let tx = Some(1);

    let sa = sc.stripe_account.as_ref().unwrap();
    create_schema_if_not_exists_and_log::<Db>(uc, &sa);
    log_using_sa(&sa);

    if let Some(possible) = TdRun::is_apply_events_possible(uc) {
        if possible {
            info!("{}", DL_NOT_NEEDED_MSG);
            drop(tx);
            poll_apply_events(sc, uc, poll_freq_ms).await;
            return;
        }

        drop_all_and_recreate_tables(uc);
    }
    /// else {Fresh empty DB}.

    download_all(sc, uc, dl).await;
    drop(tx);
    poll_apply_events(sc, uc, poll_freq_ms).await;
}


/// @todo/med Possible issue: If the event loop is blocked, this keeps adding events into the input queue and they all run at once?
/// - Or
///     - Logging not writing to stdout at the correct time.
///     - Blocking on waiting for tx. Timeout?
async fn poll_apply_events(sc: &StripeClient, uc: &mut UniCon, poll_freq_ms: u64) {
    let d = Duration::from_millis(poll_freq_ms);
    let mut interval_day = time::interval(d);

    info!("Polling for new events every {:?}.", d);

    loop {
        let now = interval_day.tick().await;
        apply_events(sc, uc, None).await;
        // return;
    }
}


async fn drop_all_and_recreate_tables(uc: &mut UniCon) {
    info!("It is not possible to incrementally apply events as the last run was longer than 28 days ago. Stripe only stores the last 30 days of events.");

    // @todo/low Store SQL drop statements to allow a newer version to apply the older versions drop table (in case of adding/removing tables).
    // - Not needed as CLI expects user to drop the old version when using a newer CLI.

    error!("Please drop the schema and try again. Tables were not automatically dropped as you may require historical records of Stripe events (the API is limited to the last 30 days).");
    panic!();
    // Db::drop_all(uc);
    // create_schema_if_not_exists_and_log::<Db>(uc);
}


// Creates schema (if the engine supports it) and tables if they do not exist.
fn create_schema_if_not_exists_and_log<T: DbStatic>(uc: &mut UniCon, sa: &StripeAccount) {
    let (created_all, target_schema) = uc.ensure_schema_and_tables_exist_and_writable::<T>();

    if created_all {
        if let Some(x) = &target_schema.schema {
            info!("Created schema `{}`.", &x.name);
        }

        info!("Created tables: {:?}", target_schema.tables.iter().map(|x| &x.name).collect::<Vec<&String>>());
        TdMetadata::insert_cli_and_stripe_versions(uc, sa);
    } else {
        if let Some(x) = &target_schema.schema {
            info!("Schema already exists: `{}`.", &x.name);
        }

        info!("Tables already exist: {:?}", target_schema.tables.iter().map(|x| &x.name).collect::<Vec<&String>>());
        let mut x = TdMetadata::check_cli_and_stripe_versions_match(uc, sa);
        x.update_stripe_account(uc, sa);
    }
}


