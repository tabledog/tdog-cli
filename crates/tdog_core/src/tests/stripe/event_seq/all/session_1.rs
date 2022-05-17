use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};



// Sessions are not writes.
// Issues:
//  - Violates the download/event symmetry.
//      - Sessions are created in the unpaid state; there is no create event.
//          - These sessions can be downloaded, resulting in only a partial dataset after applying the events, as the create events are missing.
//      - There are events that include fail/complete/succeed.
//          - Possible fix: Only download where state = paid, only apply events where state = paid.
//  - Removing for now, Sigma does not include sessions.
// @todo/low Ensure session events are skipped (cannot generate them from the API).
static EVENT_SEQ_KEY: &'static str = "session_1";

// @todo/low Test `unpaid` sessions are downloaded at the end of the apply_events process to ensure the state matches what can be downloaded (no create event triggered for initial creation of unpaid sessions).
// - Poll using a download at the end of the apply_events process.
// @todo/low Add session line items.

#[tokio::main]
#[test]
async fn event_seq_session_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let _uc = exec.fork_uc();

    exec.dl("()");


    exec.apply("c").await;


    exec.apply("u").await;

    // @todo/low Use Chrome API to generate a `completed` event.
    // written_from_event("checkout.session", &uc);

    // No `line_items`, see Session.line_items comment.


    exec.apply("d").await;
}


#[tokio::main]
#[test]
async fn event_seq_session_1_walk_2() {
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
async fn event_seq_session_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let _uc = exec.fork_uc();

        exec.dl("(u)");

        // inserted_from_dl("checkout.session", &uc);
        // No `line_items`, see Session.line_items comment.

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
async fn event_seq_session_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");
    exec.apply("d").await;
}

