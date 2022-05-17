use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCharge, UniCustomerC00F6E, UniDefaultSource, UniItems6F859C, UniPaymentIntent, UniPaymentMethod, UniPromotionCode, UniSubscription};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, GetIdFromEnumOrNone, json_key, json_string_or_none, PickOpt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema::types::invoice_line_item::{InvoiceLineItem, InvoiceLineItemWithParentId};
use crate::providers::stripe::schema_meta::{DeleteStaticLogWrite, GetInferredDeletes, LogWrite};
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Invoice {
    #[primary_key]
    pub invoice_id: Option<i64>,

    // Note: this will be null in the API type on a `invoice.upcoming` event.
    #[unique]
    pub id: String,

    // Issue: upcoming invoice has no ID, and "is not created yet"
    //      - This breaks the assumption that `id` is always set, which the write log is based on.
    // Fixes:
    // - A: is_upcoming=bool, with a composite `draft_$created_$customer` id
    // - B: invoices_upcoming table
    //      - invoiceitems?
    // - C: skip `invoice.upcoming` event, only deal with invoices with ids.
    //      - Fix later with B if needed.
    //      - Upcoming invoice seems more of a direct API interaction.
    //          - You could probably infer which customers/subs have upcoming invoices with a SQL query, and then interact with the API directly one be one, perhaps writing discounts/invoice items/dates to see what the response calculation is.
    //      - `invoice.upcoming` will still be in the `events` table so users can poll/react to it.




    pub amount_paid: i64,
    pub paid: bool,
    pub charge: Option<String>,
    pub customer: String,
    pub default_payment_method: Option<String>,
    pub default_source: Option<String>,

    // Note: `not populated if there are multiple discounts`
    // @deprecated Removed in favor of `discounts`.
    // pub discount: Option<String>,

    // #[skip]
    // pub discounts_expanded_skip: Option<Vec<Discount>>,

    pub discounts: Option<Value>,

    pub payment_intent: Option<String>,
    pub subscription: Option<String>,
    pub account_country: Option<String>,
    pub account_name: Option<String>,
    // @todo/low This was added in `2020-08-27`, re-gen Rust Stripe client.
    // account_tax_ids
    pub amount_due: i64,
    pub amount_remaining: i64,
    pub application_fee_amount: Option<i64>,
    pub attempt_count: i64,
    pub attempted: bool,
    pub auto_advance: Option<bool>,
    pub billing_reason: Option<String>,
    pub collection_method: Option<String>,
    pub currency: String,
    pub custom_fields: Option<Value>,
    pub customer_address: Option<Value>,
    pub customer_email: Option<String>,
    pub customer_name: Option<String>,
    pub customer_phone: Option<String>,
    pub customer_shipping: Option<Value>,
    pub customer_tax_exempt: Option<String>,
    pub customer_tax_ids: Option<Value>,
    pub default_tax_rates: Value,
    pub description: Option<String>,

    pub due_date: Option<DT>,
    pub ending_balance: Option<i64>,
    pub footer: Option<String>,
    pub hosted_invoice_url: Option<String>,
    pub invoice_pdf: Option<String>,
    pub last_finalization_error: Option<Value>,
    // (has_dl_list && has_direct_events) - query via invoiceitem table.

    // @todo/med There is a difference between InvoiceLineItem and InvoiceItem.
    // - Insert `InvoiceLineItem` here so that `discount_amounts` can be queried (this is the total discount per line item, and is missing on the `InvoiceItem`).
    //      - E.g. `discount_amounts: [ { amount: 500, discount: 'di_1IhHq8Jo6Ja94JKPRoAkJHKs' } ]`
    pub lines_newest_10: Value,

    pub next_payment_attempt: Option<DT>,
    pub number: Option<String>,
    pub period_end: DT,
    pub period_start: DT,
    pub post_payment_credit_notes_amount: i64,
    pub pre_payment_credit_notes_amount: i64,
    pub receipt_number: Option<String>,
    pub starting_balance: i64,
    pub statement_descriptor: Option<String>,
    pub status: Option<String>,
    pub status_transitions: Value,
    pub subscription_proration_date: Option<i64>,
    pub subtotal: i64,
    pub tax: Option<i64>,
    pub threshold_reason: Option<Value>,
    pub total: i64,
    pub total_discount_amounts: Option<Value>,
    pub total_tax_amounts: Value,
    pub transfer_data: Option<Value>,
    pub webhooks_delivered_at: Option<DT>,

    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetId for Invoice {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Invoice> for Invoice {
    fn from(x: &API::Invoice) -> Self {
        Invoice {
            invoice_id: None,

            id: match &x.id {
                Some(s) => s.clone(),
                None => unreachable!("Invoice id should always be a string (upcoming invoices that have no id should be skipped).")
            },

            amount_paid: x.amount_paid,
            paid: x.paid,
            charge: x.charge.to_json_key_or_none(),
            customer: if let UniCustomerC00F6E::String(s) = &x.customer {
                s.clone()
            } else {
                unreachable!("Customer on invoice should always be a string")
            },
            default_payment_method: x.default_payment_method.as_ref().and_then(|x2| x2.get_id_any().into()),
            default_source: (*x.default_source).as_ref().and_then(|x2| {
                if let UniDefaultSource::String(s) = &x2 {
                    return s.clone().into();
                }
                unreachable!("default_source on invoice should always be a string")
            }),


            // discounts_expanded_skip: x.discounts.and_then(|x2| {
            //     let mut o: Vec<Discount> = vec![];
            //     for x3 in x2 {
            //         match x3 {
            //             UniItems6F859C::String(_) => return None,
            //             // Only when (dl + expand=true).
            //             UniItems6F859C::Discount(d) => o.push((&d).into()),
            //             UniItems6F859C::DeletedDiscount(_) => unreachable!("Did not expect DeletedDiscount - this type is only for the return value from a delete URL. Invoices containing deleted discounts still contain the normal discount object.")
            //         }
            //     }
            //     o.into()
            // }),
            discounts: x.discounts.as_ref().and_then(|x2| {
                x2.iter().map(|x3| {
                    match x3 {
                        UniItems6F859C::String(s) => s.clone(),
                        UniItems6F859C::Discount(d) => d.id.clone(),
                        UniItems6F859C::DeletedDiscount(_) => unreachable!("Did not expect DeletedDiscount - this type is only for the return value from a delete URL. Invoices containing deleted discounts still contain the normal discount object.")
                    }
                }).collect::<Vec<String>>().json().into()

                // Note: `[]` for no discounts, not null (even though spec says null is possible it never is returned).
            }),
            payment_intent: x.payment_intent.as_ref().and_then(|x2| {
                match x2 {
                    UniPaymentIntent::String(s) => s.clone().into(),
                    UniPaymentIntent::PaymentIntent(_) => unreachable!("Expected payment_intent to be string id")
                }
            }),
            subscription: x.subscription.as_ref().and_then(|x2| match x2 {
                UniSubscription::String(s) => s.clone().into(),
                UniSubscription::Subscription(_) => unreachable!("Expected string id")
            }),
            account_country: x.account_country.clone(),
            account_name: x.account_name.clone(),
            amount_due: x.amount_due,
            amount_remaining: x.amount_remaining,
            application_fee_amount: x.application_fee_amount.clone(),
            attempt_count: x.attempt_count,
            attempted: x.attempted,
            auto_advance: x.auto_advance.clone(),
            billing_reason: x.billing_reason.to_json_key_or_none(),
            collection_method: x.collection_method.to_json_key_or_none(),
            currency: x.currency.clone(),
            custom_fields: x.custom_fields.json_or_none(),
            customer_address: x.customer_address.json_or_none(),
            customer_email: x.customer_email.clone(),
            customer_name: x.customer_name.clone(),
            customer_phone: x.customer_phone.clone(),
            customer_shipping: x.customer_shipping.json_or_none(),
            customer_tax_exempt: x.customer_tax_exempt.to_json_key_or_none(),
            customer_tax_ids: x.customer_tax_ids.json_or_none(),
            default_tax_rates: x.default_tax_rates.iter().map(|x2| x2.id.clone()).collect::<Vec<String>>().json(),
            description: x.description.clone(),
            due_date: x.due_date.as_ref().and_then(|x2| x2.to_dt().into()),
            ending_balance: x.ending_balance.clone(),
            footer: x.footer.clone(),
            hosted_invoice_url: x.hosted_invoice_url.clone(),
            invoice_pdf: x.invoice_pdf.clone(),
            last_finalization_error: (*x.last_finalization_error).as_ref().and_then(|x2| x2.json().into()),

            // Assumption: `InvoiceLineItem.price` is the same as `InvoiceItem.price` (so will already be upserted via `InvoiceItem.price` dl/event).
            lines_newest_10: x.lines.data.iter().map(|x2| x2.id.clone()).collect::<Vec<String>>().json(),
            next_payment_attempt: x.next_payment_attempt.as_ref().and_then(|x2| x2.to_dt().into()),
            number: x.number.clone(),
            period_end: x.period_end.to_dt(),
            period_start: x.period_start.to_dt(),
            post_payment_credit_notes_amount: x.post_payment_credit_notes_amount,
            pre_payment_credit_notes_amount: x.pre_payment_credit_notes_amount,
            receipt_number: x.receipt_number.clone(),
            starting_balance: x.starting_balance,
            statement_descriptor: x.statement_descriptor.clone(),
            status: x.status.to_json_key_or_none(),
            status_transitions: x.status_transitions.json(),
            subscription_proration_date: x.subscription_proration_date.clone(),
            subtotal: x.subtotal,
            tax: x.tax.clone(),
            threshold_reason: x.threshold_reason.json_or_none(),
            total: x.total,
            total_discount_amounts: x.total_discount_amounts.as_ref().and_then(|x2| x2.json().into()),
            total_tax_amounts: x.total_tax_amounts.json(),
            transfer_data: x.transfer_data.as_ref().and_then(|x2| x2.json().into()),
            webhooks_delivered_at: x.webhooks_delivered_at.and_then(|x2| x2.to_dt().into()),
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

fn upsert_discounts(utx: &mut UniTx, run_id: i64, data: &API::Invoice, w: &mut Vec<i64>) {
    if let Some(all) = &data.discounts {
        for mut d in all {
            match d {
                UniItems6F859C::Discount(d2) => {
                    // When: dl + expand (discounts are !has_dl_list + has_direct_event).
                    w.append(&mut Discount::upsert_tree(utx, run_id, &d2));
                }
                UniItems6F859C::String(_) => return,
                UniItems6F859C::DeletedDiscount(_) => return
            }
        }
    }
}


/// invoice_line_items = !has_direct_dl (must use invoice id) && !has_direct_events (only invoice.update events with items limited to newest 10).
fn delete_missing_lines_log_writes(utx: &mut UniTx, run_id: i64, data: &API::Invoice, writes: &mut Vec<i64>) {
    let active_items = data.lines.data.iter().map(|x| x.id.as_str()).collect();
    let deletes = InvoiceLineItem::get_inferred_deleted_items(utx, "invoice", &data.id.as_ref().unwrap(), active_items);
    for x in deletes {
        writes.push(InvoiceLineItem::tx_delete_static_log_write(utx, run_id, &x));
    }
}


impl WriteTree for Invoice {
    type APIType = API::Invoice;

    /// @see https://stripe.com/docs/payments/payment-intents/migration#saved-cards
    /// - This is useful to compare the state transitions between (old:sources, new:payment intent).
    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Invoice) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Invoice = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));


        if let Some(x2) = &data.default_payment_method {
            match x2 {
                UniPaymentMethod::String(_) => unreachable!("Invoice.default_payment_method should always be expanded at dl time."),
                UniPaymentMethod::PaymentMethod(x3) => {
                    // Upsert, as these are listed one per *active* customer at dl time via API call (custId, paymentMethodType).
                    // - Deleted customers are not listed at dl times, so their attached payment methods are not inserted.
                    writes.append(&mut PaymentMethod::upsert_tree(utx, run_id, &x3));
                }
            }
        }

        // This needs to be an upsert because the same discount_id can be used for many invoices (for the same customer), and discounts cannot be listed at dl time.
        upsert_discounts(utx, run_id, &data, &mut writes);


        // Insert Invoice Line Items
        if data.lines.has_more {
            // At dl time, the entire set is downloaded. At event process time, a panic occurs.
        }
        for x2 in &data.lines.data {
            // writes.append(&mut InvoiceLineItem::insert_tree(utx, run_id, &x2));

            let x3 = InvoiceLineItemWithParentId {
                parent: data.id.as_ref().unwrap().clone(),
                data: &x2,
            };

            writes.append(&mut InvoiceLineItemWithParentId::insert_tree(utx, run_id, &x3));
        }


        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Invoice) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Invoice = data.into();

        writes.push(x.upsert_first_level(utx, run_id));


        // Not needed as events always contain string id, never expanded objects.
        // if let Some(x2) = &data.default_payment_method ...

        upsert_discounts(utx, run_id, &data, &mut writes);


        // Upsert Invoice Line Items

        // Treat invoice_line_items like sub items for now (subs contain all sub items up to a max of 20, has_more=always false).
        // At dl time, the entire set is downloaded. At event process time, a panic occurs (see comment in apply_events).
        assert!(!data.lines.has_more, "has_more=true for Invoice.id={}. This should never be called during download, and another assert triggers before this one when processing events.", &data.id.as_ref().unwrap());
        delete_missing_lines_log_writes(utx, run_id, &data, &mut writes);

        for x2 in &data.lines.data {
            // writes.append(&mut InvoiceLineItem::upsert_tree(utx, run_id, &x2));

            let x3 = InvoiceLineItemWithParentId {
                parent: data.id.as_ref().unwrap().clone(),
                data: &x2,
            };

            writes.append(&mut InvoiceLineItemWithParentId::upsert_tree(utx, run_id, &x3));
        }


        writes
    }


    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Invoice) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Invoice = data.into();

        // Delete children owned by this parent.
        // @todo/low Check other child types are deleted when their parents are too.
        assert!(!data.lines.has_more); // Note: limited to < 10
        for x in &data.lines.data {
            writes.push(InvoiceLineItem::tx_delete_static_log_write(utx, run_id, &x.id));
        }

        writes.push(x.tx_delete_log_write(utx, run_id, "id"));

        writes
    }
}