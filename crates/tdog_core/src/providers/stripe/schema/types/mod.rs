pub use charge::{*};
pub use customer::{*};
pub use notification_event::{*};
pub use payment_intent::{*};
pub use payment_method::{*};
pub use price::{*};
pub use product::{*};
pub use source::{*};
pub use subscription::{*};
pub use subscription_item::{*};
pub use subscription_schedule::{*};
pub use tax_rate::{*};

use crate::providers::stripe::schema::types::balance_transaction::BalanceTransaction;
use crate::providers::stripe::schema::types::coupon::Coupon;
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema::types::dispute::Dispute;
use crate::providers::stripe::schema::types::invoice::Invoice;
use crate::providers::stripe::schema::types::invoiceitem::Invoiceitem;
use crate::providers::stripe::schema::types::plan::Plan;
use crate::providers::stripe::schema::types::promotion::PromotionCode;
use crate::providers::stripe::schema::types::refund::Refund;
use crate::providers::stripe::schema::types::setup_intent::SetupIntent;

pub mod balance_transaction;
pub mod charge;
pub mod coupon;
pub mod credit_note;
pub mod credit_note_line_item;
pub mod customer;
pub mod discount;
pub mod dispute;
pub mod invoice;
pub mod invoice_line_item;
pub mod invoiceitem;
pub mod notification_event;
pub mod payment_intent;
pub mod payment_method;
pub mod plan;
pub mod price;
pub mod product;
pub mod promotion;
pub mod refund;
pub mod session;
pub mod setup_intent;
pub mod source;
pub mod subscription;
pub mod subscription_item;
pub mod subscription_schedule;
pub mod tax_id;
pub mod tax_rate;
pub mod order;
pub mod order_return;
pub mod sku;
pub mod card;
pub mod bank_account;


// pub mod invoice;


/// The Stripe object type.
///
/// Note: This trait is applied to Row Struct (not API struct).
/// - It is used when logging Stripe row writes to the meta data SQL table.
///     - And ultimately re-used for the first apply_events logic (grouping incoming Stripe events by (type, id) to determine the correct write for a given table/row).
///
/// - Must match exactly the type given in the API JSON results
/// - E.g. its the `event.data.object.object` field.
/// - This is used to group by (type, id) in the apply_events logic to determine create-update-delete lifetimes for SQL rows.
///     - Although Stripe id's are prefixed with a type `src_1IJMW0AvrfsT2stc2D4ajrzO`,
///     *the same ID can be used for many different types*.
///     - E.g. `src_x` is used for both `Source` and `PaymentMethod`.
///     - From Stripes point of view, they PaymentMethod wraps the source, but they copy the Source ID form the wrapped child object to the parent PaymentMethod.
///     - Because they have done it once here, it's likely they will use the same pattern again, so it is better to generally handle it than add specific exceptions.
///
/// - Alternative, A: Add singular table name to the Rust insert macro.
///     - This would work, but not all table's are API objects from Stripe (E.g. meta data, breaking down a Stripe object into smaller rows without their own API defined type).
///
/// - Alternative, B: A tuple (MetaData, RowStruct), where MetaData contains the context of the RowStruct (like what type it orignated from).
///     - This will make code messier, and I prefer the "bottom up" approach of combining traits, esp for static data.
///
/// @see https://stripe.com/docs/api/events/types
pub trait GetObjType {
    fn get_obj_type_static() -> &'static str;

    fn get_obj_type(&self) -> &'static str {
        Self::get_obj_type_static()
    }
}


impl GetObjType for BalanceTransaction {
    fn get_obj_type_static() -> &'static str {
        "balance_transaction"
    }
}


impl GetObjType for Customer {
    fn get_obj_type_static() -> &'static str {
        "customer"
    }
}

impl GetObjType for Charge {
    fn get_obj_type_static() -> &'static str {
        "charge"
    }
}

impl GetObjType for Coupon {
    fn get_obj_type_static() -> &'static str {
        "coupon"
    }
}


impl GetObjType for Discount {
    fn get_obj_type_static() -> &'static str {
        "discount"
    }
}

impl GetObjType for Dispute {
    fn get_obj_type_static() -> &'static str {
        "dispute"
    }
}

impl GetObjType for Invoice {
    fn get_obj_type_static() -> &'static str {
        "invoice"
    }
}

impl GetObjType for Invoiceitem {
    fn get_obj_type_static() -> &'static str {
        "invoiceitem"
    }
}


impl GetObjType for NotificationEvent {
    fn get_obj_type_static() -> &'static str {
        "event"
    }
}

impl GetObjType for PaymentIntent {
    fn get_obj_type_static() -> &'static str {
        "payment_intent"
    }
}

impl GetObjType for PaymentMethod {
    fn get_obj_type_static() -> &'static str {
        "payment_method"
    }
}

impl GetObjType for Plan {
    fn get_obj_type_static() -> &'static str {
        "plan"
    }
}

impl GetObjType for Price {
    fn get_obj_type_static() -> &'static str {
        "price"
    }
}

impl GetObjType for Product {
    fn get_obj_type_static() -> &'static str {
        "product"
    }
}


impl GetObjType for Source {
    fn get_obj_type_static() -> &'static str {
        "source"
    }
}

impl GetObjType for Subscription {
    fn get_obj_type_static() -> &'static str {
        "subscription"
    }
}

impl GetObjType for SubscriptionItem {
    fn get_obj_type_static() -> &'static str {
        "subscription_item"
    }
}

impl GetObjType for SubscriptionSchedule {
    fn get_obj_type_static() -> &'static str {
        "subscription_schedule"
    }
}

impl GetObjType for Refund {
    fn get_obj_type_static() -> &'static str {
        "refund"
    }
}


// Valid Stripe object types (taken from Rust client, UniStrObjectX).
// - "account"
// - "account_link"
// - "alipay_account"
// - "apple_pay_domain"
// - "application"
// - "application_fee"
// - "balance"
// - "balance_transaction"
// - "bank_account"
// - "billing_portal.session"
// - "bitcoin_receiver"
// - "bitcoin_transaction"
// - "capability"
// - "card"
// - "charge"
// - "checkout.session"
// - "connect_collection_transfer"
// - "country_spec"
// - "coupon"
// - "credit_note"
// - "credit_note_line_item"
// - "customer"
// - "customer_balance_transaction"
// - "discount"
// - "dispute"
// - "ephemeral_key"
// - "event"
// - "exchange_rate"
// - "fee_refund"
// - "file"
// - "file_link"
// - "invoice"
// - "invoiceitem"
// - "issuer_fraud_record"
// - "issuing.authorization"
// - "issuing.card"
// - "issuing.cardholder"
// - "issuing.dispute"
// - "issuing.settlement"
// - "issuing.transaction"
// - "item"
// - "line_item"
// - "login_link"
// - "mandate"
// - "order"
// - "order_item"
// - "order_return"
// - "payment_intent"
// - "payment_method"
// - "payout"
// - "person"
// - "plan"
// - "platform_tax_fee"
// - "price"
// - "product"
// - "promotion_code"
// - "radar.early_fraud_warning"
// - "radar.value_list"
// - "radar.value_list_item"
// - "recipient"
// - "refund"
// - "reporting.report_run"
// - "reporting.report_type"
// - "reserve_transaction"
// - "review"
// - "scheduled_query_run"
// - "setup_attempt"
// - "setup_intent"
// - "sku"
// - "source"
// - "source_mandate_notification"
// - "source_transaction"
// - "subscription"
// - "subscription_item"
// - "subscription_schedule"
// - "tax_deducted_at_source"
// - "tax_id"
// - "tax_rate"
// - "terminal.connection_token"
// - "terminal.location"
// - "terminal.reader"
// - "three_d_secure"
// - "token"
// - "topup"
// - "transfer"
// - "transfer_reversal"
// - "usage_record"
// - "usage_record_summary"
// - "webhook_endpoint"