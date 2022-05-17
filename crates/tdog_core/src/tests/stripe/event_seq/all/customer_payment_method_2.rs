use crate::tests::stripe::event_seq::{EventSeq, Exec};

static EVENT_SEQ_KEY: &'static str = "customer_payment_method_2";

#[tokio::main]
#[test]
async fn event_seq_customer_payment_method_2_test_1() {
    let tag_seq = vec!["a".into(), "b".into()];
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);


    {
        let mut exec = Exec::from_path(es, tag_seq, "(a),b-b");
        exec.dl("(a)");
        exec.apply("b-b").await;
    }

}
