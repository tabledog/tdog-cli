use unicon::uc::{*};
use crate::providers::stripe::schema_meta::{ResActionsTaken, TdStripeApplyEvent};

//use unicon::{UniCon, UniTx};


pub mod tax_rate_1;
pub mod customer_payment_method_1;
pub mod customer_setup_intent_1;
pub mod customer_setup_intent_2;
pub mod customer_source_1;
pub mod customer_source_2;
pub mod customer_payment_intent_1;
pub mod charge_refund_1;
pub mod customer_payment_method_2;
pub mod subscription_1;
pub mod discount_2;
pub mod invoice_1;
pub mod invoice_2;
pub mod invoice_3;
pub mod credit_note_1;
pub mod session_1;
pub mod customer_1;
pub mod order_1;

// pub mod session_1;


/// All events that occur before a full account download should be skipped (because the data they contain exist in the downloaded objects).
fn assert_all_skip(uc: &UniCon) {
    let actions = TdStripeApplyEvent::test_get_actions_taken(&uc);


    for x in actions {
        if is_exception(&x) {
            continue;
        }

        assert!(x.action.starts_with("skip."), "{:?}", &x);
    }
}

/// General issues:
/// A. A parent object is deleted, which fires its `delete` event, but child object's are just `detached`.
///     - They still exist, but are not discoverable.
///     - They do not fire their `deleted` event, so the apply_events logic thinks they are create/updates.
///     - They can still be updated, just not re-attached or used.
fn is_exception(_x: &ResActionsTaken) -> bool {
    // payment_method
    // - When (customer created -> pm attached -> customer deleted -> first_dl -> first_apply_events).
    //      - pm is a write (not skip.event_before_dl) because:
    //          - They are never deleted, only un-listable.
    //          - They have their own "create" events ("attached"), meaning they do not need to have any extra logic looking for the parent in the event seq.
    //          - They can still be updated; they may be used for sending data via metadata (that users need to query).

    // if x.r#type.starts_with("payment_method.") || x.r#type.starts_with("customer.source.") {
    //     return true;
    // }


    false
}


/// Note: structure of function used to grep to get a complete list of types written from download.
fn inserted_from_dl(obj_type: &str, uc: &UniCon) {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            let mut stmt = c.prepare(format!("SELECT 1 FROM td_stripe_writes WHERE obj_type = :obj_type AND run_id = 1 AND write_type = 'c' LIMIT 1").as_str()).unwrap();
            let mut rows = stmt.query_named(&[(":obj_type", &obj_type)]).unwrap();
            if let Some(_row) = rows.next().unwrap() {
                assert!(true);
                return;
            }
            assert!(false);
            return;
        }
        UniCon::PlaceholderLibA(_) => {}
        UniCon::MySQL(_) => {}
        UniCon::Postgres(_) => {}
    }
    unreachable!()
}

/// Note: structure of function used to grep to get a complete list of types written from event list.
fn written_from_event(obj_type: &str, uc: &UniCon) {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            let mut stmt = c.prepare(format!("SELECT 1 FROM td_stripe_writes WHERE obj_type = :obj_type AND run_id > 1 LIMIT 1").as_str()).unwrap();
            let mut rows = stmt.query_named(&[(":obj_type", &obj_type)]).unwrap();
            if let Some(_row) = rows.next().unwrap() {
                assert!(true);
                return;
            }
            assert!(false);
            return;
        }
        UniCon::PlaceholderLibA(_) => {}
        UniCon::MySQL(_) => {}
        UniCon::Postgres(_) => {}
    }
    unreachable!()
}