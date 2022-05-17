use rusqlite::{Connection};
use unicon::uc::{*};
use crate::providers::stripe::schema_meta::{TdStripeApplyEvent};
use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::util::{assert_type_action_map_tuple};

static EVENT_SEQ_KEY: &'static str = "customer_setup_intent_2";


#[tokio::main]
#[test]
async fn event_seq_customer_setup_intent_2_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let uc = exec.fork_uc();

    exec.dl("()");
    exec.apply("c").await;


    exec.apply("u").await;

    assert_status_count(&uc);

    exec.apply("d").await;


    let actions = TdStripeApplyEvent::test_get_actions_taken(&uc);
    let correct = vec![];


    assert_type_action_map_tuple(&correct, &actions);
}

#[tokio::main]
#[test]
async fn event_seq_customer_setup_intent_2_walk_2() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_a(es);
        let uc = exec.fork_uc();


        exec.dl("(c)");


        exec.apply("u").await;

        assert_status_count(&uc);


        exec.apply("d").await;
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_b(es);
        let uc = exec.fork_uc();


        exec.dl("(c)");

        exec.apply("u-u").await;

        assert_status_count(&uc);

        exec.apply("d").await;
    }
}

#[tokio::main]
#[test]
async fn event_seq_customer_setup_intent_2_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let uc = exec.fork_uc();


        exec.dl("(u)");

        assert_status_count(&uc);


        exec.apply("d").await;


        let actions = TdStripeApplyEvent::test_get_actions_taken(&uc);
        let correct = vec![];
        assert_type_action_map_tuple(&correct, &actions);
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_b(es);
        let uc = exec.fork_uc();


        exec.dl("(u)");


        exec.apply("d-d").await;


        let actions = TdStripeApplyEvent::test_get_actions_taken(&uc);
        let correct = vec![];
        assert_type_action_map_tuple(&correct, &actions);
    }
}


#[tokio::main]
#[test]
async fn event_seq_customer_setup_intent_2_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");
    exec.apply("d").await;
    // assert_all_skip(&uc);
}


/// Assert: All `setup_intent.*` events are converted into SQL writes.
/// - setup_intent.succeeded
/// - setup_intent.canceled
/// - setup_intent.requires_action
/// - setup_intent.setup_failed
///     - Will result in `status=^requires_*`
fn assert_status_count(uc: &UniCon) {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            let counts = get_status_counts(c);
            let expected = vec![
                (32, "succeeded".to_string()),
                (11, "requires_payment_method".to_string()),
                (6, "requires_confirmation".to_string()),
                (6, "requires_action".to_string())
            ];
            assert_eq!(expected, counts);
        }
        UniCon::PlaceholderLibA(_) => unreachable!(),
        UniCon::MySQL(_) => {}
        UniCon::Postgres(_) => {}
    }
}

fn get_status_counts(c: &Connection) -> Vec<(i32, String)> {
    let mut stmt = c.prepare("select count(*), status from setup_intents group by status order by count(*) DESC").unwrap();
    let mut rows = stmt.query([]).unwrap();

    let mut o = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let one: (i32, String) = (row.get(0).unwrap(), row.get(1).unwrap());
        o.push(one);
    }

    o
}