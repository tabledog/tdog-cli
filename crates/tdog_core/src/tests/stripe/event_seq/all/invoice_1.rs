use unicon::uc::{*};
use crate::tests::stripe::event_seq::{EventSeq, WalksCUD};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};


/// This timeline tests basic invoice operations.
static EVENT_SEQ_KEY: &'static str = "invoice_1";

#[tokio::main]
#[test]
async fn event_seq_invoice_1_walk_1() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_1(es);
    let mut uc = exec.fork_uc();

    exec.dl("()");


    exec.apply("c").await;


    exec.apply("u").await;
    assert!(relations_4_exist(&mut uc));


    written_from_event("invoice", &uc);
    written_from_event("line_item", &uc);
    written_from_event("invoiceitem", &uc);


    exec.apply("d").await;
    assert!(!relations_4_exist(&mut uc));

    // Note: discount not deleted (it is owned by the invoice item which is deleted when the invoice is deleted).
    // - Discounts are kept immutable to keep the connection to coupons, which may be needed for historical queries on paid invoices.
}


#[tokio::main]
#[test]
async fn event_seq_invoice_1_walk_2() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_a(es);
        let mut uc = exec.fork_uc();


        exec.dl("(c)");


        exec.apply("u").await;
        assert!(relations_4_exist(&mut uc));


        exec.apply("d").await;
        assert!(!relations_4_exist(&mut uc));
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_2_b(es);
        let mut uc = exec.fork_uc();


        exec.dl("(c)");


        exec.apply("u-u").await;
        assert!(relations_4_exist(&mut uc));

        exec.apply("d").await;
        assert!(!relations_4_exist(&mut uc));
    }
}


#[tokio::main]
#[test]
async fn event_seq_invoice_1_walk_3() {
    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_a(es);
        let mut uc = exec.fork_uc();

        exec.dl("(u)");

        inserted_from_dl("invoice", &uc);
        inserted_from_dl("line_item", &uc);
        inserted_from_dl("invoiceitem", &uc);

        assert!(relations_4_exist(&mut uc));

        exec.apply("d").await;
        assert!(!relations_4_exist(&mut uc));
    }

    {
        let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
        let mut exec = WalksCUD::get_walk_3_b(es);
        let mut uc = exec.fork_uc();

        exec.dl("(u)");
        assert!(relations_4_exist(&mut uc));

        exec.apply("d-d").await;
        assert!(!relations_4_exist(&mut uc));
    }
}


#[tokio::main]
#[test]
async fn event_seq_invoice_1_walk_4() {
    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    let mut exec = WalksCUD::get_walk_4(es);
    let mut uc = exec.fork_uc();

    exec.dl("(d)");
    assert!(!relations_4_exist(&mut uc));

    exec.apply("d").await;
    assert!(!relations_4_exist(&mut uc));
}


fn relations_4_exist(uc: &mut UniCon) -> bool {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            // language=sql
            let q = r#"
                select i.id, ili.id, ii.id, d.id
                from
                invoices i left join invoice_line_items ili on(i.id = ili.invoice)
                left join invoiceitems ii, json_each(ii.discounts) je on(ili.invoice_item = ii.id)
                left join discounts d on(je.value = d.id)
            "#;

            let mut stmt = c.prepare(q).unwrap();
            let mut rows = stmt.query([]).unwrap();
            if let Some(row) = rows.next().unwrap() {
                let res: (
                    Option<String>,
                    Option<String>,
                    Option<String>,
                    Option<String>
                ) = (
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                    row.get(3).unwrap(),
                );

                match res {
                    (Some(_), Some(_), Some(_), Some(_)) => return true,
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
