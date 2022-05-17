use std::collections::HashMap;
use std::fmt::Debug;

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses as APIRes;
use stripe_client::types::types as API;
use stripe_client::types::types::{UniAccount, UniCharge, UniCustomerC00F6E, UniCustomerEDC00A, UniInvoice, UniOrder, UniPaymentIntent, UniPaymentMethod, UniProduct2CB1D4, UniRecipient, UniRefund, UniSetupIntent, UniSubscription};
use unicon::{*};
use unicon::dt::{*};
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

pub use util::{*};

use crate::fns::{get_utc_dt, get_utc_dt_from_3ms};
use crate::providers::stripe::schema::types::{*};
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
use crate::providers::stripe::schema::util::{*};

use super::schema_meta::{*};

pub mod types;
pub mod util;
pub mod relations;

//use unicon_proc_macro::{
//     Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite,
//     MySQLString, MySQLStringSchema
// };
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Debug, Clone, PartialEq)]
#[derive(Db)]
// #[derive(SQLiteStringSchema)]
// #[derive(MySQLStringSchema)]
pub enum Db {
    TdStripeApplyEvent(TdStripeApplyEvent),
    TdStripeWrite(TdStripeWrite),
    TdRun(TdRun),
    TdMetadata(TdMetadata),

    // Address(Address),
    BalanceTransaction(BalanceTransaction),
    BankAccount(BankAccount),
    Card(Card),
    Charge(Charge),
    Coupon(Coupon),
    CreditNote(CreditNote),
    CreditNoteLineItem(CreditNoteLineItem),
    Customer(Customer),
    Discount(Discount),
    Dispute(Dispute),
    Invoice(Invoice),
    Invoiceitem(Invoiceitem),
    InvoiceLineItem(InvoiceLineItem),

    NotificationEvent(NotificationEvent),
    Order(Order),
    OrderReturn(OrderReturn),

    PaymentMethod(PaymentMethod),
    PaymentIntent(PaymentIntent),

    // Prices replace plans (plan writes are mirrored to price list/events; users should query prices instead).
    // Plan(Plan),

    Price(Price),
    Product(Product),
    PromotionCode(PromotionCode),

    Refund(Refund),
    SetupIntent(SetupIntent),
    Sku(Sku),
    // Session(Session),
    Source(Source),
    Subscription(Subscription),
    SubscriptionItem(SubscriptionItem),
    SubscriptionSchedule(SubscriptionSchedule),
    TaxId(TaxId),
    TaxRate(TaxRate),

}

impl Db {
    /// All types that can be written to their own SQL tables (matches `Db`).
    pub fn event_is_table_write(e: &API::NotificationEvent) -> bool {
        use API::UniNotificationEventDataObject::*;

        match &(*e.data.object) {
            Charge(_) |
            Coupon(_) |
            CreditNote(_) |
            Customer(_) |
            Discount(_) |
            Dispute(_) |
            UniPolymorphic70BAFA(_) | // card|bank via `customer.source.x` event.
            InvoiceItem(_) |
            Order(_) |
            OrderReturn(_) |
            PaymentIntent(_) |
            PaymentMethod(_) |
            Price(_) |
            // Plan(_) Note: plans are just aliases for prices.
            Product(_) |
            PromotionCode(_) |
            Refund(_) |
            SetupIntent(_) |
            // Session(_) | See comment on test.
            Sku(_) |

            // Note: `customer.source.created` can contain (source, card, bank, ...).
            Source(_) |
            Subscription(_) |
            // SubscriptionItem(_) | (!has_dl_list && !has_direct_event) - these are inserted/upserted with Subscription which always includes 100% of the sub items.
            SubscriptionSchedule(_) |
            TaxId(_) |
            TaxRate(_)
            => true,
            Invoice(i) => {
                // Ignore these as they have no `id` - it is the only exception to the rule `every type has an id` which the write log is based on.
                // Docs: `invoice.upcoming` event, Occurs X number of days before a subscription is scheduled to create an invoice that is automatically charged—where X is determined by your subscriptions settings. Note: The received Invoice object will not have an invoice ID.
                if e.type_x == "invoice.upcoming" {
                    assert_eq!((*i).id, None);
                    return false;
                }
                true
            }
            _ => false
        }
    }
}

/// `WriteTree` converts a single tree-like (Rust struct) API data structure into many SQL rows which can be inserted.
/// - Note: Some tree structures may flatten into many rows (E.g. a parent and many children).
/// - The trait is used to allow a single function to process many different (APITypeX, RowTypeY) combinations.
/// - Also used to call the same function from both download and events.
pub trait WriteTree {
    /// Note: Associated types easier as each trait fn call site does not need to declare a concrete generic type param.
    /// - In the future a generic trait may be used if there are many APIData sources (HTTP, GQL etc) for a single SAAS API.
    type APIType;


    /// Used when the code is certain the first-level row does not exist.
    /// - Note: child elements may still be upserted.
    ///     - When: child type !has_dl_list && used_in_many_parent_objects (E.g. discount).
    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &Self::APIType) -> Vec<i64>;


    // `update_tree` needed?


    /// Used at apply_events, for each event, to ensure the write happens regardless of if it is an insert or update.
    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &Self::APIType) -> Vec<i64>;

    /// `.deleted` event triggers this; it does not need to target a row (changes can be 0).
    /// - E.g. the 30 day event window could remove c, u events, and only return the d event.
    ///     - The download list is missing the deleted object, and the d event is applied.
    ///     - `delete from x where y=z` will return changes=0, which is OK in this case (delete if it exists - no need to do an exists query).
    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &Self::APIType) -> Vec<i64>;


    // delete_if_exists
}


trait GetIdAny {
    /// E.g. For an enum that represents one of many of the same object (E.g. customer string, obj, deleted_obj).
    /// - Use when it does not matter if the network API type is a string or an object.
    fn get_id_any(&self) -> String;
}

impl GetIdAny for UniCustomerC00F6E {
    fn get_id_any(&self) -> String {
        match self {
            UniCustomerC00F6E::String(x) => x.clone(),
            UniCustomerC00F6E::Customer(x) => x.id.clone(),
            UniCustomerC00F6E::DeletedCustomer(x) => x.id.clone()
        }
    }
}


impl GetIdAny for UniCustomerEDC00A {
    fn get_id_any(&self) -> String {
        match self {
            UniCustomerEDC00A::String(x) => x.clone(),
            UniCustomerEDC00A::Customer(x) => x.id.clone()
        }
    }
}


impl GetIdAny for UniInvoice {
    fn get_id_any(&self) -> String {
        match self {
            UniInvoice::String(x) => x.clone(),
            UniInvoice::Invoice(x) => x.id.clone().unwrap()
        }
    }
}


impl GetIdAny for UniPaymentIntent {
    fn get_id_any(&self) -> String {
        match self {
            UniPaymentIntent::String(x) => x.clone(),
            UniPaymentIntent::PaymentIntent(x) => x.id.clone(),
        }
    }
}

impl GetIdAny for UniSetupIntent {
    fn get_id_any(&self) -> String {
        match self {
            UniSetupIntent::String(x) => x.clone(),
            UniSetupIntent::SetupIntent(x) => x.id.clone()
        }
    }
}

impl GetIdAny for UniSubscription {
    fn get_id_any(&self) -> String {
        match self {
            UniSubscription::String(x) => x.clone(),
            UniSubscription::Subscription(x) => x.id.clone()
        }
    }
}

impl GetIdAny for UniCharge {
    fn get_id_any(&self) -> String {
        match self {
            UniCharge::String(x) => x.clone(),
            UniCharge::Charge(x) => x.id.clone()
        }
    }
}

impl GetIdAny for UniRefund {
    fn get_id_any(&self) -> String {
        match self {
            UniRefund::String(x) => x.clone(),
            UniRefund::Refund(x) => x.id.clone()
        }
    }
}


impl GetIdAny for UniOrder {
    fn get_id_any(&self) -> String {
        match self {
            UniOrder::String(x) => x.clone(),
            UniOrder::Order(x) => x.id.clone()
        }
    }
}

impl GetIdAny for UniProduct2CB1D4 {
    fn get_id_any(&self) -> String {
        match self {
            UniProduct2CB1D4::String(x) => x.clone(),
            UniProduct2CB1D4::Product(x) => x.id.clone()
        }
    }
}

impl GetIdAny for UniPaymentMethod {
    fn get_id_any(&self) -> String {
        match self {
            UniPaymentMethod::String(x) => x.clone(),
            UniPaymentMethod::PaymentMethod(x) => x.id.clone()
        }
    }
}

impl GetIdAny for UniAccount {
    fn get_id_any(&self) -> String {
        match self {
            UniAccount::String(x) => x.clone(),
            UniAccount::Account(x) => x.id.clone()
        }
    }
}

impl GetIdAny for UniRecipient {
    fn get_id_any(&self) -> String {
        match self {
            UniRecipient::String(x) => x.clone(),
            UniRecipient::TransferRecipient(x) => x.id.clone()
        }
    }
}

trait GetIdFromEnum {
    // Question: Can you use a generic type in a match: `match T {T::String(s) ...}`? (instead of implementing match for every concrete enum type)?
    fn get_id(&self) -> String;
}

trait GetIdFromEnumOrNone {
    fn get_id_or_none(&self) -> Option<String>;
}

/// Gets the id and object type of a Stripe data object.
/// - Especially useful when dealing with enums with polymorphic types where all variants are an object with an id (instead of creating a match branch for 20+ variants).
/// @todo/med Have the Open API generator output this traits function without having to serialize to JSON (very expensive to stringify large structures just to get a single field).
/// - Or: impl trait for each enum and use match manually.
pub trait GetIdAndObject {
    fn get_id_and_object_type(&self) -> (String, String);
    fn get_id_and_object_type_opt(&self) -> (Option<String>, Option<String>);
}

impl<T> GetIdAndObject for T where T: Serialize + Debug {
    fn get_id_and_object_type(&self) -> (String, String) {
        match serde_json::to_value(self).unwrap() {
            Value::Object(mut x) => {
                match (x.remove("id"), x.remove("object")) {
                    (Some(Value::String(id)), Some(Value::String(object))) => {
                        return (id, object);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        dbg!(self);
        panic!("Could not get the (id, object_type) from JSON structure; the function was called on an incorrect type.")
    }

    /// Some objects have an object type but no id, E.g. `balance.available` from notification events.
    fn get_id_and_object_type_opt(&self) -> (Option<String>, Option<String>) {
        let mut id = None;
        let mut object = None;
        match serde_json::to_value(self).unwrap() {
            Value::Object(mut x) => {
                if let Some(Value::String(s)) = x.remove("id") {
                    id = Some(s);
                }
                if let Some(Value::String(s)) = x.remove("object") {
                    object = Some(s);
                }
            }
            _ => {}
        }
        (id, object)
    }
}


impl<T> GetIdFromEnumOrNone for Option<T> where T: GetIdFromEnum {
    fn get_id_or_none(&self) -> Option<String> {
        self.as_ref()?.get_id().into()
    }
}


// @todo/high Move this to the Open API code generator (instead of implementing each by hand).
impl GetIdFromEnum for API::UniApplication {
    fn get_id(&self) -> String {
        if let API::UniApplication::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniCustomerC00F6E {
    fn get_id(&self) -> String {
        if let API::UniCustomerC00F6E::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniInvoice {
    fn get_id(&self) -> String {
        if let API::UniInvoice::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniAccount {
    fn get_id(&self) -> String {
        if let API::UniAccount::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniPaymentMethod {
    fn get_id(&self) -> String {
        if let API::UniPaymentMethod::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniReview {
    fn get_id(&self) -> String {
        if let API::UniReview::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}


impl GetIdFromEnum for API::UniTransfer {
    fn get_id(&self) -> String {
        if let API::UniTransfer::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniBalanceTransaction {
    fn get_id(&self) -> String {
        if let API::UniBalanceTransaction::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniFee {
    fn get_id(&self) -> String {
        if let API::UniFee::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniPaymentIntent {
    fn get_id(&self) -> String {
        if let API::UniPaymentIntent::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniOrder {
    fn get_id(&self) -> String {
        if let API::UniOrder::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniTransferReversal {
    fn get_id(&self) -> String {
        if let API::UniTransferReversal::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}

impl GetIdFromEnum for API::UniCharge {
    fn get_id(&self) -> String {
        if let API::UniCharge::String(s) = self {
            return s.clone();
        }
        unreachable!("should be string")
    }
}



