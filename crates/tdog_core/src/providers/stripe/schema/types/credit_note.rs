use std::collections::HashMap;
//use unicon::dt3::DT3;
//use unicon::dt::DT;
use std::hash::BuildHasherDefault;

use rusqlite::params;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::responses::UniPolymorphic646C3F;
use stripe_client::types::types::{GetId, UniCustomerBalanceTransaction, UniCustomerC00F6E, UniDefaultSource, UniPaymentMethod, UniPromotionCode, UniRefund};
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

use crate::providers::stripe::schema::{ArrayPKs, f, f_opt, GetIdAny, json_key, json_string_or_none, Source, ToDT, ToJSONKey, ToJSONKeyOrNone, ToVal, ToValOrNone, unix_to_iso, WriteTree, x};
use crate::providers::stripe::schema::types::{*};
use crate::providers::stripe::schema::types::credit_note_line_item::{CreditNoteLineItem, CreditNoteLineItemWithParentId};
use crate::providers::stripe::schema::types::discount::Discount;
use crate::providers::stripe::schema_meta::{DeleteStaticLogWrite, GetInferredDeletes, LogWrite, TdStripeWrite};
use crate::providers::traits::{ExistsTx, ExistsTxSelf, UpsertFirstLevel};

//use unicon_proc_macro::{Db, Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct CreditNote {
    #[primary_key]
    pub credit_note_id: Option<i64>,

    #[unique]
    pub id: String,
    pub r#type: String,

    pub customer: String,
    pub customer_balance_transaction: Option<String>,
    pub invoice: String,
    pub refund: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub discount_amount: i64,
    pub discount_amounts: Value,

    // Lines = (has_direct_dl && !has_direct_events) - this has the same issues as the invoice line items (>10 in an event requires a download).
    pub lines_first_x: Value,
    pub memo: Option<String>,
    pub number: String,
    pub out_of_band_amount: Option<i64>,
    pub pdf: String,
    pub reason: Option<String>,
    pub status: String,
    pub subtotal: i64,
    pub tax_amounts: Value,
    pub total: i64,
    pub voided_at: Option<DT>,

    pub created: DT,
    pub livemode: bool,


    pub metadata: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,

    #[update_ts]
    pub update_ts: Option<DT3>,
}

impl GetObjType for CreditNote {
    fn get_obj_type_static() -> &'static str {
        "credit_note"
    }
}

impl GetId for CreditNote {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl From<&API::CreditNote> for CreditNote {
    fn from(x: &API::CreditNote) -> Self {
        let x2 = x.clone();

        CreditNote {
            credit_note_id: None,
            id: x2.id,

            r#type: x2.type_x.to_json_key(),
            customer: x2.customer.get_id_any(),
            customer_balance_transaction: x2.customer_balance_transaction.and_then(|x3| match x3 {
                UniCustomerBalanceTransaction::String(x4) => x4,
                UniCustomerBalanceTransaction::CustomerBalanceTransaction(x4) => x4.id,
            }.into()),
            invoice: x2.invoice.get_id_any(),
            refund: x2.refund.and_then(|x3| match x3 {
                UniRefund::String(x4) => x4,
                UniRefund::Refund(x4) => x4.id
            }.into()),
            amount: x2.amount,
            currency: x2.currency,
            discount_amount: x2.discount_amount,
            discount_amounts: x2.discount_amounts.json(),
            lines_first_x: x2.lines.data.iter().map(|x3| x3.id.clone()).collect::<Vec<String>>().json(),
            memo: x2.memo,
            number: x2.number,
            out_of_band_amount: x2.out_of_band_amount,
            pdf: x2.pdf,
            reason: x2.reason.and_then(|x3| x3.to_json_key().into()),
            status: x2.status.to_json_key(),
            subtotal: x2.subtotal,
            tax_amounts: x2.tax_amounts.json(),
            total: x2.total,
            voided_at: x2.voided_at.and_then(|x3| x3.to_dt().into()),
            created: x.created.to_dt(),
            livemode: x.livemode,
            metadata: x.metadata.json_or_none(),

            insert_ts: None,
            update_ts: None,

        }
    }
}


// @todo/low Note: it appears that the `credit_note.lines` property is immutable after creation, so inferred deletes are not needed?
fn delete_missing_lines_log_writes(utx: &mut UniTx, run_id: i64, data: &API::CreditNote, writes: &mut Vec<i64>) {
    let active_items = data.lines.data.iter().map(|x| x.id.as_str()).collect();
    let deletes = CreditNoteLineItem::get_inferred_deleted_items(utx, "credit_note_id", &data.id, active_items);
    for x in deletes {
        writes.push(CreditNoteLineItem::tx_delete_static_log_write(utx, run_id, &x));
    }
}


impl WriteTree for CreditNote {
    type APIType = API::CreditNote;


    fn insert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::CreditNote) -> Vec<i64> {
        let mut w = vec![];
        let mut x: CreditNote = data.into();

        w.push(x.tx_insert_set_pk_log_write(utx, run_id));


        for l in &data.lines.data {
            let x2 = CreditNoteLineItemWithParentId {
                parent: x.id.clone(),
                data: &l,
            };

            w.append(&mut CreditNoteLineItemWithParentId::insert_tree(utx, run_id, &x2));
        }

        w
    }

    fn upsert_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::CreditNote) -> Vec<i64> {
        let mut w = vec![];

        let mut x: CreditNote = data.into();
        w.push(x.upsert_first_level(utx, run_id));


        // Infer deletes (may not be needed as lines is immutable after creation - left in, in case edits are allowed in the future).
        // Note: modelled after invoice.
        assert!(!data.lines.has_more, "has_more=true for CreditNote.id={}. This should never be called during download, and another assert triggers before this one when processing events.", &data.id);
        delete_missing_lines_log_writes(utx, run_id, &data, &mut w);

        for x2 in &data.lines.data {
            let x3 = CreditNoteLineItemWithParentId {
                parent: x.id.clone(),
                data: &x2,
            };

            w.append(&mut CreditNoteLineItemWithParentId::upsert_tree(utx, run_id, &x3));
        }

        w
    }

    fn delete_tree<'a>(utx: &mut UniTx<'a>, run_id: i64, data: &API::CreditNote) -> Vec<i64> {
        unimplemented!("CreditNote's are not deleted, only voided - which is an update.")
    }
}

