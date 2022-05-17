use std::collections::HashMap;
use std::time::Duration;

use chrono::{DateTime, Utc};
use futures::future::join_all;
use futures::future::TryFutureExt;
use futures::join;
use futures::stream::{self, Stream, StreamExt, TryStream, TryStreamExt, FuturesOrdered};
use futures::try_join;
use futures_util::pin_mut;
use log::{info, trace, warn};
use stripe_client::http::http::{Config, StripeClient, UniErr};
use stripe_client::types::req_params::{GetBalanceHistory, GetCharges, GetCheckoutSessions, GetCountrySpecs, GetCoupons, GetCreditNotes, GetCustomers, GetCustomersCustomerSources, GetDisputes, GetEvents, GetInvoiceitems, GetInvoices, GetOrderReturns, GetOrders, GetPaymentIntents, GetPaymentMethods, GetPrices, GetProducts, GetPromotionCodes, GetRefunds, GetSetupIntents, GetSkus, GetSubscriptionItems, GetSubscriptions, GetSubscriptionSchedules, GetTaxRates, UniStrStatus3EB683, UniStrTypeBAE85E};
use stripe_client::types::responses::{ApmsSourcesSourceListF0771E, UniPolymorphic646C3F};
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
use crate::providers::stripe::schema::{Db, WriteTree};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::balance_transaction::BalanceTransaction;
use crate::providers::stripe::schema::types::bank_account::BankAccount;
use crate::providers::stripe::schema::types::card::Card;
use crate::providers::stripe::schema::types::coupon::Coupon;
use crate::providers::stripe::schema::types::credit_note::CreditNote;
use crate::providers::stripe::schema::types::credit_note_line_item::CreditNoteLineItemWithParentId;
use crate::providers::stripe::schema::types::dispute::Dispute;
use crate::providers::stripe::schema::types::invoice::Invoice;
use crate::providers::stripe::schema::types::invoice_line_item::{InvoiceLineItem, InvoiceLineItemWithParentId};
use crate::providers::stripe::schema::types::invoiceitem::Invoiceitem;
use crate::providers::stripe::schema::types::order::Order;
use crate::providers::stripe::schema::types::order_return::OrderReturn;
use crate::providers::stripe::schema::types::promotion::PromotionCode;
use crate::providers::stripe::schema::types::refund::Refund;
use crate::providers::stripe::schema::types::session::Session;
use crate::providers::stripe::schema::types::setup_intent::SetupIntent;
use crate::providers::stripe::schema::types::sku::Sku;
use crate::Stripe;

use super::schema_meta::{*};
use crate::providers::stripe::rate_limit::{RateLimit};
use crate::providers::stripe::queue::Queue;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

pub async fn dl_events(c: &StripeClient, mut uc: &mut UniCon) {
    let p = Some(GetEvents {
        type_x: None,
        created: None,
        delivery_success: None,
        ending_before: None,
        expand: None,
        limit: Some(100),
        starting_after: None,
        types: None,
    });


    let mut st = c.v1_events_get_st(&p.unwrap());
    pin_mut!(st);
    while let Some(val) = st.next().await {
        for i in val.unwrap().data {
            let mut i2: NotificationEvent = (&i).into();
            i2.insert_set_pk(&mut uc);
        }
    }
}


// fn insert_customers(utx: &UniTx, run_id: i64, data: Vec<APICustomer>) {
//     for c in data {
//         let mut c2: Customer = (&c).into();
//
//         // c2.insert_set_pk(&uc);
//         c2.tx_insert_set_pk_log_write(utx, run_id);
//
//         if let Some(sources) = &c.sources {
//             for poly in &sources.data {
//                 match poly {
//                     UniPolymorphic646C3F::Source(s) => {
//                         let mut rw: Source = s.into();
//                         // rw.insert_set_pk(&uc);
//                         rw.tx_insert_set_pk_log_write(utx, run_id);
//                     }
//                     _ => {
//                         // @todo/next Card, Bank - list via their API points.
//                         // @todo/low Alipay, Bitcoin?
//                         info!("Note: Only Source objects supported (missing Alipay, Bank, Card etc).");
//                         continue;
//                     }
//                 }
//             }
//         }
//     }
// }


/// Note: subscription_item = has_dl_list && !has_direct_events
/// @todo/next ensure all discount containing objects (subs, invoice, invoiceitems, customer) call insert_tree (and not tx_insert_set_pk_log_write).
// fn insert_subscriptions(utx: &UniTx, run_id: i64, data: Vec<APISubscription>) {
//
//
//
//     for s in data {
//         let mut s2: Subscription = (&s).into();
//         // s2.insert_set_pk(&uc);
//         s2.tx_insert_set_pk_log_write(utx, run_id);
//
//         for s3 in &mut s2.items_skip {
//             // dbg!("Inserting sub item with price id {}", &s3.price);
//             // s3.insert_set_pk(&uc);
//             s3.tx_insert_set_pk_log_write(utx, run_id);
//         }
//     }
// }


pub async fn dl_customers(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let p = Some(GetCustomers {
        created: None,
        email: None,
        ending_before: None,
        expand: Some(vec![
            // "data.default_source".into()

            // Note: There is no list for `sources`. PaymentIntent replaces `Sources`.
            // - Access `Source` from `Customer` object.
            "data.sources".into(),
            "data.tax_ids".into(), // (!has_dl_list (is per customer) && ~has_direct_event (on attach to customer?))

            // Always expanded.
            // "data.discount".into()
        ]),
        limit: Some(100),
        starting_after: None,
    });

    let mut all_customers: Vec<String> = vec![];
    let mut st = c.v1_customers_get_st(&p.unwrap());
    pin_mut!(st);


    while let Some(val) = st.next().q_high(q_mt_a).await {
        let res = val.unwrap();
        let utx = &mut (utx_mt.lock().await);

        for c2 in res.data {
            all_customers.push(c2.id.clone());
            Customer::insert_tree(utx, run_id, &c2);

            assert!(&c2.sources.is_some(), "Expected Customer.sources to be expanded.");
            if let Some(x) = &c2.sources {
                if x.has_more {
                    let p = GetCustomersCustomerSources {
                        object: None,
                        ending_before: None,
                        expand: None,
                        limit: Some(100),
                        starting_after: None,
                    };

                    // Note: `stream` fn not generated as this returns a polymorphic type that does not currently have a `get_id` trait for the ending_before param.
                    // - Ignore for now as its unlikely a single customer has >100 sources, and sources is being replaced by the intents API.
                    let x: ApmsSourcesSourceListF0771E = (c.v1_customers_x_sources_get(c2.id.clone(), &p.into()).await).unwrap();
                    assert!(!x.has_more, "Customer.id={} has more than 100 sources, pagination not currently supported.", &c2.id);

                    for x2 in &x.data {
                        match x2 {
                            UniPolymorphic646C3F::Source(x3) => {
                                Source::upsert_tree(utx, run_id, &x3);
                            }
                            UniPolymorphic646C3F::Card(x3) => {
                                Card::upsert_tree(utx, run_id, &x3);
                            }
                            UniPolymorphic646C3F::BankAccount(x3) => {
                                BankAccount::upsert_tree(utx, run_id, &x3);
                            }

                            UniPolymorphic646C3F::AlipayAccount(_) |
                            UniPolymorphic646C3F::BitcoinReceiver(_) => {
                                unreachable!("Listing of customer sources >10 contains a type that is not written to the SQL store (probably a Alipay or Bitcoin). Customer.id={}. This is currently a program exit to prevent invalid queries. These types may be written in the future, or a CLI option to acknowledge the missing writes will be added. Contact TD if you need to query these types.", &c2.id);
                            }
                        }
                    }
                }
            }
        }
    }




    // @todo/low Issue: If `all_customers` is 1M+ items, this takes a lot of RAM. Fix: Use a paginated SQL query to read back these ID's instead of storing in RAM.
    for batch in all_customers.chunks(100) {
        let mut all_payment_methods = vec![];
        for c2 in batch {
            all_payment_methods.push(dl_one_customer_payment_methods(&c, &q_mt_a, &utx_mt, run_id, c2.clone()));
        }
        join_all(all_payment_methods).await;
    }
}


pub async fn dl_coupons(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let p = Some(GetCoupons {
        created: None,
        ending_before: None,
        expand: None,
        limit: Some(100),
        starting_after: None,
    });


    let mut st = c.v1_coupons_get_st(&p.unwrap());
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let res = val.unwrap();
        let utx = &mut (utx_mt.lock().await);
        for c in res.data {
            // Note: When a coupon is "deleted", valid=false.
            //      - These cannot be listed via the API, but are still listed on (customer, sub, invoice, invoiceitem) via expand=discount, discount.coupon.
            Coupon::insert_tree(utx, run_id, &c);
        }
    }
}

pub async fn dl_promotion_codes(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let p = Some(GetPromotionCodes {
        created: None,
        active: None,
        code: None,
        coupon: None,
        customer: None,
        ending_before: None,
        expand: None,
        limit: Some(100),
        starting_after: None,
    });


    let mut st = c.v1_promotion_codes_get_st(&p.unwrap());
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let res = val.unwrap();
        let utx = &mut (utx_mt.lock().await);
        for c in res.data {
            // Note: includes p.active = false.
            PromotionCode::insert_tree(utx, run_id, &c);
        }
    }
}

// Note: there is a difference between `invoiceitems` and `invoice_lines`, *they are different types but have 90% similar data*.
// - https://stripe.com/docs/api/invoices/invoice_lines (attached to invoice, has tax, discounts etc)
// - https://stripe.com/docs/api/invoiceitems/list (attach to upcoming invoice)
pub async fn dl_invoice_lines(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64, invoice_id: &str) {
    let p = GetCountrySpecs {
        ending_before: None,
        expand: None,

        // Assumption: Not needed, as expand=discount set on invoiceitems (which own the discount); The discount id should be inserted via that dl/event path.
        // expand: Some(vec![
        //     "data.discounts".into()
        // ]),

        limit: None,
        starting_after: None,
    };

    let mut st = c.v1_invoices_x_lines_get_st(invoice_id.to_string(), &p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let res = val.unwrap();
        let utx = &mut (utx_mt.lock().await);
        for x in res.data {
            // Upsert needed as parent invoice contains first 10 lines (and will insert them).
            // InvoiceLineItem::upsert_tree(utx, run_id, &x);

            let x2 = InvoiceLineItemWithParentId {
                parent: invoice_id.to_string(),
                data: &x,
            };
            InvoiceLineItemWithParentId::upsert_tree(utx, run_id, &x2);
        }
    }
}

pub async fn dl_invoices(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let p = Some(GetInvoices {
        created: None,
        due_date: None,
        collection_method: None,
        customer: None,
        ending_before: None,
        expand: Some(vec![
            "data.discounts".into(),
            "data.default_payment_method".into(),
        ]),
        limit: Some(100),
        starting_after: None,
        status: None,
        subscription: None,
    });


    let mut st = c.v1_invoices_get_st(&p.unwrap());
    pin_mut!(st);
    let mut lines_has_more = vec![];
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let res = val.unwrap();
        let utx = &mut (utx_mt.lock().await);
        for x in res.data {
            Invoice::insert_tree(utx, run_id, &x);

            /// When lines > 10, after the download completes, all items are inserted.
            /// - If there are any invoice.x events fired with lines > 10, the TD process will crash early.
            /// - Eventually this will be handled with one of:
            ///     - A. `option.ignore_events_lines_gt_10` - do not crash, and do not dl extra items; user understands queries may be incorrect, and can determine this on a per invoice basis by `invoice.lines_newest_10.length===10`
            ///     - B. Inline download - when processing events, download the extra items.
            ///         - This is put off as:
            ///             - Guess: less than 30% of users have invoices with >10 items.
            ///             - It makes testing much harder.
            ///                 - It requires mocking the Stripe server and responses.
            ///                 - There are no read tx's with Stripes API, so the time gap between an `invoice.x` event, and downloading all the lines could contain API writes.
            ///                     - Item downloads are always HEAD/latest, where as the invoice.x event is a snapshot when the event was fired.
            ///                         - Processing the parent/children separately means the sum(parent) and sum(child) could be different, producing invalid SQL query results.
            ///                         - Fix: lookahead in the event list to see if there are any more invoice.x events, if none, dl items, check again, and only commit when certain the items list is the latest up to date version.
            if x.lines.has_more {
                lines_has_more.push(x.id.unwrap());
            }
        }
    }

    for x in lines_has_more {
        dl_invoice_lines(&c, q_mt_a, utx_mt, run_id, x.as_str()).await;
    }


    // @todo/med Augment `apply_events` with `x.updated` events for all parent types containing lists with has_more=true.
    // - Only when applying events up until `now` (not when applying old, locally cached events for testing).
    // - Delete all items connected to parent, re-insert them.
    // Issue: The gap between a parents event, and when its children are downloaded an inserted can be an issue.
    // - E.g. invoice items contribute to a sum on the parent, and if the process (upsert from event, wait 10s, dl_children and insert), then in that 10s gap the parent sum could be incorrect, and would need re-downloading.
    //      - General issue: no read transaction when interacting with the Stripe server (A: dl invoice, C: dl child items), at point B more child items could be added than is represented in A.sum.


    // @todo/low Create a Stripe issue, ask them to commit to a non-lossy event stream (missing out data and requiring a direct download).
    // - This prevents converting a stream to complete database (because when has_more=true, there is missing data).
    // - And prevents creating testing without having to use a stateful Stripe server.
}


pub async fn dl_invoiceitems(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let p = Some(GetInvoiceitems {
        created: None,
        customer: None,
        ending_before: None,
        expand: Some(vec![
            "data.discounts".into()
        ]),
        invoice: None,
        limit: Some(100),
        pending: None,
        starting_after: None,
    });


    let mut st = c.v1_invoiceitems_get_st(&p.unwrap());
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let res = val.unwrap();
        let utx = &mut (utx_mt.lock().await);
        for x in res.data {
            Invoiceitem::insert_tree(utx, run_id, &x);
        }
    }
}


pub async fn dl_credit_notes(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let p = GetCreditNotes {
        customer: None,
        ending_before: None,
        expand: None,
        invoice: None,
        limit: Some(100),
        starting_after: None,
    };


    let mut st = c.v1_credit_notes_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let res = val.unwrap();
        for x in res.data {
            {
                let utx = &mut (utx_mt.lock().await);
                CreditNote::insert_tree(utx, run_id, &x);
            }


            if x.lines.has_more {
                dl_credit_note_line_items(&c, q_mt_a, utx_mt, run_id, x.id.as_str()).await;
            }
        }
    }
}

pub async fn dl_credit_note_line_items(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64, credit_note_id: &str) {
    let p = GetCountrySpecs {
        ending_before: None,
        expand: None,
        limit: Some(100),
        starting_after: None,
    };


    let mut st = c.v1_credit_notes_x_lines_get_st(credit_note_id.to_string(), &p.into());
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let res = val.unwrap();
        let utx = &mut (utx_mt.lock().await);
        for x in res.data {
            // Upsert needed as parent credit_note contains first 10 lines (and will insert them).

            let x2 = CreditNoteLineItemWithParentId {
                parent: credit_note_id.to_string(),
                data: &x,
            };
            CreditNoteLineItemWithParentId::upsert_tree(utx, run_id, &x2);
        }
    }
}


pub async fn dl_one_customer_payment_methods(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64, cid: String) {
    // Download PaymentMethods
    // PaymentMethods can be attached to a customer, and then used in many (Setup|Payment)Intent
    // - Issue: They can only be seen by 1 request per (custId, type).
    //      - If these are updated, the apply_events code will try to update a non-existent row (if they are not downloaded).

    // @todo/low (Setup|Payment)Intent PaymentMethod where customer==null should insert into PaymentMethod's as this loop would have missed those PaymentMethods.
    //      - Can these be updated with events (if they are not attached to customers)?

    let payment_method_types = vec![
        // Global
        UniStrTypeBAE85E::Card,

        // (6 EU countries)
        UniStrTypeBAE85E::SepaDebit,

        // (EU countries)
        UniStrTypeBAE85E::Sofort,

        // (UK only)
        UniStrTypeBAE85E::BacsDebit,
        // @todo/low Provide CLI option to give types to iterate over, to reduce the number of requests.
        // Rest are regional
    ];

    let mut all = vec![];
    for type_x in &payment_method_types {
        let type_x_2 = type_x.clone();
        let cid_2 = cid.clone();

        let one = async {
            let p = GetPaymentMethods {
                type_x: type_x_2,
                customer: cid_2,
                ending_before: None,
                expand: None,
                limit: Some(100),
                starting_after: None,
            };

            let list = c.v1_payment_methods_get(&p).q_low(&q_mt_a).await.unwrap();

            // @todo/low use stream in case of a customer having > 100 methods.
            assert_eq!(list.has_more, false);

            for pm in list.data {
                // This is an upsert as (inv, sub, etc) expand=payment_method, so they could insert it first.
                let mut utx = &mut (utx_mt.lock().await);
                PaymentMethod::upsert_tree(utx, run_id, &pm);
            }
        };

        all.push(one);
    }

    futures::future::join_all(all).await;
}


pub async fn dl_prices(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p_price = GetPrices {
        type_x: None,
        created: None,
        active: None,
        currency: None,
        ending_before: None,
        expand: Some(vec![
            "data.tiers".into(),
        ]),
        limit: Some(100),
        lookup_keys: None,
        product: None,
        recurring: None,
        starting_after: None,
    };

    let mut st = c.v1_prices_get_st(&p_price);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            Price::insert_tree(utx, run_id, &x);
        }
    }

    // @todo/next Issue: This returns an empty list - archived prices are still used in subscription items, causing the FK relation to fail on insert.
    // @see https://dashboard.stripe.com/test/prices/price_1I8BAWBjw9m35HdrTLZ0oAHU
    p_price.active = Some(false);
    let mut st = c.v1_prices_get_st(&p_price);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            Price::insert_tree(utx, run_id, &x);
        }
    }
}

pub async fn dl_subscriptions(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let subs = async {
        let p_subs = GetSubscriptions {
            created: None,
            current_period_end: None,
            current_period_start: None,
            collection_method: None,
            customer: None,
            ending_before: None,
            expand: Some(vec![
                // Note: direct sub items list requires a sub id, expand instead:
                "data.items".into(),
                "data.default_payment_method".into(),

                // Expanded by default (never a string)
                // "data.discount".into(),
            ]),
            limit: Some(100),
            price: None,
            starting_after: None,
            status: Some(UniStrStatus3EB683::All),
        };

        let mut st = c.v1_subscriptions_get_st(&p_subs);
        pin_mut!(st);
        while let Some(val) = st.next().q_high(q_mt_a).await {
            let res = val.unwrap();
            let utx = &mut (utx_mt.lock().await);
            for x in res.data {
                Subscription::insert_tree(utx, run_id, &x);
                // Note: no need to download subscription_items as has_more is always false.
                // - Sub items limited to 20, are always included in both dl and events.
            }
        }
    };

    let schedules = async {
        let p_sched = GetSubscriptionSchedules {
            canceled_at: None,
            completed_at: None,
            created: None,
            released_at: None,
            customer: None,
            ending_before: None,
            expand: None,
            limit: Some(100),
            scheduled: None,
            starting_after: None,
        };

        let mut st = c.v1_subscription_schedules_get_st(&p_sched);
        pin_mut!(st);
        while let Some(val) = st.next().q_high(q_mt_a).await {
            let res = val.unwrap();
            let utx = &mut (utx_mt.lock().await);
            for x in res.data {
                SubscriptionSchedule::insert_tree(utx, run_id, &x);
            }
        }
    };

    join!(
        subs,
        schedules
    );
}

pub async fn dl_tax_rates(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p = GetTaxRates {
        created: None,
        active: None,
        ending_before: None,
        expand: None,
        inclusive: None,
        limit: Some(100),
        starting_after: None,
    };

    let mut st = c.v1_tax_rates_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            TaxRate::insert_tree(utx, run_id, &x);
        }
    }

    // Note: `active=None` returns both active/inactive.
    // p.active = Some(false);
    // let mut st = c.v1_tax_rates_get_st(&p);
    // pin_mut!(st);
    // while let Some(val) = st.next().await {
    //     insert_tax_rates(&uc, val.unwrap().data);
    // }
}


pub async fn dl_products(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p = GetProducts {
        created: None,
        active: None,
        ending_before: None,
        expand: None,
        ids: None,
        limit: Some(100),
        shippable: None,
        starting_after: None,
        url: None,
    };

    let mut st = c.v1_products_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            Product::insert_tree(utx, run_id, &x);
        }
    }
}


pub async fn dl_payment_intents(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p = GetPaymentIntents {
        created: None,
        customer: None,
        ending_before: None,
        expand: Some(vec![
            "data.payment_method".into(),
        ]),
        limit: Some(100),
        starting_after: None,
    };

    let mut st = c.v1_payment_intents_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            PaymentIntent::insert_tree(utx, run_id, &x);
        }
    }
}


pub async fn dl_setup_intents(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p = GetSetupIntents {
        created: None,
        customer: None,
        ending_before: None,
        expand: Some(vec![
            "data.payment_method".into(),
        ]),
        limit: Some(100),
        payment_method: None,
        starting_after: None,
    };

    let mut st = c.v1_setup_intents_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            SetupIntent::insert_tree(utx, run_id, &x);
        }
    }
}


pub async fn dl_charges(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p = GetCharges {
        created: None,
        customer: None,
        ending_before: None,
        expand: None,
        limit: Some(100),
        payment_intent: None,
        starting_after: None,
        transfer_group: None,
    };

    let mut st = c.v1_charges_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            Charge::insert_tree(utx, run_id, &x);
        }
    }
}

pub async fn dl_refunds(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p = GetRefunds {
        created: None,
        charge: None,
        ending_before: None,
        expand: None,
        limit: Some(100),
        payment_intent: None,
        starting_after: None,
    };

    let mut st = c.v1_refunds_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            // Note: Refunds exist as children on (OrderReturn, TransferReversal, CreditNote), so can be inserted already from any of these depending on download order. (This can be an insert_tree if refunds are downloaded first).
            Refund::upsert_tree(utx, run_id, &x);
        }
    }
}

pub async fn dl_disputes(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p = GetDisputes {
        created: None,
        charge: None,
        ending_before: None,
        expand: None,
        limit: Some(100),
        payment_intent: None,
        starting_after: None,
    };

    let mut st = c.v1_disputes_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            Dispute::insert_tree(utx, run_id, &x);
        }
    }
}

/// These are not kept up to date with apply_events as there are no events for balance transactions (only other types that may contain balance transactions).
/// - Download them anyway in case users are querying only the post-download DB.
///     - Assumption: Users will realise this is not kept up to date at dev time.
///
/// Note: Balances are different from BalanceTransactions.
/// - BalanceTransactions do not have events so cannot be kept up to date.
///     - The `balance.available` is just a "current balance" event?
pub async fn dl_balance_transactions(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p = GetBalanceHistory {
        type_x: None,
        available_on: None,
        created: None,
        currency: None,
        ending_before: None,
        expand: None,
        limit: Some(100),
        payout: None,
        source: None,
        starting_after: None,
    };

    let mut st = c.v1_balance_transactions_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {
            BalanceTransaction::insert_tree(utx, run_id, &x);
        }
    }
}


pub async fn dl_sessions(c: &StripeClient, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let mut p = GetCheckoutSessions {
        ending_before: None,
        expand: None, // line_items can be expanded, but its always missing from events.
        limit: Some(100),
        payment_intent: None,
        starting_after: None,
        subscription: None,
    };

    let mut st = c.v1_checkout_sessions_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().await {
        let utx = &mut (utx_mt.lock().await);
        for x in val.unwrap().data {

            // @todo/low Potentially filter both download and events for payment_status=paid, as these can be kept up to date with events (unpaid cannot as there is no create event).

            Session::insert_tree(utx, run_id, &x);

            // Note: `line_items` ignored, see `Session.line_items` comment.
            //      - 0 items are in events which would result in a panic until downloads are implemented at the end of apply events for missing child items (the same as credit note items and invoice line items).
            //      - The `completed` event converts session items into invoice items which can be queried.
            // @todo/med Potential issue: `payment_status=unpaid` are only downloaded; there is no create event.
            // - This could result in potential invalid queries for users who assume applying the events will get them all unpaid checkout sessions.
            //      - I do not think this will be an issue as the checkout sessions seem to be temporary state until a payment is complete.

            // Fix: Get the starting_after of the last `unpaid`, download all sessions at the end of the apply events process, write them.
            // dl_at_end_of_apply_events
        }
    }
}


pub async fn dl_orders(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let p = GetOrders {
        created: None,
        customer: None,
        ending_before: None,
        // `returns` expanded in both dl and events by default.
        expand: None,
        ids: None,
        limit: Some(100),
        starting_after: None,
        status: None,
        status_transitions: None,
        upstream_ids: None,
    };


    let mut st = c.v1_orders_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let utx = &mut (utx_mt.lock().await);
        let res = val.unwrap();
        for x in res.data {
            Order::insert_tree(utx, run_id, &x);
        }
    }
}

// Note: always expanded on `order`, `order` events are complete (order_return is missing update event with prevents refund from being updated)
// pub async fn dl_order_returns(c: &StripeClient, utx: &mut UniTx<'_>, run_id: i64) {
//     let p = GetOrderReturns {
//         created: None,
//         ending_before: None,
//         expand: None,
//         limit: Some(100),
//         order: None,
//         starting_after: None,
//     };
//
//
//     let mut st = c.v1_order_returns_get_st(&p);
//     pin_mut!(st);
//     while let Some(val) = st.next().await {
//         let res = val.unwrap();
//         for x in res.data {
//             OrderReturn::insert_tree(utx, run_id, &x);
//         }
//     }
// }

pub async fn dl_skus(c: &StripeClient, q_mt_a: &Arc<Mutex<Queue>>, utx_mt: &Mutex<UniTx<'_>>, run_id: i64) {
    let p = GetSkus {
        active: None,
        attributes: None,
        ending_before: None,
        expand: None,
        ids: None,
        in_stock: None,
        limit: Some(100),
        product: None,
        starting_after: None,
    };


    let mut st = c.v1_skus_get_st(&p);
    pin_mut!(st);
    while let Some(val) = st.next().q_high(q_mt_a).await {
        let res = val.unwrap();
        let utx = &mut (utx_mt.lock().await);
        for x in res.data {
            Sku::insert_tree(utx, run_id, &x);
        }
    }
}