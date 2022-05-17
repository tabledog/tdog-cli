use serde_json::Value;
use unicon::uc::{*};
use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};

use crate::tests::stripe::util::{get_db_as_hm_by_test_id};

static EVENT_SEQ_KEY: &'static str = "customer_payment_method_1";


#[tokio::main]
#[test]
async fn event_seq_customer_payment_method_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let _uc = exec.fork_uc();

    exec.dl("()");

    exec.apply("c").await;
    exec.apply("u").await;
    exec.apply("d").await;

}

#[tokio::main]
#[test]
async fn event_seq_customer_payment_method_1_walk_2() {

    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let exec = WalksCUD::get_walk_2_a(es);
    let _uc = exec.fork_uc();

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
async fn event_seq_customer_payment_method_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let _uc = exec.fork_uc();

        exec.dl("(u)");
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
async fn event_seq_customer_payment_method_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let mut uc = exec.fork_uc();

    exec.dl("(d)");

    assert!(pm_not_in_download(&mut uc));

    exec.apply("d").await;

    assert!(pm_inserted_from_event_with_update(&mut uc));
    // assert_all_skip(&uc);
}


lazy_static! {
    static ref PM_2: (String, String) = ("payment_methods".into(), "pm_2".into());
}

/// It is not in the download because payment_methods attached to customers (with no intent) can only be downloaded via an active customer.
fn pm_not_in_download(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    !hm.contains_key(&PM_2)
}

fn pm_inserted_from_event_with_update(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    if let Some(o1) = hm.get(&PM_2) {
        if let Value::Object(o2) = o1.get("metadata").unwrap() {
            return o2.contains_key("update_1");
        }
    }
    false
}
