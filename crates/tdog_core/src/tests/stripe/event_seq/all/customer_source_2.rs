use serde_json::{Value};
use unicon::uc::{*};
use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::util;

static EVENT_SEQ_KEY: &'static str = "customer_source_2";

#[tokio::main]
#[test]
async fn event_seq_customer_source_2_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let mut uc = exec.fork_uc();

    exec.dl("()");
    exec.apply("c").await;
    exec.apply("u").await;
    exec.apply("d").await;

    assert!(source_exists_and_chargeable(&mut uc));
}

#[tokio::main]
#[test]
async fn event_seq_customer_source_2_walk_2() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_a(es);
        let _uc = exec.fork_uc();

        exec.dl("(c)");
        exec.apply("u").await;
        exec.apply("d").await;
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_b(es);
        let _uc = exec.fork_uc();

        exec.dl("(c)");
        exec.apply("u-u").await;
        exec.apply("d").await;
    }
}


#[tokio::main]
#[test]
async fn event_seq_customer_source_2_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let mut uc = exec.fork_uc();

        exec.dl("(u)");

        // Exists from the download.
        assert!(source_exists_and_chargeable(&mut uc));

        exec.apply("d").await;
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_b(es);
        let _uc = exec.fork_uc();

        exec.dl("(u)");
        exec.apply("d-d").await;
    }
}


#[tokio::main]
#[test]
async fn event_seq_customer_source_2_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let mut uc = exec.fork_uc();

    exec.dl("(d)");

    // It is not in the download because the API requires access through a active customerId (customer deleted at this point).
    assert!(!source_exists_and_chargeable(&mut uc));


    exec.apply("d").await;

    // Now they exist because both source and payment_method have a C event (even though they have no active customer).
    assert!(source_exists_and_chargeable(&mut uc));
    // assert_all_skip(&uc);
}



fn source_exists_and_chargeable(mut uc: &mut UniCon) -> bool {
    let hm = util::get_db_as_hm_by_test_id(&mut uc);
    let s_1 = "s_1".to_string();

    let source_exists = hm.contains_key(&("sources".to_string(), s_1.clone()));
    let pm_exists = hm.contains_key(&("payment_methods".to_string(), s_1.clone()));


    let chargeable = source_exists && if let Value::String(s) = hm.get(&("sources".to_string(), s_1.clone())).unwrap().get("status").unwrap() {
        // Note: On the server, this will be `consumed`.
        // - Issue: After the customer this is attached to is deleted, the `Source` does not get any update events which means it cannot be kept up to date.
        // - It can no longer be downloaded via the API as the download URL is via the deleted customer?
        s == "chargeable"
    } else {
        false
    };

    source_exists &&
            pm_exists &&
            chargeable
}
