use crate::providers::stripe::schema_meta::{TdStripeWrite};
use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};
use crate::tests::stripe::util::{assert_writes_eq};

static EVENT_SEQ_KEY: &'static str = "customer_source_1";


#[tokio::main]
#[test]
async fn event_seq_customer_source_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let uc = exec.fork_uc();

    exec.dl("()");
    exec.apply("c").await;
    exec.apply("u").await;
    exec.apply("d").await;

    written_from_event("source", &uc);
}

#[tokio::main]
#[test]
async fn event_seq_customer_source_1_walk_2() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_a(es);
        let uc = exec.fork_uc();

        exec.dl("(c)");

        // Objects exist from download.
        let writes = TdStripeWrite::get_all(&uc);
        let correct = vec![
            (1, "customers", "c"),
            (2, "sources", "c"),
            (3, "customers", "c"),
            (4, "payment_methods", "c")
        ];
        assert_writes_eq(&correct, &writes);


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
async fn event_seq_customer_source_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let uc = exec.fork_uc();

        exec.dl("(u)");

        inserted_from_dl("source", &uc);

        exec.apply("d").await;;


    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_b(es);
        let _uc = exec.fork_uc();

        exec.dl("(u)");
        exec.apply("d-d").await;;


    }
}


#[tokio::main]
#[test]
async fn event_seq_customer_source_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");
    exec.apply("d").await;
}

