use unicon::uc::{*};
use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};

/// This timeline tests an invoice with more than 10 line items (items > 10 are excluded from events requiring custom logic).
static EVENT_SEQ_KEY: &'static str = "invoice_2";

#[tokio::main]
#[test]
#[should_panic]
async fn event_seq_invoice_2_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let _uc = exec.fork_uc();

    exec.dl("()");

    exec.apply("c").await;


    // let result = std::panic::catch_unwind(async ||  exec.apply("u").await);
    // use futures::FutureExt;
    // `error[E0277]: the type `&mut event_seq::Exec` may not be safely transferred across an unwind boundary` (this is caused by FFI code that must implement UnwindSafe - Rusqlite does not).
    // let x = async {
    //     exec.apply("u").await
    // };
    //
    // let x2 = x.catch_unwind().await;
    // assert!(x2.is_err());


    exec.apply("u").await

    // exec.apply("d").await;
}


#[tokio::main]
#[test]
#[should_panic]
async fn event_seq_invoice_2_walk_2() {
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
#[should_panic] // Note: event `invoice.deleted` contains has_more=true
async fn event_seq_invoice_2_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let _uc = exec.fork_uc();

        exec.dl("(u)");
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
#[should_panic] // Note: event `invoice.deleted` contains has_more=true
async fn event_seq_invoice_2_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");
    exec.apply("d").await;
}


/// Download-only, not apply_events.
/// - At download time, the extra list items are downloaded.
#[tokio::main]
#[test]
async fn event_seq_invoice_2_walk_3_dl_only() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let mut uc = exec.fork_uc();

        exec.dl("(u)");
        assert!(more_than_10_items_plus_relations_exist(&mut uc));
        // exec.apply("d").await;
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_b(es);
        let _uc = exec.fork_uc();

        exec.dl("(u)");
        // exec.apply("d-d").await;
    }
}


#[tokio::main]
#[test]
async fn event_seq_invoice_2_walk_4_dl_only() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let _uc = exec.fork_uc();

    exec.dl("(d)");
    // exec.apply("d").await;
}


fn more_than_10_items_plus_relations_exist(uc: &mut UniCon) -> bool {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            // language=sql
            let q = r#"
                SELECT
                    (
                        SELECT
                            sum(json_extract(je.value, "$.amount"))
                        FROM
                            json_each(i.total_discount_amounts) je
                    ) invoice_total_discounts,

                    (
                        SELECT
                        sum(json_extract(je.value, "$.amount"))
                        FROM
                            invoice_line_items ili,
                            json_each(ili.discount_amounts) je
                        WHERE
                            invoice = i.id
                    ) ili_total_discounts,

                    (select count(*) from invoiceitems where invoice=i.id) invoiceitem_count,

                    (select count(*) from invoice_line_items where invoice=i.id) invoice_line_items_count,

                    (select count(*) from discounts where invoice=i.id) discount_count

                FROM
                    invoices i;
            "#;

            let mut stmt = c.prepare(q).unwrap();
            let mut rows = stmt.query([]).unwrap();
            if let Some(row) = rows.next().unwrap() {
                let res: (
                    Option<i64>,
                    Option<i64>,
                    Option<i64>,
                    Option<i64>,
                    Option<i64>
                ) = (
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                    row.get(4).unwrap(),
                );

                match res {
                    (Some(a), Some(b), Some(c), Some(d), Some(e)) => {
                        let parent_child_sync = a == b;
                        let line_items_sync = c == d && c > 10;
                        let discounts_written = e == 2;

                        return parent_child_sync && line_items_sync && discounts_written;
                    }
                    _ => {}
                }
            }
            return false;
        }
        UniCon::PlaceholderLibA(_) => {}
        UniCon::MySQL(_) => {}
        UniCon::Postgres(_) => {}
    }
    unreachable!()
}






