use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};


//use unicon::UniCon;

static EVENT_SEQ_KEY: &'static str = "tax_rate_1";


#[tokio::main]
#[test]
async fn event_seq_tax_rate_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let uc = exec.fork_uc();

    // Note: relations are checked after each `exec` for all objects.
    exec.dl("()");


    exec.apply("c").await;


    exec.apply("u").await;


    exec.apply("d").await;


    written_from_event("tax_rate", &uc);
    written_from_event("tax_id", &uc);
}


#[tokio::main]
#[test]
async fn event_seq_tax_rate_1_walk_2() {
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
async fn event_seq_tax_rate_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let uc = exec.fork_uc();

        exec.dl("(u)");


        inserted_from_dl("tax_rate", &uc);
        inserted_from_dl("tax_id", &uc);


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
async fn event_seq_tax_rate_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");


    exec.apply("d").await;


    // Note: `tax_ids` will not contain the tax_ids for the deleted customer as this is an customer-expanded property at dl time.
    // - But for the event stream, deleted customers do not delete their tax_ids.
}
