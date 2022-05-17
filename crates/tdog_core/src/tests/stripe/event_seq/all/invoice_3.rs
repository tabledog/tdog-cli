use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};



/// This timeline tests moving invoice(s) between different states.
/// Note: `subscription_1` implicitly tests `invoice_line_items` which are (type=subscription)
static EVENT_SEQ_KEY: &'static str = "invoice_3";

#[tokio::main]
#[test]
async fn event_seq_invoice_3_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let _uc = exec.fork_uc();

    exec.dl("()");

    exec.apply("c").await;


    exec.apply("u").await;

    exec.apply("d").await;
}


#[tokio::main]
#[test]
async fn event_seq_invoice_3_walk_2() {
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
async fn event_seq_invoice_3_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let _uc = exec.fork_uc();

        exec.dl("(u)");

        // Note: `select obj_id, group_concat(write_type) from td_stripe_writes where "table_name"='invoice_line_items' group by obj_id`
        // Result: `il_1IlHr0Jo6Ja94JKPlozFGcM0`, `c,d,c,u,u,u,u,u`
        // - The lifetime of invoice line items may seem incorrect (create-delete-create-...) because a delete is inferred when applying a historical sequence of invoice.update events.
        // - Assumption: This is OK, as once apply_events gets to the final most recent event, the invoice_line_items will have the latest items.

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
async fn event_seq_invoice_3_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");
    exec.apply("d").await;
}







