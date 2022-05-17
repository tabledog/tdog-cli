use std::collections::HashMap;

// This file defines the foreign keys between Rust structs (that represent 1 level deep rows; no nested types).
// @see `./relations-readme.md`
use serde::{Deserialize, Serialize};
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

use crate::providers::stripe::schema::Db;
use crate::providers::stripe::schema::types::{Charge, Customer, PaymentIntent, PaymentMethod, Price, Product, Source, Subscription, SubscriptionItem, SubscriptionSchedule, TaxRate};
use crate::providers::stripe::schema::types::balance_transaction::BalanceTransaction;
use crate::providers::stripe::schema::types::bank_account::BankAccount;
use crate::providers::stripe::schema::types::card::Card;
use crate::providers::stripe::schema::types::coupon::Coupon;
use crate::providers::stripe::schema::types::credit_note::CreditNote;
use crate::providers::stripe::schema::types::credit_note_line_item::CreditNoteLineItem;
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema::types::dispute::Dispute;
use crate::providers::stripe::schema::types::invoice::Invoice;
use crate::providers::stripe::schema::types::invoice_line_item::InvoiceLineItem;
use crate::providers::stripe::schema::types::invoiceitem::Invoiceitem;
use crate::providers::stripe::schema::types::order::Order;
use crate::providers::stripe::schema::types::order_return::OrderReturn;
use crate::providers::stripe::schema::types::plan::Plan;
use crate::providers::stripe::schema::types::promotion::PromotionCode;
use crate::providers::stripe::schema::types::refund::Refund;
use crate::providers::stripe::schema::types::session::Session;
use crate::providers::stripe::schema::types::setup_intent::SetupIntent;
use crate::providers::stripe::schema::types::sku::Sku;
use crate::providers::stripe::schema::types::tax_id::TaxId;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum RelType {
    // This is a standard SQL FK (id => id OR string => string)
    Normal,

    // JSON array of ids: `[1, 2, 3]` OR `["a", "b", "c"]` OR `[]`.
    JSONArray,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct Edge {
    // `parent` in SQL FK terminology.
    owner_tbl: String,

    // `child` in SQL FK terminology.
    copy_tbl: String,
    copy_col: String,

    rel_type: RelType,

    // Is this relation enforced by the API (and as a result can be enforced by our copy of the DB).
    // - E.g. A Stripe customer can be deleted, but invoices and subs still reference the deleted customer ID.
    // - Collecting this data allows:
    //      - Tests that can check the relations at specific points in time when they should exist.
    //      - CLI that outputs all missing relations to inform query writing.
    enforced_by_api: bool,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum EdgeGrp {
    Single(Edge),

    // When: polymorphic type, where the `copy` can be one of many `owner` tables.
    // E.g. customer.default_source = (cards.id OR bank_account.id OR sources.id)
    Or(Vec<Edge>),
}

impl From<Edge> for EdgeGrp {
    fn from(e: Edge) -> Self {
        EdgeGrp::Single(e)
    }
}

impl From<Vec<Edge>> for EdgeGrp {
    fn from(x: Vec<Edge>) -> Self {
        assert!(x.len() > 1, "Edge does not need to be wrapped with Vec.");
        let first = &x.first().unwrap();
        for x2 in &x {
            // A group references the same copy table and field (one of many possible owners point to a single copy).
            assert_eq!(first.copy_tbl, x2.copy_tbl);
            assert_eq!(first.copy_col, x2.copy_col);
            assert_eq!(first.enforced_by_api, x2.enforced_by_api);
            assert_eq!(first.rel_type, x2.rel_type);
        }

        EdgeGrp::Or(x)
    }
}


impl Edge {
    fn std<O: TableStatic, C: TableStatic>(copy_col: &str) -> Edge {
        let owner_tbl = O::get_table_name_static().to_string();
        let copy_tbl = C::get_table_name_static().to_string();

        Edge {
            owner_tbl,
            copy_tbl,
            copy_col: copy_col.to_string(),
            rel_type: RelType::Normal,
            enforced_by_api: true,
        }
    }

    fn std_not_enforced<O: TableStatic, C: TableStatic>(copy_col: &str) -> Edge {
        let owner_tbl = O::get_table_name_static().to_string();
        let copy_tbl = C::get_table_name_static().to_string();

        Edge {
            owner_tbl,
            copy_tbl,
            copy_col: copy_col.to_string(),
            rel_type: RelType::Normal,
            enforced_by_api: false,
        }
    }

    fn json_array<O: TableStatic, C: TableStatic>(copy_col: &str) -> Edge {
        let owner_tbl = O::get_table_name_static().to_string();
        let copy_tbl = C::get_table_name_static().to_string();

        Edge {
            owner_tbl,
            copy_tbl,
            copy_col: copy_col.to_string(),
            rel_type: RelType::JSONArray,
            enforced_by_api: true,
        }
    }

    fn json_array_not_enforced<O: TableStatic, C: TableStatic>(copy_col: &str) -> Edge {
        let owner_tbl = O::get_table_name_static().to_string();
        let copy_tbl = C::get_table_name_static().to_string();

        Edge {
            owner_tbl,
            copy_tbl,
            copy_col: copy_col.to_string(),
            rel_type: RelType::JSONArray,
            enforced_by_api: false,
        }
    }
}

/// Store as data to enable use as input into other tooling/processes.
/// - E.g.
///     - Adding native FK constraints depending on SQL engine (alter table add fk after all tables are created).
///     - Future API changes (GraphQL/read-txs).
///     - Auto generated documentation/code for other tooling.
///
/// To quickly define edges, new:
/// - `relations.json`
///     - Side by side with this file, progress top - bottom.
///     - Q: Why not auto generate this edge meta data?
///         - It needs a human decision to determine if:
///             - It is enforced by the API.
///             - If the type is a JSON array of strings (this could be done if the JSON type was encoded in the SQL row Rust struct instead of String).
///             - The field contains an ID of the owner table (and not just a string).
///
///
/// To quickly define edges, old:
/// - Use regex `^\s+[a-z0-9_]+:(?<=customer.+)`.
/// - IntelliJ search against dir `lib-app/src/providers/stripe/schema/types`
/// - Pin search window, type relations here.
///
/// @todo/low Document all the exceptions to the relations so that users know:
/// - Which relations are consistent on first download (because of the expanded objects).
/// - Which relations are sometimes consistent (only customer if not deleted).
/// - Which relations are never consistent (payment method not attached to customer at apply events time).
///
/// @todo/low Use notes/exceptions on this list to choose which URL/objects to download at event processing time to keep sets complete and queries correct/symmetric at download/event apply times.
fn get_edges() -> Vec<EdgeGrp> {
    let mut o: Vec<EdgeGrp> = vec![];


    // Charge
    {
        o.push(Edge::std::<Charge, Dispute>("charge").into());
        o.push(Edge::std::<Charge, Invoice>("charge").into());
        o.push(Edge::std::<Charge, Order>("charge").into());

        // This is a list limited to just one item (has_more=true when the length is >1).
        // - Users can query via `charge.payment_intent`
        // o.push(Edge::json_array::<Charge, PaymentIntent>("charges"));

        o.push(Edge::std::<Charge, Refund>("charge").into());
    }


    // Coupon
    {
        o.push(Edge::std::<Coupon, Discount>("coupon").into());
        // What is `external_coupon_code`?
        o.push(Edge::std::<Coupon, PromotionCode>("coupon").into());
    }


    // CreditNote
    {
        o.push(Edge::std::<CreditNote, CreditNoteLineItem>("credit_note_id").into());
    }

    // CreditNoteLineItem
    {
        o.push(Edge::json_array::<CreditNoteLineItem, CreditNote>("lines_first_x").into());
    }


    // Customer
    {
        // When customer is deleted these are not valid missing relations.
        o.push(Edge::std_not_enforced::<Customer, Charge>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, CreditNote>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, Discount>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, Invoice>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, Invoiceitem>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, Order>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, PaymentIntent>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, PaymentMethod>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, PromotionCode>("customer").into());
        // o.push(Edge::std_not_enforced::<Customer, Session>("customer"));
        o.push(Edge::std_not_enforced::<Customer, SetupIntent>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, Source>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, Subscription>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, SubscriptionSchedule>("customer").into());
        o.push(Edge::std_not_enforced::<Customer, TaxId>("customer").into());
    }


    // Discount
    {
        o.push(Edge::std::<Discount, Customer>("discount").into());
        o.push(Edge::json_array::<Discount, Invoice>("discounts").into());
        o.push(Edge::json_array::<Discount, InvoiceLineItem>("discounts").into());
        o.push(Edge::json_array::<Discount, Invoiceitem>("discounts").into());
        o.push(Edge::std::<Discount, Subscription>("discount").into());
    }


    // Invoice
    {
        o.push(Edge::std::<Invoice, Charge>("invoice").into());
        o.push(Edge::std::<Invoice, CreditNote>("invoice").into());

        o.push(Edge::std::<Invoice, Discount>("invoice").into());
        o.push(Edge::std::<Invoice, InvoiceLineItem>("invoice").into());
        o.push(Edge::std::<Invoice, Invoiceitem>("invoice").into());
        o.push(Edge::std::<Invoice, PaymentIntent>("invoice").into());
        o.push(Edge::std::<Invoice, Subscription>("latest_invoice").into());
    }


    // InvoiceLineItem
    {
        o.push(Edge::std::<InvoiceLineItem, CreditNoteLineItem>("invoice_line_item").into());
        o.push(Edge::json_array::<InvoiceLineItem, Invoice>("lines_newest_10").into());

        // @todo/next Search for `lines`, `lines_last_10` and `lines_x`
    }


    // Order
    {
        o.push(Edge::std::<Order, Charge>("order_id").into());
        o.push(Edge::std::<Order, OrderReturn>("order_id").into());
        o.push(Edge::std::<Order, OrderReturn>("order_id").into());
    }


    // PaymentIntent
    {
        o.push(Edge::std::<PaymentIntent, Charge>("payment_intent").into());
        o.push(Edge::std::<PaymentIntent, Dispute>("payment_intent").into());
        o.push(Edge::std::<PaymentIntent, Invoice>("payment_intent").into());
        o.push(Edge::std::<PaymentIntent, Refund>("payment_intent").into());
        // o.push(Edge::std::<PaymentIntent, Session>("payment_intent"));
    }

    // PaymentMethod
    // - These are not enforced as they are missing from the event stream when not attached to a customer (should be complete at dl time).
    {

        // Exception: walk_4, `(d),d`.
        // - When:
        //      - Customer is deleted.
        //          - PaymentMethod's are not downloaded as the API lists them via (customerId, paymentMethodType).
        // - Fix, A: Ignore for the moment.
        //      - The payment_method data has a snapshot at `charge.payment_method_details`
        //          - The other types lead to this field within 1-3 joins.
        //      - `payment_method` is not expandable on charge listing.
        // - Fix, B: Expand payment_methods on all types at dl time, upsert on insert_tree.
        //      - Completed.

        // When: customer is deleted:
        // `stripe payment_methods list --customer cus_JTE23nzRgTyrxp`
        // - Error: resource missing
        // `stripe payment_methods retrieve pm_1IqHa0Jo6Ja94JKP6kY2GVuU`
        // - Returns object.


        // Issue: Is it possible to have a (charge && deleted_customer && not(inv, x_intent, sub))? - In this case this join will fail as the payment_method cannot be expanded on the charge object at dl time.
        o.push(Edge::std_not_enforced::<PaymentMethod, Charge>("payment_method").into()); // expandable=0

        o.push(Edge::std_not_enforced::<PaymentMethod, Invoice>("default_payment_method").into()); // expandable=1
        o.push(Edge::std_not_enforced::<PaymentMethod, PaymentIntent>("payment_method").into()); // expandable=1
        o.push(Edge::std_not_enforced::<PaymentMethod, SetupIntent>("payment_method").into()); // expandable=1
        o.push(Edge::std_not_enforced::<PaymentMethod, Subscription>("default_payment_method").into()); // expandable=1

        // Assumption: For events, all new payment_methods will trigger `payment_method.attach`.
        // - payment_method is never expanded on events, so this is the only way to write new creations.


        // @todo/low Issue: PaymentMethods cannot be kept in sync with just the events stream.
        // - E.g. PaymentIntents create payment methods *without ever attaching them to a customer*.
        //      - This means that `payment_method.attached` is not triggered.
        //      - `payment_method` is not expanded.
        //          - So those new `payment_methods` are never in the event stream.
        //      - Note: last_payment_error.payment_method is expanded in events.
        //
        // - Fix, A: Ignore for the moment.
        //      - charge.payment_method_details can be used for missing payment methods
        //      - Downloads should be complete as they expand payment methods.
        //      - Eventual fix: download payment method whilst processing the events.
        //      - These rows do not contain monetary sums so should not result in monetary query errors.
        //      - Still keeps customer-attached payment methods up to date.
    }


    // Price
    {
        o.push(Edge::std::<Price, InvoiceLineItem>("price").into());
        o.push(Edge::std::<Price, Invoiceitem>("price").into());
        // o.push(Edge::std::<Price, Sku>("price")); Not a FK, is an integer.
        o.push(Edge::std::<Price, SubscriptionItem>("price").into());
    }

    // Product
    {
        // o.push(Edge::std::<Product, Plan>("product")); // Plans not written as data mirrored in price objects.


        // Issue: When:
        //      - Using `await stripe.invoiceItems.create({amount: 1000})`
        //          - This creates a product with `name="Product for invoice item ii_1IqOr4Jo6Ja94JKPITQG2V0q", active=false`
        //          - Issue: this is invisible to both dl and events.
        //              - DL:
        //                  - `stripe products list --active=false` does not list them.
        //                  - ``stripe products retrieve prod_JTLYGr69gUwWJt` *does* return the object.
        //                      - @todo/maybe download each product for each event to ensure this relation is consistent.
        //                      - Or: Allow customised relation queries to exclude invisible product IDs created via the invoice item API.
        //              - Events: Only the ID to this newly created product is in the event stream.
        o.push(Edge::std_not_enforced::<Product, Price>("product").into());

        o.push(Edge::std::<Product, Sku>("product").into());
    }


    // PromotionCode
    {
        o.push(Edge::std::<PromotionCode, Discount>("promotion_code").into());
    }

    // Refund
    {
        o.push(Edge::json_array::<Refund, Charge>("refunds").into());
        o.push(Edge::std::<Refund, CreditNote>("refund").into());
        o.push(Edge::std::<Refund, OrderReturn>("refund").into());
    }


    // Session
    {
        // o.push(Edge::std::<Session, Discount>("checkout_session"));
    }

    // SetupIntent
    {
        // o.push(Edge::std::<SetupIntent, Session>("setup_intent"));
        o.push(Edge::std::<SetupIntent, Subscription>("pending_setup_intent").into());
    }

    // Source
    {
        // Note: this is not a `Source`, it is the ID almost any Stripe object.
        // o.push(Edge::std::<Source, BalanceTransaction>("source"));

        // Note: When adding a source to a customer using the older source API (`stripe.customers.createSource(c_1, {source: "tok_x"});`)
        //  - A `payment_method.attached` event is triggered along with `customer.source.created`, *but both have the same `card_123` ID*.
        //      - True: old-source triggers a new-payment-method.
        //      - False: new-payment-method triggers a old-source.
        //          - This means:
        //              - old source API users can use the payment methods APIs.
        //              - new payment method API users cannot use the old source APIs (and fields).
        //
        // So old source API users should query using payment method fields/tables.
        //      - But if they need just the data read/written by the old source API's they will still need those tables synced/written to (this is why it may still be worth writing the old source API state to tables in addition to payment methods even though the data is copied from source->payment method).
        o.push(
            vec![
                Edge::std::<Source, Customer>("default_source"),
                Edge::std::<Card, Customer>("default_source"),
                Edge::std::<BankAccount, Customer>("default_source"),
            ].into()
        );

        o.push(
            vec![
                Edge::std::<Source, Subscription>("default_source"),
                Edge::std::<Card, Subscription>("default_source"),
                Edge::std::<BankAccount, Subscription>("default_source"),
            ].into()
        );

        o.push(
            vec![
                Edge::std::<Source, Invoice>("default_source"),
                Edge::std::<Card, Invoice>("default_source"),
                Edge::std::<BankAccount, Invoice>("default_source"),
            ].into()
        );
    }

    // Subscription
    {
        o.push(Edge::std::<Subscription, Discount>("subscription").into());
        o.push(Edge::std::<Subscription, Invoice>("subscription").into());
        o.push(Edge::std::<Subscription, InvoiceLineItem>("subscription").into());
        o.push(Edge::std::<Subscription, Invoiceitem>("subscription").into());
        // o.push(Edge::std::<Subscription, Session>("subscription"));
        o.push(Edge::std::<Subscription, SubscriptionItem>("subscription").into());
        o.push(Edge::std::<Subscription, SubscriptionSchedule>("subscription").into());
        o.push(Edge::std::<Subscription, SubscriptionSchedule>("released_subscription").into());
    }


    // SubscriptionItem
    {
        // @todo/low What happens when a invoice is generated and the sub_item has been deleted? Add to tests to ensure this is that case.
        // - invoiceitems move to invoice_line_items when the upcoming-no-id invoice is created.
        //      - If the (sub_items, invoiceitems) relation is not enforced, this one should not be either.
        o.push(Edge::std_not_enforced::<SubscriptionItem, InvoiceLineItem>("subscription_item").into());

        // When:
        // - Sub items are removed from subscription (with `stripe.subscriptionItems.del`).
        // - Stripe generates "Unused ..." and "Remaining ..." invoiceitems for the upcoming invoice.
        //      - These have the original sub_item.id that is now deleted (and cannot be downloaded).
        //          - So a sub_item.id that is no longer active/attached to a parent sub can still be copied into an invoiceitem and effect the next invoice total (but a join from invoiceitems -> sub_items will not work for deleted sub_items).
        //
        // Fix, A: Still delete sub_items as all the data needed is copied to the invoiceitem (amount, quantity, tax_rates) and also exists on the `price`, only sub_item.billing_thresholds is missing which is for defining when to generate the next invoice for the parent sub.
        o.push(Edge::std_not_enforced::<SubscriptionItem, Invoiceitem>("subscription_item").into());

        // select i.id, i.subscription, i.subscription_item, (select json_group_array(write_type) from td_stripe_writes where obj_id=i.subscription_item order by write_id asc) cud, s.id, i.* from invoiceitems i left join subscription_items s on(i.subscription_item=s.id) where i.subscription = "sub_JVWPI1XDd7RaIW"
    }

    // TaxId
    {
        // These are the full tax id objects, JSON objects are small so include them.
        // o.push(Edge::json_array::<TaxId, Invoice>("customer_tax_ids"));
    }


    // TaxRate
    {
        o.push(Edge::json_array::<TaxRate, CreditNoteLineItem>("tax_rates").into());

        o.push(Edge::json_array::<TaxRate, Invoice>("default_tax_rates").into());
        o.push(Edge::json_array::<TaxRate, InvoiceLineItem>("tax_rates").into());
        o.push(Edge::json_array::<TaxRate, Invoiceitem>("tax_rates").into());

        o.push(Edge::json_array::<TaxRate, Subscription>("default_tax_rates").into());
        o.push(Edge::json_array::<TaxRate, SubscriptionItem>("tax_rates").into());
    }


    o
}


impl Db {
    pub fn get_missing_owner_all(uc: &UniCon) -> Vec<(EdgeGrp, Vec<(String, String)>)> {
        let mut o = vec![];


        let get_missing = |e: &Edge| {
            match e.rel_type {
                RelType::Normal => get_missing_owner(&uc, &e.owner_tbl, &e.copy_tbl, &e.copy_col),
                RelType::JSONArray => get_missing_owner_json_array(&uc, &e.owner_tbl, &e.copy_tbl, &e.copy_col),
            }
        };


        for g in get_edges() {
            match g {
                EdgeGrp::Single(ref e) => {
                    if !e.enforced_by_api {
                        continue;
                    }

                    let missing = get_missing(&e);
                    if missing.len() > 0 {
                        o.push((g.clone(), missing));
                    }
                }
                EdgeGrp::Or(ref x2) => {
                    if !&x2.first().unwrap().enforced_by_api {
                        continue;
                    }

                    let mut hm = HashMap::new();

                    for e in x2 {
                        let missing = get_missing(&e);
                        for m in missing {
                            let i = hm.entry(m).or_insert(0);
                            *i += 1;
                        }
                    }

                    let total_or = x2.len() as u32;

                    let missing = hm.into_iter().filter(|(copy_id, count)| {
                        assert!(count <= &total_or);
                        // 0 edges exist; at least one should (OR).
                        count == &total_or
                    }).map(|(copy_id, count)| copy_id).collect::<Vec<(String, String)>>();

                    if missing.len() > 0 {
                        o.push((g.clone(), missing));
                    }
                }
            }
        }

        o
    }
}


// O = Owner (parent)
// C = Copy (child)
// @todo/low Replace `fk_field` with static trait.
fn get_missing_owner(uc: &UniCon, owner_tbl: &str, copy_tbl: &str, copy_field_name: &str) -> Vec<(String, String)> {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            let q = format!(r###"
                select copy.id, copy.{} from {} copy left join {} owner on(copy.{}=owner.id) where owner.id is null and copy.{} is not null
            "###, copy_field_name, copy_tbl, owner_tbl, copy_field_name, copy_field_name);

            let mut stmt = c.prepare_cached(&q).unwrap();
            let mut rows = stmt.query([]).unwrap();
            let mut o = vec![];
            while let Some(row) = rows.next().unwrap() {
                o.push((
                    row.get(0).unwrap(),
                    row.get(1).unwrap()
                ));
            }
            return o;
        }
        _ => unreachable!("Not implemented for other engines as logic tests are done in SQLite.")
    }
}


/// For every json array of fk ids:
/// - Ensure that those ids join back to the table where the id is the primary key (or a proxy for the primary key, like the Stripe ID mapping 1:1 with the SQLite rowid).
///     - Implicitly ensures join-based queries are correct.
fn get_missing_owner_json_array(uc: &UniCon, owner_tbl: &str, copy_tbl: &str, copy_field_name: &str) -> Vec<(String, String)> {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            let q = format!(r###"
                select
                    copy.id, copy.atom
                from
                    (
                        select c.id, je.atom from {} c, json_each(c.{}) je
                    ) copy
                    left join {} owner on(copy.atom=owner.id)
                where owner.id is null
            "###, copy_tbl, copy_field_name, owner_tbl);

            let mut stmt = c.prepare_cached(&q).unwrap();
            let mut rows = stmt.query([]).unwrap();
            let mut o = vec![];
            while let Some(row) = rows.next().unwrap() {
                o.push((
                    row.get(0).unwrap(),
                    row.get(1).unwrap()
                ));
            }
            return o;
        }
        _ => unreachable!()
    }

}
