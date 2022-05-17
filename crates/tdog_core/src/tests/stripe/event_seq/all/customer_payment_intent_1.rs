use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};


static EVENT_SEQ_KEY: &'static str = "customer_payment_intent_1";

// Note: BalanceTransactions
// Issues:
// - There are no cud events for this type, and charges just give you the string id.
// - But they can be listed at dl time.
// - How to keep them up to date after the first dl?
//      - Just save them to each of the objects tables they are contained inside of.
//          - Include in dl for users who just use the one time dl file to create reports.


/// @todo/low `charge.expired` and `charge.dispute.funds_reinstated` still need to be triggered (logic already implemented to handle those as updates).
#[tokio::main]
#[test]
async fn event_seq_customer_payment_intent_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let uc = exec.fork_uc();

    exec.dl("()");
    exec.apply("c").await;
    exec.apply("u").await;
    exec.apply("d").await;

    written_from_event("dispute", &uc);
}

#[tokio::main]
#[test]
async fn event_seq_customer_payment_intent_1_walk_2() {
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
async fn event_seq_customer_payment_intent_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let uc = exec.fork_uc();


        exec.dl("(u)");
        exec.apply("d").await;

        inserted_from_dl("dispute", &uc);
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
async fn event_seq_customer_payment_intent_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");
    exec.apply("d").await;
}


#[tokio::main]
#[test]
async fn event_seq_customer_payment_intent_1_walk_blank_db_apply() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_blank_db_apply_all(es);
        let _uc = exec.fork_uc();

        exec.dl("()");
        exec.apply("d").await;
    }


    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_blank_db_apply_u(es);
        let _uc = exec.fork_uc();

        exec.dl("()");
        exec.apply("u").await;
        exec.apply("d").await;
    }
}