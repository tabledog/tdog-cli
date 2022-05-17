use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};


static EVENT_SEQ_KEY: &'static str = "order_1";


#[tokio::main]
#[test]
async fn event_seq_order_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let uc = exec.fork_uc();

    exec.dl("()");


    exec.apply("c").await;


    // @todo/next Issue order_return.created -> charge.refunded.
    // - This does not issue a order_return.updated with order_return.refund=x (so it is always null).

    exec.apply("u").await;


    exec.apply("d").await;

    // Note: sku.delete can delete sku rows as sku's can only be deleted if not used in any orders.


    written_from_event("order", &uc);
    written_from_event("order_return", &uc);
    written_from_event("sku", &uc);
}


#[tokio::main]
#[test]
async fn event_seq_order_1_walk_2() {
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
async fn event_seq_order_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let uc = exec.fork_uc();

        exec.dl("(u)");


        inserted_from_dl("order", &uc);
        inserted_from_dl("order_return", &uc);
        inserted_from_dl("sku", &uc);


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
async fn event_seq_order_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");


    exec.apply("d").await;
}

