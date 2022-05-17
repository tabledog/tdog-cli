use serde_json::Value;
use unicon::uc::{*};
use crate::tests::stripe::event_seq::{EventSeq, Exec, TagSeq};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};
use crate::tests::stripe::util::{get_db_as_hm_by_stripe_id, get_db_as_hm_by_test_id};

static EVENT_SEQ_KEY: &'static str = "subscription_1";


// @todo/next prices that are deleted or created inline, that are not in the list, should be upserted via the sub/invoice/other objects?


fn get_exec_c(path: &str) -> Exec {
    let ts: TagSeq = vec![
        "c".into(),
        "sub_c".into(),
    ].into();

    let _ = 1+1;

    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    Exec::from_path(es, ts, path)
}

fn get_exec_sub(path: &str) -> Exec {
    let ts: TagSeq = vec![
        "sub_c".into(),
        "sub_u".into(),
        "sub_d".into(),
        "sub_sched_c".into()
    ].into();

    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    Exec::from_path(es, ts, path)
}


fn get_exec_sched(path: &str) -> Exec {
    let ts: TagSeq = vec![
        "sub_sched_c".into(),
        "sub_sched_u".into(),
        "sub_sched_release".into(),
        "sub_sched_d".into(),
        "sub_item_c".into()
    ].into();

    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    Exec::from_path(es, ts, path)
}


fn get_exec_item(path: &str) -> Exec {
    let ts: TagSeq = vec![
        "sub_item_c".into(),
        "sub_item_u".into(),
        "sub_item_d".into(),
    ].into();

    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    Exec::from_path(es, ts, path)
}


/// Writing plans to the Stripe API results in the data being replicated to prices.
/// - Prices are newer; only write the prices to SQL.
fn assert_plan_write_mirrored_to_price(mut uc: &mut UniCon) {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let x = &("prices".to_string(), "pl_1".to_string());
    match hm.get(x).unwrap().get("id").unwrap() {
        Value::String(id) => {
            assert!(id.starts_with("plan_"));
            return;
        }
        _ => {}
    }
    unreachable!();
}


/// sub.items[].price_data creates a price inline, assert it is written to its own row.
fn assert_inline_price_written(mut uc: &mut UniCon) {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let x = &("subscriptions".to_string(), "s_1".to_string());
    match hm.get(x).unwrap().get("items").unwrap() {
        Value::Array(x2) => {
            match &x2[1] {
                Value::Object(x3) => {
                    if let Value::String(price_id) = x3.get("id").unwrap() {
                        let hm2 = get_db_as_hm_by_stripe_id(&mut uc);
                        let x = &("prices".to_string(), price_id.to_string());
                        if hm2.get(x).is_some() {
                            return;
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    unreachable!();
}


/// Creating new child objects inline with creation of the parent can be inconsistent (e.g. some are not listed via dl even after creation).
///
/// - Also: Implicitly test the relations.
/// - Represents an end user joining those tables.
fn assert_inline_price_and_product_written(uc: &mut UniCon) {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            let q = r#"
                select
                    (select count(*) from prices where id = (select price from subscription_items where id = (select json_extract(items, "$[0]") from subscriptions limit 1))) price,
                    (select count(*) from products where name = "Inline Product") product
            "#;

            let mut stmt = c.prepare(q).unwrap();
            let mut rows = stmt.query([]).unwrap();
            if let Some(row) = rows.next().unwrap() {
                let counts: (i64, i64) = (
                    row.get(0).unwrap(),
                    row.get(1).unwrap()
                );

                assert_eq!(counts, (1, 1));
                return;
            }
        }
        UniCon::PlaceholderLibA(_) => {}
        UniCon::MySQL(_) => {}
        UniCon::Postgres(_) => {}
    }
    unreachable!()
}


/// sub.items have a max of 20 items, and they are always all included in both download and events (never has_more=true).
fn assert_all_sub_items_written(uc: &mut UniCon) {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            let q = r#"
                select count(*) max from subscription_items group by subscription order by count(*) desc limit 1
            "#;

            let mut stmt = c.prepare(q).unwrap();
            let mut rows = stmt.query([]).unwrap();
            if let Some(row) = rows.next().unwrap() {
                let counts: i64 = row.get(0).unwrap();

                assert_eq!(counts, (20));
                return;
            }
        }
        UniCon::PlaceholderLibA(_) => {}
        UniCon::MySQL(_) => {}
        UniCon::Postgres(_) => {}
    }
    unreachable!()
}

/// Delete inferred as it is missing in the next sub.items list of a sub.update event.
fn assert_sub_item_deleted(uc: &mut UniCon) {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;

            let q = r#"
                select count(*) max from subscription_items group by subscription order by count(*) desc limit 1
            "#;

            let mut stmt = c.prepare(q).unwrap();
            let mut rows = stmt.query([]).unwrap();
            if let Some(row) = rows.next().unwrap() {
                let counts: i64 = row.get(0).unwrap();

                assert_eq!(counts, (19));
                return;
            }
        }
        UniCon::PlaceholderLibA(_) => {}
        UniCon::MySQL(_) => {}
        UniCon::Postgres(_) => {}
    }
    unreachable!()
}


#[tokio::main]
#[test]
async fn event_seq_subscription_1_c() {
    {
        let mut exec = get_exec_c("(),c,sub_c");
        let mut uc = exec.fork_uc();
        exec.dl("()");

        exec.apply("c").await;
        assert_plan_write_mirrored_to_price(&mut uc);

        // written_from_event("plan", &uc);
        written_from_event("price", &uc);
        written_from_event("product", &uc);
    }


    {
        let mut exec = get_exec_c("(c),sub_c");
        let mut uc = exec.fork_uc();
        exec.dl("(c)");
        assert_plan_write_mirrored_to_price(&mut uc);

        // inserted_from_dl("plan", &uc);
        inserted_from_dl("price", &uc);
        inserted_from_dl("product", &uc);
    }
}


#[tokio::main]
#[test]
async fn event_seq_subscription_1_sub() {
    {
        let mut exec = get_exec_sub("(),sub_c,sub_u,sub_d,sub_sched_c");
        let mut uc = exec.fork_uc();
        exec.dl("()");

        exec.apply("sub_c").await;
        assert_inline_price_and_product_written(&mut uc);

        exec.apply("sub_u").await;
        exec.apply("sub_d").await;
    }


    {
        let mut exec = get_exec_sub("(sub_c),sub_u,sub_d,sub_sched_c");
        let mut uc = exec.fork_uc();
        exec.dl("(sub_c)");
        assert_inline_price_and_product_written(&mut uc);

        exec.apply("sub_u").await;
        exec.apply("sub_d").await;
    }
}


#[tokio::main]
#[test]
async fn event_seq_subscription_1_sched() {
    {
        let mut exec = get_exec_sched("(),sub_sched_c,sub_sched_u,sub_sched_release,sub_sched_d,sub_item_c");
        let uc = exec.fork_uc();
        exec.dl("()");

        exec.apply("sub_sched_c").await;

        written_from_event("subscription_schedule", &uc);
        written_from_event("subscription", &uc);
        written_from_event("subscription_item", &uc);

        exec.apply("sub_sched_u").await;
        exec.apply("sub_sched_release").await;
        exec.apply("sub_sched_d").await;
    }


    {
        let mut exec = get_exec_sched("(sub_sched_c),sub_sched_u,sub_sched_release,sub_sched_d,sub_item_c");
        let uc = exec.fork_uc();
        exec.dl("(sub_sched_c)");

        inserted_from_dl("subscription_schedule", &uc);
        inserted_from_dl("subscription", &uc);
        inserted_from_dl("subscription_item", &uc);

        exec.apply("sub_sched_u").await;
        exec.apply("sub_sched_release").await;
        exec.apply("sub_sched_d").await;
    }


    {
        let mut exec = get_exec_sched("(sub_sched_u),sub_sched_release,sub_sched_d,sub_item_c");
        let _uc = exec.fork_uc();
        exec.dl("(sub_sched_u)");

        exec.apply("sub_sched_release").await;
        exec.apply("sub_sched_d").await;
    }

    {
        let mut exec = get_exec_sched("(sub_sched_release),sub_sched_d,sub_item_c");
        let _uc = exec.fork_uc();
        exec.dl("(sub_sched_release)");

        exec.apply("sub_sched_d").await;
    }

    {
        let mut exec = get_exec_sched("(sub_sched_d),sub_item_c");
        let _uc = exec.fork_uc();
        exec.dl("(sub_sched_d)");
    }
}


#[tokio::main]
#[test]
async fn event_seq_subscription_1_item() {
    {
        let mut exec = get_exec_item("(),sub_item_c,sub_item_u,sub_item_d");
        let mut uc = exec.fork_uc();
        exec.dl("()");

        exec.apply("sub_item_c").await;
        assert_all_sub_items_written(&mut uc);

        // Note:
        // - Adding subscription items to an existing parent sub creates "line items" ("add these to the next invoice").
        // - Adding subscription items inline with the parent sub creation *does not create line items*, they go directly to "invoice line item" with no "invoiceitem.created" event.
        //      - Test if "line items" are created for non-immediate payment/invoice sub creations.

        // Assert: There are 18 line items, and 2 missing line items:
        // - 1. The sub item created inline with sub parent does not have a line item (only an invoice line item attached to a paid invoice).
        // - 2. A zero-quantity sub item did not create a line item event, but is still listed in the sub items.

        exec.apply("sub_item_u").await;
        assert_sub_item_deleted(&mut uc);

        // @todo/next Calculate invoice total to ensure that it matches sub/sub item/price/product quantities.

        exec.apply("sub_item_d").await;
    }


    {
        let mut exec = get_exec_item("(sub_item_c),sub_item_u,sub_item_d");
        let mut uc = exec.fork_uc();

        exec.dl("(sub_item_c)");
        assert_all_sub_items_written(&mut uc);

        // dbg!(&exec.es);
        // dbg!(&exec.tag_seq);
        // dbg!(&exec.path);
        // dbg!(&exec.step);
        // dbg!(&exec.cur_pos);
        // dbg!(&exec.db_file);

        // This one is failing
        exec.apply("sub_item_u").await;
        assert_sub_item_deleted(&mut uc);

        exec.apply("sub_item_d").await;
    }


    {
        let mut exec = get_exec_item("(sub_item_u),sub_item_d");
        let mut uc = exec.fork_uc();
        exec.dl("(sub_item_u)");
        assert_sub_item_deleted(&mut uc);


        exec.apply("sub_item_d").await;
    }

    {
        let mut exec = get_exec_item("(sub_item_d)");
        let _uc = exec.fork_uc();
        exec.dl("(sub_item_d)");
    }
}







