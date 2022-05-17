use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use mysql::Params;
use mysql::prelude::Queryable;
use rusqlite::params;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCustomerC00F6E, UniDefaultSource, UniPaymentMethod, UniPromotionCode};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, json_key, json_string_or_none, Source, ToDT, ToJSONKeyOrNone, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::bank_account::BankAccount;
use crate::providers::stripe::schema::types::card::Card;
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema::types::tax_id::TaxId;
use crate::providers::stripe::schema_meta::{DeleteStaticLogWrite, GetInferredDeletes, LogWrite, TdStripeWrite};
use crate::providers::traits::{ExistsTx, ExistsTxSelf, UpsertFirstLevel};

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Customer {
    #[primary_key]
    pub customer_id: Option<i64>,

    #[unique]
    pub id: String,

    pub name: Option<String>,
    pub email: Option<String>,
    pub default_source: Option<String>,

    pub address: Option<Value>,
    pub shipping: Option<Value>,

    // https://stripe.com/docs/api/customer_balance_transactions?lang=node
    // Docs: `Each customer has a balance value, which denotes a debit or credit that's automatically applied to their next invoice upon finalization. You may modify the value directly by using the update customer API, or by creating a Customer Balance Transaction`
    // @todo/low Download customer_balance_transactions. Issue: !has_direct_events, only CreditNote contains these; cannot be kept up to date with events.
    // Temp fix: Ignore, users can use this balance field.
    pub balance: Option<i64>,
    pub currency: Option<String>,
    pub delinquent: Option<bool>,
    pub description: Option<String>,
    pub discount: Option<String>,


    // Query from `subscriptions` table using customer FK. (customer listUniCon is limited to 10 items).
    // pub subscriptions: Option<String>,

    pub invoice_prefix: Option<String>,

    pub invoice_settings: Option<Value>,

    pub next_invoice_sequence: Option<i64>,
    pub phone: Option<String>,


    pub preferred_locales: Option<Value>,

    // Query from `sources` table using customer FK.
    // pub sources: Option<ApmsSourcesSourceListC6CBF8>,

    pub tax_exempt: Option<String>,

    // Do not include this as it is not included by default (it will not exist on customer.updated events which violates download-event symmetry).
    // - Users should query from `tax_ids` table using the customer_id FK as this can be kept up to date with `customer.tax_id.x` events.
    // - Docs: This field is not included by default. To include set expand=tax_ids.
    // pub tax_ids: Option<String>,

    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl Customer {
    /// Keeps the `customer.discount` column up to date in response to `customer.discount.x` events where the discount owner is a customer.
    /// - `customer.updated` is not triggered when discount moves between a `string` and `null`.
    ///     - But subscription, invoice, invoiceitem all have their update events triggered for this same transition.
    ///
    /// - It is important to keep `customer.discount` up to date, as discounts rows are not deleted.
    ///     - Paid invoices/invoice items connect to coupons via discount ids; they are needed for historical reports.
    ///     - Users should join from parent->child (customers->discounts).
    ///
    /// - @see stripe-simulator/src/src/event_seq/all/discount.xlsx
    pub fn update_discount_id(utx: &mut UniTx, run_id: i64, event_type: &str, d: &API::Discount) -> i64 {

        // Assert: Discount owned by customer.
        let customer_id: String = match (&d.customer, &d.subscription, &d.invoice, &d.invoice_item) {
            (Some(UniCustomerC00F6E::String(x)), None, None, None) => x.clone(),
            _ => unreachable!("Discount {} is not owned by a customer, so customer.discount should not be updated.", &d.id)
        };


        let mut new_discount_id: Option<String> = match event_type {
            "customer.discount.created" | "customer.discount.updated" => d.id.clone().into(),
            "customer.discount.deleted" => None,
            _ => unreachable!("Incorrect event type, should be customer.discount.x")
        };

        // A customer must exist for add/remove discount operations (and events) to be triggered.
        assert!(Self::exists_tx(utx, &customer_id));

        let table = Self::get_table_name_static();
        let std_sql = format!("UPDATE {} SET discount=? WHERE id=?", table);
        let pg_sql = format!("UPDATE {} SET discount = $1 WHERE id = $2", table);

        // @todo/med Add the ability to update any set of cols using any where to the Rust SQL macro.
        match utx {
            UniTx::Rusqlite(tx) => {
                let mut stmt = tx.prepare_cached(&std_sql).unwrap();
                let changes = stmt.execute(params![&new_discount_id, &customer_id]).unwrap();
                assert_eq!(changes, 1);
            }
            UniTx::MySQL(tx) => {
                let params = Params::Positional(vec![new_discount_id.clone().into(), customer_id.clone().into()]);
                // @todo/next test this
                tx.exec_drop(&std_sql, params).unwrap();
                assert_eq!(tx.affected_rows(), 1);
            }
            UniTx::Postgres(tx) => {
                let changes = tx.execute(pg_sql.as_str(), &[&new_discount_id, &customer_id]).unwrap();
                assert_eq!(changes, 1);
            }
            UniTx::PlaceholderLibA(_) => {}
        }

        let mut write = TdStripeWrite {
            write_id: None,
            run_id,
            obj_type: Self::get_obj_type_static().to_string(),
            obj_id: customer_id,
            table_name: Self::get_table_name_static().to_string(),
            write_type: "u".to_string(),
            insert_ts: None,
        };

        write.tx_insert_set_pk(utx)
    }
}

impl GetId for Customer {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Customer> for Customer {
    fn from(x: &API::Customer) -> Self {
        Customer {
            customer_id: None,
            id: x.id.clone(),
            name: x.name.clone(),
            email: x.email.clone(),

            default_source: (*x.default_source).as_ref().and_then(|x2| {
                match x2 {
                    UniDefaultSource::String(s) => s.clone().into(),
                    _ => unreachable!("Expanded default_source on customer object, expected string. Sources are read from customer.sources.")
                }
            }),

            address: x.address.json_or_none(),
            shipping: x.shipping.json_or_none(),
            balance: x.balance.clone(),
            currency: x.currency.clone(),
            delinquent: x.delinquent.clone(),
            description: x.description.clone(),

            discount: x.discount.as_ref().and_then(|x| x.id.clone().into()),
            invoice_prefix: x.invoice_prefix.clone(),

            invoice_settings: x.invoice_settings.json_or_none(),
            next_invoice_sequence: x.next_invoice_sequence.clone(),
            phone: x.phone.clone(),
            preferred_locales: x.preferred_locales.json_or_none(),
            tax_exempt: x.tax_exempt.to_json_key_or_none(),
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),

            insert_ts: None,
            update_ts: None,

        }
    }
}


impl WriteTree for Customer {
    type APIType = API::Customer;

    /// - This is useful to compare the state transitions between (old:sources, new:payment intent).
    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Customer) -> Vec<i64> {
        let mut write_ids = vec![];
        let mut c2: Customer = data.into();

        // c2.insert_set_pk(&uc);
        write_ids.push(c2.tx_insert_set_pk_log_write(utx, run_id));


        // @todo/low Issue: Never set on events; can events keep sources up to date?
        if let Some(list) = &data.sources {
            for poly in &list.data {
                match poly {
                    UniPolymorphic646C3F::Source(x) => {
                        let mut rw: Source = x.into();
                        write_ids.push(rw.tx_insert_set_pk_log_write(utx, run_id));
                    }
                    UniPolymorphic646C3F::Card(x) => {
                        let mut rw: Card = x.as_ref().into();
                        write_ids.push(rw.tx_insert_set_pk_log_write(utx, run_id));
                    }
                    UniPolymorphic646C3F::BankAccount(x) => {
                        let mut rw: BankAccount = x.into();
                        write_ids.push(rw.tx_insert_set_pk_log_write(utx, run_id));
                    }
                    _ => {
                        dbg!(&poly);
                        unreachable!("Customer.sources.data contains a type that is not written to the SQL store (probably a Alipay or Bitcoin). Customer.id={}. This is currently a program exit to prevent invalid queries. These types may be written in the future, or a CLI option to acknowledge the missing writes will be added. Contact TD if you need to query these types.", &data.id);
                    }
                }
            }
        }

        // Upsert discounts (!has_dl_list) - these can also be written from subs, invoices and invoice items.
        if let Some(x) = &data.discount {
            write_ids.append(&mut Discount::upsert_tree(utx, run_id, &x));
        }


        // Insert tax ids.
        assert!(&data.tax_ids.is_some(), "insert_tree is called from the dl process which should expand tax_ids (so they do not need to be listed per customer using another request). customer.tax_id is not processed in upserts as this call will come from the event process which cannot expand that key.");
        let x2 = &data.tax_ids.as_ref().unwrap();
        assert!(!x2.has_more, "customer.tax_ids.has_more=true, it should be false as tax ids are limited to 5 per customer");
        for x3 in &x2.data {
            write_ids.append(&mut TaxId::insert_tree(utx, run_id, &x3));
        }


        write_ids
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Customer) -> Vec<i64> {
        let mut write_ids = vec![];
        let mut c2: Customer = data.into();

        write_ids.push(c2.upsert_first_level(utx, run_id));

        // Docs: `sources` is not included by default (always missing from events).
        assert!(&data.sources.is_none(), "Expected Customer.sources to be null or missing on a customer during event processing/upsert {}", &data.id);

        // Upsert discounts.
        if let Some(x) = &data.discount {
            write_ids.append(&mut Discount::upsert_tree(utx, run_id, &x));
        }

        write_ids
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Customer) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Customer = data.into();

        // Note: this will only be called via an event, which will not have `sources`.
        writes.push(x.tx_delete_log_write(utx, run_id, "id"));

        // @todo/next Look for other types without a delete event that need to be deleted here too.

        let deletes = TaxId::get_inferred_deleted_items(utx, "customer", &data.id, vec![]);
        for x in deletes {
            writes.push(TaxId::tx_delete_static_log_write(utx, run_id, &x));
        }


        writes
    }
}

