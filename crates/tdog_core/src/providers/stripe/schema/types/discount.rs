use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use serde::{Deserialize, Deserializer, Serialize};
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCharge, UniCustomerC00F6E, UniDefaultSource, UniPaymentIntent, UniPaymentMethod, UniPromotionCode};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdFromEnumOrNone, json_key, json_string_or_none, PickOpt, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema_meta::LogWrite;
use crate::providers::traits::UpsertFirstLevel;

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct Discount {
    #[primary_key]
    pub discount_id: Option<i64>,

    #[unique]
    pub id: String,

    pub coupon: String,

    pub customer: Option<String>,
    pub subscription: Option<String>,
    pub invoice: Option<String>,
    pub invoice_item: Option<String>,

    pub promotion_code: Option<String>,
    pub checkout_session: Option<String>,

    pub start: DT,

    // `end` is a reserved keyword in Postgres.
    pub end_ts: Option<DT>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}


impl GetId for Discount {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::Discount> for Discount {
    fn from(x: &API::Discount) -> Self {
        Discount {
            discount_id: None,
            id: x.id.clone(),
            customer: x.customer.as_ref().and_then(|x2| {
                match x2 {
                    UniCustomerC00F6E::String(s) => s.clone().into(),
                    _ => unreachable!("Discount customer should be a string")
                }
            }),
            promotion_code: x.promotion_code.as_ref().and_then(|x2| {
                match x2 {
                    UniPromotionCode::String(s) => s.clone().into(),
                    _ => unreachable!("Discount promotion_code should be a string")
                }
            }),
            checkout_session: x.checkout_session.clone(),

            // These are has_dl_list (only when valid=true) + has_direct_events
            // Issue: when coupon.valid=false they are no longer listed in the download list so must be inserted as children of expanded discounts.
            coupon: x.coupon.id.clone(),

            end_ts: x.end.and_then(|x2| x2.to_dt().into()),
            invoice: x.invoice.clone(),
            invoice_item: x.invoice_item.clone(),
            start: x.start.to_dt(),
            subscription: x.subscription.clone(),
            insert_ts: None,
            update_ts: None,
        }
    }
}

impl WriteTree for Discount {
    type APIType = API::Discount;

    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Discount) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Discount = data.into();
        writes.push(x.tx_insert_set_pk_log_write(utx, run_id));

        // Upsert coupon (when expand=discount on invoice, invoiceitem, customer, or sub without expand).
        // Needed when: coupon.valid=false, so is not in direct dl list.
        let mut c: Coupon = (&data.coupon).into();
        writes.push(c.upsert_first_level(utx, run_id));

        writes
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Discount) -> Vec<i64> {
        let mut writes = vec![];
        let mut x: Discount = data.into();
        writes.push(x.upsert_first_level(utx, run_id));

        let mut c: Coupon = (&data.coupon).into();
        writes.push(c.upsert_first_level(utx, run_id));

        writes
    }

    // @see event_seq/all/discount.xlsx
    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::Discount) -> Vec<i64> {
        // Immutable objects that reference discounts: Coupons, Paid invoice/invoiceitems.
        // - Objects that inherit the discount contain just the discount id.
        //      - Queries may join (inherit row, discounts, coupons), even after the discount is deleted from the parent object.
        unimplemented!("No delete for Discount as it needs to be referenced from older invoices.")
    }
}