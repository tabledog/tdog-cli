use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};


static EVENT_SEQ_KEY: &'static str = "charge_refund_1";



#[tokio::main]
#[test]
async fn event_seq_charge_refund_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let uc = exec.fork_uc();

    exec.dl("()");

    exec.apply("c").await;
    exec.apply("u").await;
    exec.apply("d").await;


    written_from_event("payment_method", &uc);
    written_from_event("payment_intent", &uc);
    written_from_event("charge", &uc);
    written_from_event("refund", &uc);
}


#[tokio::main]
#[test]
async fn event_seq_charge_refund_1_walk_2() {
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
async fn event_seq_charge_refund_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let uc = exec.fork_uc();

        exec.dl("(u)");

        inserted_from_dl("payment_method", &uc);
        inserted_from_dl("payment_intent", &uc);
        inserted_from_dl("charge", &uc);
        inserted_from_dl("refund", &uc);

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
async fn event_seq_charge_refund_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");
    exec.apply("d").await;
    // assert_all_skip(&uc);
}


#[tokio::main]
#[test]
async fn event_seq_charge_refund_1_apply_all() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_blank_db_apply_all(es);
    let _uc = exec.fork_uc();

    exec.dl("()");
    exec.apply("d").await;


    // let actions = TdStripeApplyEvent::test_get_actions_taken(&uc);
    // let correct = vec![
    //     ("customer", "customer.created", "skip.deleted_before_dl"),
    //     ("payment_method", "payment_method.attached", "write.c"),
    //     ("payment_intent", "payment_intent.created", "skip.not_last_write"),
    //     ("charge", "charge.succeeded", "skip.not_last_write"),
    //     ("payment_intent", "payment_intent.succeeded", "write.c"),
    //     ("charge", "charge.refunded", "skip.not_last_write"),
    //     ("charge", "charge.refunded", "skip.not_last_write"),
    //     ("refund", "charge.refund.updated", "skip.not_last_write"),
    //     ("refund", "charge.refund.updated", "skip.not_last_write"),
    //     // Assert: This write is converted to a skip as the parent charge write exists later.
    //     ("refund", "charge.refund.updated", "skip.parent_write_exists_later"),
    //     ("charge", "charge.updated", "write.c"),
    //     ("customer", "customer.deleted", "skip.deleted_before_dl")
    // ];
    // assert_type_action_map_tuple(&correct, &actions);
}
