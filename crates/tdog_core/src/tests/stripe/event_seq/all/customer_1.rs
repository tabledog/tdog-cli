use unicon::uc::{*};
use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};
use crate::tests::stripe::util::{get_db_as_hm_by_test_id};

static EVENT_SEQ_KEY: &'static str = "customer_1";


#[tokio::main]
#[test]
async fn event_seq_customer_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let mut uc = exec.fork_uc();

    exec.dl("()");

    assert!(!customer_exists(&mut uc));


    exec.apply("c").await;

    assert!(customer_exists(&mut uc));


    exec.apply("u").await;
    exec.apply("d").await;


    written_from_event("customer", &uc);
}


#[tokio::main]
#[test]
async fn event_seq_customer_1_walk_2() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_a(es);
        let mut uc = exec.fork_uc();


        exec.dl("(c)");

        assert!(customer_exists(&mut uc));

        exec.apply("u").await;
        exec.apply("d").await;
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_b(es);
        let mut uc = exec.fork_uc();


        exec.dl("(c)");

        assert!(customer_exists(&mut uc));

        exec.apply("u-u").await;
        exec.apply("d").await;
    }
}


#[tokio::main]
#[test]
async fn event_seq_customer_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let mut uc = exec.fork_uc();

        exec.dl("(u)");
        inserted_from_dl("customer", &uc);

        assert!(customer_exists(&mut uc));

        exec.apply("d").await;

        assert!(!customer_exists(&mut uc));

    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_b(es);
        let mut uc = exec.fork_uc();

        exec.dl("(u)");

        assert!(customer_exists(&mut uc));

        exec.apply("d-d").await;

        assert!(!customer_exists(&mut uc));
    }
}


#[tokio::main]
#[test]
async fn event_seq_customer_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let mut uc = exec.fork_uc();

    exec.dl("(d)");

    assert!(!customer_exists(&mut uc));

    exec.apply("d").await;
}

fn customer_exists(mut uc: &mut UniCon) -> bool {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let c_1 = &("customers".to_string(), "c_1".to_string());
    hm.contains_key(c_1)
}
