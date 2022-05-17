use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCustomerC00F6E, UniDefaultSource, UniInvoice, UniItemsE47473, UniPaymentMethod, UniPromotionCode, UniSubscription};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string_or_none, Source, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::{ExistsTxSelf, UpsertFirstLevel};

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
#[index("CREATE INDEX invoice ON self (invoice)")]
#[index("CREATE INDEX subscription ON self (subscription)")]
#[index("CREATE INDEX invoice_item ON self (invoice_item)")]
pub struct InvoiceLineItem {
    #[primary_key]
    pub invoice_line_item_id: Option<i64>,

    #[unique]
    pub id: String,

    pub r#type: String,

    pub invoice: String,
    pub invoice_item: Option<String>,
    pub subscription: Option<String>,
    pub subscription_item: Option<String>,

    pub discounts: Option<Value>,
    pub amount: i64,
    pub currency: String,
    pub description: Option<String>,
    pub discount_amounts: Option<Value>,
    pub discountable: bool,
    // `period` is a reserved keyword in Postgres.
    pub period_json: Value,
    pub price: Option<String>,
    pub proration: bool,
    pub quantity: Option<i64>,

    pub tax_amounts: Option<Value>,
    pub tax_rates: Option<Value>,

    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

/// Note: `InvoiceItem` == `invoiceitem`
impl GetObjType for InvoiceLineItem {
    fn get_obj_type_static() -> &'static str {
        "line_item"
    }
}

impl GetId for InvoiceLineItem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// `invoice` id is not included in the `invoice_line_item` object.
/// - It seems to always be used in a context with that has access to the invoice id.
/// - `invoice_line_items` can only be be listed with `dl_list(inv_id)`
/// - The invoice id is needed in this row, because the `invoice.lines` key is limited to the newest 10.
///     - When: dl, all invoice items downloaded (>10), queries will need to join on this invoice id.
pub struct InvoiceLineItemWithParentId<'a> {
    pub parent: String,
    pub data: &'a API::InvoiceLineItem
}


impl From<&InvoiceLineItemWithParentId<'_>> for InvoiceLineItem {
    fn from(x2: &InvoiceLineItemWithParentId) -> Self {
        let x = x2.data;

        InvoiceLineItem {
            invoice_line_item_id: None,
            id: x.id.clone(),
            r#type: x.type_x.to_json_key(),
            invoice: x2.parent.clone(),

            discounts: x.discounts.as_ref().and_then(|x2| {
                x2.iter().map(|x3| {
                    match x3 {
                        UniItemsE47473::String(x4) => x4.clone(),
                        UniItemsE47473::Discount(_) => unreachable!("Expected InvoiceLineItem.discount to always be a string, as 100% of those discounts are downloaded via InvoiceItem with expand=discount OR via events with customer.discount.created. InvoiceLineItem.id={}", &x.id)
                    }
                }).collect::<Vec<String>>().json().into()
            }),
            amount: x.amount,
            currency: x.currency.clone(),
            description: x.description.clone(),
            discount_amounts: x.discount_amounts.as_ref().and_then(|x2| x2.json().into()),
            discountable: x.discountable,
            invoice_item: x.invoice_item.clone(),
            period_json: x.period.json(),
            price: x.price.as_ref().and_then(|x2| x2.id.clone().into()),
            proration: x.proration,
            quantity: x.quantity.clone(),
            subscription: x.subscription.clone(),
            subscription_item: x.subscription_item.clone(),
            tax_amounts: x.tax_amounts.json_or_none(),
            tax_rates: x.tax_rates.as_ref().and_then(|x2| x2.iter().map(|x3| x3.id.clone()).collect::<Vec<String>>().json().into()),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

impl<'b> WriteTree for InvoiceLineItemWithParentId<'b> {
    type APIType = InvoiceLineItemWithParentId<'b>;


    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &InvoiceLineItemWithParentId) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: InvoiceLineItem = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));

        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &InvoiceLineItemWithParentId) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: InvoiceLineItem = data.into();
        writes.push(x.upsert_first_level(utx, run_id));

        writes
    }

    /// Inferred from not being included in a subsequent `invoice.updated, invoice.lines`
    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &InvoiceLineItemWithParentId) -> Vec<i64> {
        // !has_direct_event.
        // Assumption: API will block any deletes of invoice line items for paid invoices (used as part of a transaction calculation).
        let mut x: InvoiceLineItem = data.into();
        vec![x.tx_delete_log_write(utx, run_id, "id")]
    }
}
