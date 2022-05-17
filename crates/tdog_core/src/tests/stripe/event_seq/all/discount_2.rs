use serde_json::{Map, Value};
use unicon::uc::{*};
use crate::tests::stripe::event_seq::{EventSeq, Exec, TagSeq};
use crate::tests::stripe::event_seq::all::{inserted_from_dl, written_from_event};
use crate::tests::stripe::util::{get_db_as_hm_by_test_id};

// This test makes sure attaching/detaching discounts from parent objects works correctly for both the download and apply_events.
// - At any point in the event timeline, a download database, and an apply_events database should yield the same query results for queries joining discounts.
//      - This test essentially ensures the (download, apply_events) join is consistent.
static EVENT_SEQ_KEY: &'static str = "discount_2";


fn get_exec_cust(path: &str) -> Exec {
    let ts: TagSeq = vec![
        "cust_attach".into(),
        "cust_dettach".into(),
    ].into();


    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    Exec::from_path(es, ts, path)
}

fn get_exec_sub(path: &str) -> Exec {
    let ts: TagSeq = vec![
        "sub_attach".into(),
        "sub_dettach".into(),
    ].into();


    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    Exec::from_path(es, ts, path)
}


fn get_exec_inv(path: &str) -> Exec {
    let ts: TagSeq = vec![
        "inv_attach".into(),
        "inv_dettach".into(),
    ].into();


    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    Exec::from_path(es, ts, path)
}


fn get_exec_invitem(path: &str) -> Exec {
    let ts: TagSeq = vec![
        "invitem_attach".into(),
        "invitem_dettach".into(),
    ].into();


    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    Exec::from_path(es, ts, path)
}


fn get_exec_promo(path: &str) -> Exec {
    let ts: TagSeq = vec![
        "promo_c".into(),
        "promo_u".into(),
    ].into();


    let es = EventSeq::from_local_dir(&EVENT_SEQ_KEY);
    Exec::from_path(es, ts, path)
}


fn get_customer(mut uc: &mut UniCon) -> Option<Map<String, Value>> {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let x = &("customers".to_string(), "c_1".to_string());
    // let x: Option<&Map<String, Value>> =
    hm.get(x).and_then(|x| x.clone().into())
}

fn get_sub(mut uc: &mut UniCon) -> Option<Map<String, Value>> {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let x = &("subscriptions".to_string(), "s_1".to_string());
    // let x: Option<&Map<String, Value>> =
    hm.get(x).and_then(|x| x.clone().into())
}

fn get_inv(mut uc: &mut UniCon) -> Option<Map<String, Value>> {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let x = &("invoices".to_string(), "i_1".to_string());
    // let x: Option<&Map<String, Value>> =
    hm.get(x).and_then(|x| x.clone().into())
}

fn get_invitem(mut uc: &mut UniCon) -> Option<Map<String, Value>> {
    let hm = get_db_as_hm_by_test_id(&mut uc);
    let x = &("invoiceitems".to_string(), "ii_2".to_string());
    // let x: Option<&Map<String, Value>> =
    hm.get(x).and_then(|x| x.clone().into())
}


fn has_discount(x: Map<String, Value>) -> bool {
    match x.get("discount") {
        Some(x2) => match x2 {
            Value::String(_) => return true,
            _ => return false
        },
        None => {}
    }
    unreachable!("Should have discount field set to a string.");
}


fn get_discounts(x: Map<String, Value>) -> Option<Vec<Value>> {
    match x.get("discounts") {
        Some(x2) => match x2 {
            Value::Array(x) => return Some(x.clone()),
            _ => return None
        },
        None => {}
    }
    unreachable!("Should have discounts field set to an array.");
}


#[tokio::main]
#[test]
async fn event_seq_discount_2_cust_owner() {

    // @see event_seq/all/discount.xlsx


    {
        let mut exec = get_exec_cust("(),cust_attach,cust_dettach");
        let mut uc = exec.fork_uc();
        exec.dl("()");

        exec.apply("cust_attach").await;

        written_from_event("coupon", &uc);
        written_from_event("discount", &uc);

        {
            let c = get_customer(&mut uc);
            assert!(has_discount(c.unwrap()));
        }

        exec.apply("cust_dettach").await;

        {
            let c = get_customer(&mut uc);
            assert!(!has_discount(c.unwrap()));
        }
    }


    {
        let mut exec = get_exec_cust("(cust_attach),cust_dettach");
        let mut uc = exec.fork_uc();

        exec.dl("(cust_attach)");

        inserted_from_dl("coupon", &uc);
        inserted_from_dl("discount", &uc);

        {
            let c = get_customer(&mut uc);
            assert!(has_discount(c.unwrap()));
        }

        exec.apply("cust_dettach").await;

        {
            let c = get_customer(&mut uc);
            assert!(!has_discount(c.unwrap()));
        }
    }


    {
        let mut exec = get_exec_cust("(cust_dettach)");
        let mut uc = exec.fork_uc();
        exec.dl("(cust_dettach)");

        {
            let c = get_customer(&mut uc);
            assert!(!has_discount(c.unwrap()));
        }
    }
}


#[tokio::main]
#[test]
async fn event_seq_discount_2_sub_owner() {
    {
        let mut exec = get_exec_sub("(),sub_attach,sub_dettach");
        let mut uc = exec.fork_uc();
        exec.dl("()");

        exec.apply("sub_attach").await;

        {
            let x = get_sub(&mut uc);
            assert!(has_discount(x.unwrap()));
        }

        exec.apply("sub_dettach").await;

        {
            let x = get_sub(&mut uc);
            assert!(!has_discount(x.unwrap()));
        }
    }


    {
        let mut exec = get_exec_sub("(sub_attach),sub_dettach");
        let mut uc = exec.fork_uc();

        exec.dl("(sub_attach)");

        {
            let x = get_sub(&mut uc);
            assert!(has_discount(x.unwrap()));
        }

        exec.apply("sub_dettach").await;

        {
            let x = get_sub(&mut uc);
            assert!(!has_discount(x.unwrap()));
        }
    }


    {
        let mut exec = get_exec_sub("(sub_dettach)");
        let mut uc = exec.fork_uc();
        exec.dl("(sub_dettach)");

        {
            let x = get_sub(&mut uc);
            assert!(!has_discount(x.unwrap()));
        }
    }
}


#[tokio::main]
#[test]
async fn event_seq_discount_2_inv_owner() {
    {
        let mut exec = get_exec_inv("(),inv_attach,inv_dettach");
        let mut uc = exec.fork_uc();
        exec.dl("()");

        exec.apply("inv_attach").await;

        {
            let x = get_inv(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 2);
        }

        exec.apply("inv_dettach").await;

        {
            let x = get_inv(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 0);
        }
    }


    {
        let mut exec = get_exec_inv("(inv_attach),inv_dettach");
        let mut uc = exec.fork_uc();

        exec.dl("(inv_attach)");

        {
            let x = get_inv(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 2);
        }

        exec.apply("inv_dettach").await;

        {
            let x = get_inv(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 0);
        }
    }


    {
        let mut exec = get_exec_inv("(inv_dettach)");
        let mut uc = exec.fork_uc();
        exec.dl("(inv_dettach)");

        {
            let x = get_inv(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 0);
        }
    }
}


#[tokio::main]
#[test]
async fn event_seq_discount_2_invitem_owner() {
    {
        let mut exec = get_exec_invitem("(),invitem_attach,invitem_dettach");
        let mut uc = exec.fork_uc();
        exec.dl("()");

        exec.apply("invitem_attach").await;

        {
            let x = get_invitem(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 2);
        }

        exec.apply("invitem_dettach").await;

        {
            let x = get_invitem(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 0);
        }
    }


    {
        let mut exec = get_exec_invitem("(invitem_attach),invitem_dettach");
        let mut uc = exec.fork_uc();

        exec.dl("(invitem_attach)");

        {
            let x = get_invitem(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 2);
        }

        exec.apply("invitem_dettach").await;

        {
            let x = get_invitem(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 0);
        }
    }


    {
        let mut exec = get_exec_invitem("(invitem_dettach)");
        let mut uc = exec.fork_uc();
        exec.dl("(invitem_dettach)");

        {
            let x = get_invitem(&mut uc);
            assert_eq!(get_discounts(x.unwrap()).unwrap().len(), 0);
        }
    }
}


#[tokio::main]
#[test]
async fn event_seq_discount_2_promo() {
    {
        let mut exec = get_exec_promo("(),promo_c,promo_u");
        let uc = exec.fork_uc();
        exec.dl("()");
        assert_eq!(discount_count_promo_code_set(&uc), 0);


        exec.apply("promo_c").await;

        written_from_event("promotion_code", &uc);

        assert_eq!(discount_count_promo_code_set(&uc), 0);

        exec.apply("promo_u").await;
        assert_eq!(discount_count_promo_code_set(&uc), 1);
    }


    {
        let mut exec = get_exec_promo("(promo_c),promo_u");
        let uc = exec.fork_uc();

        exec.dl("(promo_c)");
        assert_eq!(discount_count_promo_code_set(&uc), 0);

        exec.apply("promo_u").await;
        assert_eq!(discount_count_promo_code_set(&uc), 1);
    }


    {
        let mut exec = get_exec_promo("(promo_u)");
        let mut uc = exec.fork_uc();
        exec.dl("(promo_u)");

        inserted_from_dl("promotion_code", &uc);

        let hm = get_db_as_hm_by_test_id(&mut uc);


        {
            // Assert: active.false still downloaded.
            let x = &("promotion_codes".to_string(), "pro_2".to_string());
            assert!(hm.contains_key(&x));


            assert_eq!(discount_count_promo_code_set(&uc), 1);
        }
    }
}

fn discount_count_promo_code_set(uc: &UniCon) -> i64 {
    match uc {
        UniCon::Rusqlite(x) => {
            let c = &x.c;
            let mut stmt = c.prepare(format!("SELECT count(*) FROM discounts WHERE promotion_code IS NOT NULL").as_str()).unwrap();
            let mut rows = stmt.query([]).unwrap();
            if let Some(row) = rows.next().unwrap() {
                return row.get(0).unwrap();
            }
            unreachable!()
        }
        UniCon::PlaceholderLibA(_) => {}
        UniCon::MySQL(_) => {}
        UniCon::Postgres(_) => {}
    }
    unreachable!()
}
