use serde_json::{Value};
use unicon::uc::{*};
use crate::providers::stripe::schema_meta::{TdStripeApplyEvent};
use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};
use crate::tests::stripe::util::{assert_type_action_map_tuple, get_db_as_hm_by_test_id};

// Note: Some of the tests that test for joins/relations may not be needed as the more general `get_missing_owner` is checked after download/apply during development (added after these tests were written).
static EVENT_SEQ_KEY: &'static str = "customer_setup_intent_1";


#[tokio::main]
#[test]
async fn event_seq_customer_setup_intent_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let mut uc = exec.fork_uc();

    exec.dl("()");
    exec.apply("c").await;

    assert!(si_1_c_1_exist(&mut uc));
    assert!(!pm_1_exists(&mut uc));


    exec.apply("u").await;

    assert!(three_connected(&mut uc));

    exec.apply("d").await;


    assert!(si_1_pm_1_exist_no_c_1(&mut uc));


    written_from_event("setup_intent", &uc);
}

#[tokio::main]
#[test]
async fn event_seq_customer_setup_intent_1_walk_2() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_a(es);
        let mut uc = exec.fork_uc();


        exec.dl("(c)");

        assert!(si_1_c_1_exist(&mut uc));
        assert!(!pm_1_exists(&mut uc));

        exec.apply("u").await;

        assert!(three_connected(&mut uc));


        exec.apply("d").await;

        assert!(si_1_pm_1_exist_no_c_1(&mut uc));
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_b(es);
        let mut uc = exec.fork_uc();


        exec.dl("(c)");

        exec.apply("u-u").await;

        assert!(three_connected(&mut uc));

        exec.apply("d").await;

        assert!(si_1_pm_1_exist_no_c_1(&mut uc));
    }
}

#[tokio::main]
#[test]
async fn event_seq_customer_setup_intent_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let mut uc = exec.fork_uc();


        exec.dl("(u)");

        inserted_from_dl("setup_intent", &uc);

        assert!(three_connected(&mut uc));

        exec.apply("d").await;
        ;

        assert!(si_1_pm_1_exist_no_c_1(&mut uc));

        let actions = TdStripeApplyEvent::test_get_actions_taken(&uc);
        let correct = vec![];
        assert_type_action_map_tuple(&correct, &actions);
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_b(es);
        let mut uc = exec.fork_uc();


        exec.dl("(u)");

        assert!(three_connected(&mut uc));

        exec.apply("d-d").await;

        assert!(si_1_pm_1_exist_no_c_1(&mut uc));


        let actions = TdStripeApplyEvent::test_get_actions_taken(&uc);
        let correct = vec![];
        assert_type_action_map_tuple(&correct, &actions);
    }
}


#[tokio::main]
#[test]
async fn event_seq_customer_setup_intent_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let mut uc = exec.fork_uc();

    exec.dl("(d)");

    assert!(si_1_exists(&mut uc));

    // @todo/low Carefully document incomplete data sets to ensure query writers understand which sets are complete and which are not.
    // @incomplete-data When customer deleted payment_methods *may* be missing (missing when they have no payment_method.attached event).
    // Note: This should not be an issue if TD is watching the event stream as data is written (so no missing events) AND if payment_methods are attached to customers (indirectly happens when attached to intents).
    // assert!(!pm_1_exists(&mut uc));

    assert!(!c_1_exists(&mut uc));


    exec.apply("d").await;
    // assert_all_skip(&uc);


    // The `payment_method.attached` event when the customer was active is processed here and the payment_method is inserted (even though the customer has been deleted; this differs from download in w1 - w3 because the customer is active so the payment_method is downloaded with the customer).
    // - PaymentMethods can be inserted at:
    // - 1. Download: By being requested one-by-one whilst iterating over active customers.
    // - 2. Apply events: `payment_method.attached` *regardless of if customer is active*.
    //      - @todo/low Will this cause any issues by applying very old events?
    // Note: They are not inserted:
    //  - 1. Download, on SetupIntent downloaded by expanding payment_method (payment_method_option is a JSON object representing a snapshot of the payment method at the time of taking payment - the user can use this).
    assert!(si_1_pm_1_exist_no_c_1(&mut uc));


    let actions = TdStripeApplyEvent::test_get_actions_taken(&uc);
    let correct = vec![];
    assert_type_action_map_tuple(&correct, &actions);
}


fn si_1_c_1_exist(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let si_1 = &("setup_intents".to_string(), "si_1".to_string());
    let c_1 = &("customers".to_string(), "c_1".to_string());

    let s = hm.get(si_1).unwrap();
    let c = hm.get(c_1);

    c.is_some() &&
        // Nodes not yet connected.
        s.get("customer").unwrap().is_null()
}


fn si_1_pm_1_exist_no_c_1(mut uc: &mut UniCon) -> bool {
    // Intents are not deletable in Stripe; they are more like a log.
    si_1_exists(&mut uc) &&
        // PM still exists from when it was downloaded via active customer->payment_method (unlike in walk 4 where it is missing because the at download time the customer is inactive).
        pm_1_exists(&mut uc) &&
        !c_1_exists(&mut uc)
}


fn pm_1_exists(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let pm_1 = &("payment_methods".to_string(), "pm_1".to_string());
    let p = hm.get(pm_1);

    p.is_some()
}


fn si_1_exists(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let k = &("setup_intents".to_string(), "si_1".to_string());
    let v = hm.get(k);

    v.is_some()
}

fn c_1_exists(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let k = &("payment_methods".to_string(), "c_1".to_string());
    let v = hm.get(k);

    v.is_some()
}


fn pm_1_points_to_c_1(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let pm_1 = &("payment_methods".to_string(), "pm_1".to_string());
    let p = hm.get(pm_1).unwrap();

    p.get("customer").unwrap().is_string()
}

fn si_1_points_to_c_1_and_pm_1(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let si_1 = &("setup_intents".to_string(), "si_1".to_string());
    let s = hm.get(si_1).unwrap();

    s.get("customer").unwrap().is_string() &&
        s.get("payment_method").unwrap().is_string()
}

fn si_1_status_succeeded(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let si_1 = &("setup_intents".to_string(), "si_1".to_string());
    let s = hm.get(si_1).unwrap();

    if let Value::String(s) = s.get("status").unwrap() {
        return s == "succeeded";
    }
    false
}

fn three_connected(mut uc: &mut UniCon) -> bool {
    pm_1_points_to_c_1(&mut uc) &&
        si_1_points_to_c_1_and_pm_1(&mut uc) &&
        si_1_status_succeeded(&mut uc)
}






