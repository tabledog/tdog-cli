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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string_or_none, Source, ToDT, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::{ExistsTxSelf, UpsertFirstLevel};

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Invoiceitem {
    #[primary_key]
    pub invoiceitem_id: Option<i64>,

    #[unique]
    pub id: String,

    pub customer: String,


    // Note: creating an invoiceitem with no invoice means "add this to their next invoice at bill time" (it is possible some implementations allow days or weeks to pass with invoiceitems not attached to invoices; it is a useful queryable state, so invoiceitems should be dl-listed/event-updated and not just inserted via the parent invoice object).
    pub invoice: Option<String>,

    pub subscription: Option<String>,
    pub subscription_item: Option<String>,


    // Note: there is another type, InvoiceLineItem, that is a child of invoice, *with different fields*.
    // discount_amounts

    // Add discount ids to force queries to start from the parent->child.
    // Querying via the discounts table would be ideal/natural for SQL, but when discounts are deleted/removed from their parent, they still contain the parent id even though they are not attached.
    // @todo/high Document this issue as it could lead to invalid queries (or possibly add a deleted/attached flag).
    pub discounts: Option<Value>,

    pub amount: i64,
    pub currency: String,
    // `date` is a reserved keyword in Postgres.
    pub date_ts: DT,
    pub description: Option<String>,
    pub discountable: bool,
    // `period` is a reserved keyword in Postgres.
    pub period_json: Value,
    pub price: Option<String>,
    pub proration: bool,
    pub quantity: i64,

    pub tax_rates: Option<Value>,
    pub unit_amount: Option<i64>,
    pub unit_amount_decimal: Option<String>,

    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetId for Invoiceitem {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::InvoiceItem> for Invoiceitem {
    fn from(x: &API::InvoiceItem) -> Self {
        Invoiceitem {
            invoiceitem_id: None,
            id: x.id.clone(),
            customer: if let UniCustomerC00F6E::String(s) = &x.customer {
                s.clone()
            } else {
                unreachable!("Invoiceitem customer should always be a string, and not expanded")
            },
            discounts: x.discounts.as_ref().and_then(|x2| {
                x2.iter().map(|x3| {
                    match x3 {
                        UniItemsE47473::String(s) => s.clone(),
                        UniItemsE47473::Discount(d) => d.id.clone()
                    }
                }).collect::<Vec<String>>().json().into()
            }),
            invoice: x.invoice.as_ref().and_then(|x2| {
                match x2 {
                    UniInvoice::String(s) => s.clone().into(),
                    UniInvoice::Invoice(_) => unreachable!("Invoiceitem invoice should be a string")
                }
            }),
            subscription: x.subscription.as_ref().and_then(|x2| {
                match x2 {
                    UniSubscription::String(s) => s.clone().into(),
                    UniSubscription::Subscription(_) => unreachable!("Invoiceitem subscription should be a string")
                }
            }),
            amount: x.amount,
            currency: x.currency.clone(),
            date_ts: x.date.to_dt(),
            description: x.description.clone(),
            discountable: x.discountable,
            period_json: x.period.json(),
            price: x.price.as_ref().and_then(|x2| x2.id.clone().into()),
            proration: x.proration,
            quantity: x.quantity,
            subscription_item: x.subscription_item.clone(),

            // tax_rates = has_dl_list && has_direct_events (only write ids here to avoid having to update them when any of the data changes).
            tax_rates: x.tax_rates.as_ref().and_then(|x2| x2.iter().map(|x3| x3.id.clone()).collect::<Vec<String>>().json_or_none()),
            unit_amount: x.unit_amount.clone(),
            unit_amount_decimal: x.unit_amount_decimal.clone(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}


fn upsert_discounts(utx: &mut UniTx, run_id: i64, data: &API::InvoiceItem, writes: &mut Vec<i64>) {
    if let Some(all) = &data.discounts {
        for x in all {
            match x {
                /// When: downloading list with expand=discounts
                UniItemsE47473::Discount(x2) => {
                    writes.append(&mut Discount::upsert_tree(utx, run_id, &x2));
                }
                /// When: events containing invoiceitem
                UniItemsE47473::String(_) => return
            }
        }
    }
}


impl WriteTree for Invoiceitem {
    type APIType = API::InvoiceItem;

    /// @see https://stripe.com/docs/payments/payment-intents/migration#saved-cards
    /// - This is useful to compare the state transitions between (old:sources, new:payment intent).
    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::InvoiceItem) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Invoiceitem = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));

        upsert_discounts(utx, run_id, &data, &mut writes);


        // Upsert price.
        if let Some(data_price) = &data.price {
            // When price created via inline `price_data`: `price.created` not triggered, price not in dl list.
            writes.append(&mut Price::upsert_tree(utx, run_id, &data_price));
        }


        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::InvoiceItem) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Invoiceitem = data.into();
        writes.push(x.upsert_first_level(utx, run_id));

        upsert_discounts(utx, run_id, &data, &mut writes);

        // Upsert price.
        if let Some(data_price) = &data.price {
            // When price created via inline `price_data`: `price.created` not triggered, price not in dl list.
            writes.append(&mut Price::upsert_tree(utx, run_id, &data_price));
        }


        writes
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::InvoiceItem) -> Vec<i64> {
        // has_direct_event.
        // Assumption: API will block any deletes of invoiceitems for paid invoices (used as part of a transaction calculation).
        let mut x: Invoiceitem = data.into();
        vec![x.tx_delete_log_write(utx, run_id, "id")]
    }
}
