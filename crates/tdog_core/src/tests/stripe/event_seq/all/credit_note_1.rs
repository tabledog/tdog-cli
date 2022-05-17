use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};


static EVENT_SEQ_KEY: &'static str = "credit_note_1";


#[tokio::main]
#[test]
async fn event_seq_credit_note_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let uc = exec.fork_uc();

    exec.dl("()");

    


    exec.apply("c").await;

    


    exec.apply("u").await;

    written_from_event("credit_note", &uc);
    written_from_event("credit_note_line_item", &uc);


    // exec.apply("d").await;


    
}


#[tokio::main]
#[test]
async fn event_seq_credit_note_1_walk_2() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_a(es);
        let _uc = exec.fork_uc();


        exec.dl("(c)");

        

        exec.apply("u").await;
        // exec.apply("d").await;
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_b(es);
        let _uc = exec.fork_uc();


        exec.dl("(c)");

        

        exec.apply("u-u").await;
        // exec.apply("d").await;
    }
}


#[tokio::main]
#[test]
async fn event_seq_credit_note_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let uc = exec.fork_uc();

        exec.dl("(u)");
        inserted_from_dl("credit_note", &uc);
        inserted_from_dl("credit_note_line_item", &uc);



        // exec.apply("d").await;



    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_b(es);
        let _uc = exec.fork_uc();

        exec.dl("(u)");



        // exec.apply("d-d").await;

        // @todo/low Assert panic on applying event with has_more=true.

    }
}


#[tokio::main]
#[test]
async fn event_seq_credit_note_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");

    // @todo/low Assert 15 items inserted


    // exec.apply("d").await;
}

