// use crate::*;
use chrono::prelude::{DateTime, Utc};
use serde_json::json;

// @todo/low Macro expands to code that depends on this. `use` in macro code which does not conflict with usage scope.
use std::collections::HashMap;

use rusqlite::{params, Connection, Result, ToSql};

use unicon::{*};
use crate::uc::{*};
use crate::traits::{*};
use crate::table::{*};
use crate::engines::placeholder::{*};
use crate::engines::sqlite::{*};
use crate::engines::mysql::{*};
use unicon_proc_macro::{*};

// use uc_macro::{
//     Insert,
//     Db,
// };
//
//
// use uc_macro::{
//     SQLiteString,
//     SQLiteFuncRusqlite,
// };
// use uc_macro::{PlaceholderString, PlaceholderFuncStd};


// use mysql::*;
// use mysql::prelude::*;
use std::hash::BuildHasherDefault;
use twox_hash::XxHash;
use std::time::SystemTime;


use serde_json::{Value};

// @see https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro
// - Simple guide to proc macros.


pub trait HasSeqAutoIds {
    fn has_seq_auto_ids(&self) -> bool;
}

impl HasSeqAutoIds for UniCon {
    // Cockroach labs is a distributed Postgres DB, so they do not use sequential auto increment IDs (they generate a random number each time, similar to a UUID).
    // - Postgres auto increment ID's can have gaps (E.g. failed transactions), but for the purpose of these tests the auto increment ID's are assumed to always be sequential.
    fn has_seq_auto_ids(&self) -> bool {
        match self {
            UniCon::Rusqlite(_) => {}
            UniCon::MySQL(_) => {}
            UniCon::Postgres(x) => {
                match &x.create.addr {
                    Addr::IP(x) => {
                        if x.ip.contains("cockroachlabs") {
                            return false;
                        }
                    }
                    Addr::Socket(_) => {}
                }
            }
            UniCon::PlaceholderLibA(_) => {}
        }
        return true;
    }
}

mod util;
mod test_db;

#[cfg(test)]
mod test {
    use super::*;

    use util::{get_temp_file, get_unique_id};
    use rusqlite::{TransactionBehavior, Error};
    use std::time::Duration;
    use test_db::{RowA, DbTest, RowB, RowD};
    use mysql::chrono::NaiveDateTime;
    use crate::dt::DT;


    #[test]
    fn single_insert_row() {
        // let z = RowA {
        //     ..Default::default()
        // };
        // let (_, conn) = get_conn();
        // let mut stmt = conn.prepare(&z.get_sqlite_insert()).unwrap();
        // let params: Vec<_> = SQLiteFuncRusqlite::to_kv_writable_only(&z).into_iter().map(|(k, v)| v).collect();
        // stmt.execute_named(&params).unwrap();
        //
        // println!("{}", <RowA as TableTr>::get_table_name(&z));
    }


    // @see evernote:///view/14186947/s134/ed2b3fc1-4f0d-4d7c-ad69-25855741144b/ed2b3fc1-4f0d-4d7c-ad69-25855741144b/
    // - Two types of trait usage: A and (A + B).
    #[test]
    fn insert_many_rows() {
        let mut all = get_ucs();

        for mut uc in all {
            let a = RowA {
                a_bool: true,
                some_fk: Some(1),
                fk_uuid: "unique-obj-id-1".into(),
                ..Default::default()
            };
            let b = RowA {
                a_bool: true,
                some_fk: Some(2),
                fk_uuid: "unique-obj-id-2".into(),
                ..Default::default()
            };
            let c = RowA {
                a_bool: true,
                some_fk: Some(3),
                fk_uuid: "unique-obj-id-3".into(),
                ..Default::default()
            };


            // Trait usage type A: Fn called from a Sized (type is concrete) instance.
            a.insert(&mut uc);

            let mut many_rows: Vec<&dyn Insert> = vec![
                &b, &c,
            ];

            for r in many_rows {
                // Trait usage type B: Fn called from a not-Sized, dyn (type is not concrete until runtime) instance.
                r.insert(&mut uc);

                // Panics as row was only inserted.
                // r.get_pk();
            }
        }

        // Fresh dbs.
        let mut all = get_ucs();
        for mut uc in all {
            let mut a1 = RowA {
                a_bool: true,
                some_fk: Some(4),
                fk_uuid: "unique-obj-id-4".into(),
                ..Default::default()
            };
            a1.insert_set_pk(&mut uc);

            if uc.has_seq_auto_ids() {
                assert_eq!(a1.get_pk(), 1);
            }

            // Assert WHERE PK works
            a1.title = "Updated Title A.1".into();
            a1.update(&mut uc, "row_a_id");

            // Assert WHERE unique key works
            // @todo/low ensure only a single row is updated to avoid issue when struct value is null which could accidentally update many rows.
            a1.title = "Updated Title A.2".into();
            a1.update(&mut uc, "fk_uuid");


            // @todo/low Note: leave `set_primary_key` fn to user to implement (instead of via macro)?
            // @todo/low Support getting back an auto increment id, and inserting into fk, with the possiblity of rolling back
            // - When writing nested foreign keys with rollback support: Timing, transactions, rolling back, nested foreign keys make this difficult to support generally.
            // let insert_id = conn.last_insert_rowid();
            // match r {
            //     Row::RowA(mut x) => x.row_a_id = Some(insert_id),
            //     Row::RowB(mut x) => x.row_b_id = Some(insert_id)
            // }
        }
    }

    #[test]
    fn query_trait() {
        let to_3ms = "%Y-%m-%d %H:%M:%S.%3f";
        let to_s = "%Y-%m-%d %H:%M:%S";

        // Issue: Using `Utc::now().naive_utc()` results in having microsecond precision that seems to randomly get rounded up to the nearest ms (when using `.format`, and then writing/reading back to a SQLite TEXT column).
        let now_3 = Utc::now().naive_utc().format(&to_3ms).to_string();
        let now_s = Utc::now().naive_utc().format(&to_s).to_string();
        let plain_dt3 = NaiveDateTime::parse_from_str(now_3.as_str(), &to_3ms).unwrap();

        let plain_dt_a = NaiveDateTime::parse_from_str(now_3.as_str(), &to_3ms).unwrap();
        // let plain_dt_b = NaiveDateTime::parse_from_str(now_3.as_str(), &to_s).unwrap(); // Error: parsing must match format exactly.
        let plain_dt_b = NaiveDateTime::parse_from_str(now_s.as_str(), &to_s).unwrap();

        let mut all = get_ucs();
        for mut uc in all {
            let mut rows = vec![
                RowA {
                    a_bool: true,
                    some_fk: Some(1),
                    fk_uuid: "unique-obj-id-1".into(),
                    ..Default::default()
                },
                RowA {
                    a_bool: true,
                    some_fk: Some(2),
                    fk_uuid: "unique-obj-id-2".into(),
                    json_opt: Some(serde_json::from_str("[1,2,3]").unwrap()),
                    plain_dt: Some(plain_dt_b.clone().into()),
                    ..Default::default()
                },
                RowA {
                    a_bool: true,
                    some_fk: Some(3),
                    fk_uuid: "unique-obj-id-3".into(),
                    plain_dt3: Some(plain_dt3.clone().into()),
                    plain_dt: Some(plain_dt_a.clone().into()),
                    json_opt: Some(json! {{"abc": 123}}),
                    ..Default::default()
                },
            ];

            for r in &mut rows {
                r.insert_set_pk(&mut uc);
            }

            let last: RowA = RowA::get_last(&mut uc, "row_a_id").unwrap();

            if uc.has_seq_auto_ids() {
                assert_eq!(last.row_a_id.unwrap(), 3);
            }

            assert_eq!(last.row_a_id.unwrap(), rows.last().unwrap().row_a_id.unwrap());

            // Ensure timestamp is written and read back OK.
            assert_eq!(last.plain_dt3.unwrap().dt, plain_dt3);

            assert!(match last.json_opt.unwrap() {
                serde_json::Value::Object(obj) => {
                    match &obj["abc"] {
                        serde_json::Value::Number(x) => x.as_i64().unwrap() == 123,
                        _ => false
                    }
                }
                _ => false
            });


            let middle: Vec<RowA> = RowA::get_where(&mut uc, ("fk_uuid", &"unique-obj-id-2"));
            assert_eq!(middle.len(), 1);
            assert_eq!(middle[0].some_fk, Some(2));


            let middle: Vec<RowA> = RowA::get_where(&mut uc, (("fk_uuid", &"unique-obj-id-2"), ("a_bool", &true)));
            assert_eq!(middle.len(), 1);
            assert_eq!(middle[0].some_fk, Some(2));


            let middle: Vec<RowA> = RowA::get_where(&mut uc, (("fk_uuid", &"unique-obj-id-2"), ("a_bool", &false)));
            assert_eq!(middle.len(), 0);


            let plain_dt: DT = plain_dt_b.clone().into();
            let middle: Vec<RowA> = RowA::get_where(&mut uc, (("fk_uuid", &"unique-obj-id-2"), ("a_bool", &true), ("plain_dt", &plain_dt)));
            assert_eq!(middle.len(), 1);
            assert_eq!(middle[0].some_fk, Some(2));


            let null: Option<DT> = None;
            let middle: Vec<RowA> = RowA::get_where(&mut uc, (("fk_uuid", &"unique-obj-id-2"), ("a_bool", &true), ("plain_dt", &null)));
            assert_eq!(middle.len(), 0);


            // @todo/low Add WHERE JSON support for MySQL.
            // Note: Works with SQLite and Postgres, but not MySQL, see `where` fn comment.
            // let json_val: Value = serde_json::from_str("[1,2,3]").unwrap();
            // let middle: Vec<RowA> = RowA::get_where(&mut uc, (("fk_uuid", &"unique-obj-id-2"), ("a_bool", &true), ("json_opt", &json_val)));
            // assert_eq!(middle.len(), 1);
            // assert_eq!(middle[0].some_fk, Some(2))


            // DbTest::drop_all(&mut uc);
        }
    }

    #[test]
    fn test_tx_write_lock() {
        // Create database.
        let f = get_temp_file(get_unique_id() + ".sqlite").unwrap();

        let mut uc = UniCon::new(&UniConCreate {
            engine: Engine::SQLite(SQLiteCreate {
                file: f.clone()
            })
        });

        uc.ensure_schema_and_tables_exist_and_writable::<DbTest>();

        // Open tx.
        let tx = uc.tx_open();


        // Try to open tx on another connection, ensure it fails/is isolated.
        let mut conn = Connection::open(&f).unwrap();
        conn.busy_timeout(Duration::from_secs(0)); // Fail immediately, do not retry after sleep.
        let r = conn.transaction_with_behavior(TransactionBehavior::Immediate);


        // @todo/low MySQL and other engines.
        assert!(r.is_err());
        if let Err(Error::SqliteFailure(x, _)) = r {
            assert_eq!(x.extended_code, 5);
        }
    }


    fn get_pks(x: &Vec<RowA>) -> Vec<i64> {
        x.iter().map(|x2| x2.row_a_id.unwrap()).collect()
    }

    #[test]
    fn test_tx_writes() {
        let mut all = get_ucs();
        for mut uc in all {
            let mut rows = vec![
                RowA {
                    a_bool: true,
                    some_fk: Some(1),
                    fk_uuid: "unique-obj-id-1".into(),
                    ..Default::default()
                },
                RowA {
                    a_bool: true,
                    some_fk: Some(2),
                    fk_uuid: "unique-obj-id-2".into(),
                    ..Default::default()
                },
                RowA {
                    a_bool: true,
                    some_fk: Some(3),
                    fk_uuid: "unique-obj-id-3".into(),
                    ..Default::default()
                },
            ];

            // TX fails.
            {
                let mut utx = uc.tx_open().unwrap();
                assert!(utx.get_tables().len() > 0);

                for r in &mut rows {
                    // Issue: PK writes into rows before commit. Allow rollback.
                    r.tx_insert_set_pk(&mut utx);
                }

                // No close. Implicit rollback here on drop.
            }

            // TX fails.
            {
                let mut utx = uc.tx_open().unwrap();
                for r in &mut rows {
                    // Issue: PK writes into rows before commit. Allow rollback.
                    r.tx_insert_set_pk(&mut utx);
                }

                utx.tx_rollback();
            }


            // Issue: If the TX fails, the row IDs are still written to the Rust struct instances.
            assert!(rows.first().unwrap().row_a_id.is_some());

            /// Possible Fixes:
            /// - A. COW, implement rollback logic.
            /// - B. Store writes in a closure, run it after commit. `apply_ids()`
            ///     - C. On write, store a revert function against the type/primary key, drop the hashmap of revert functions on tx complete.
            /// - D. Use a closure that wraps TX code to allow it to be retried.
            ///
            /// Why this does not need a fix.
            /// - Network requests are cheaper than RAM.
            ///     - When downloading very large datasets it will not be possible to store them all in RAM.
            ///     - It's easier and cheaper (RAM) to just re-download the data from the API.
            ///     - Data is streamed from the network API responses into the DB as soon as possible to reduce RAM.
            /// - `rows` will be a child of `tx_open`.
            ///     - If the tx fails, rows can be dropped and re-downloaded.
            ///     - If the tx fails, it will be because of:
            ///         - A. Coding error.
            ///         - B. Connection/tx attainment issues.
            ///         - C. Database modified to an invalid state by external process.
            ///     - Assuming A never happens, B and C will both need operator assistance; *there is no way to make a failed tx succeed (re-trying it is likely to hit the same block)*.
            ///         - Re-try semantics can match "just re-run the CLI", instead of going more granular and retrying on a per-row level (which requires managing the rollback state of structs).
            /// - Rollback/retry state can be handled in app code on a per-closure/scope basis.
            ///     - Closures and ownership, retry logic can be handled outside of this library.
            /// - The success path is the most likely.
            ///     - It should be very unlikely that the tx will fail.
            ///
            /// Why this needs a fix.
            /// - It limits this libraries use a generally re-usable library.


            // Assert: TX OK.
            // - This is ok as the previous tx failed, which means there is no unique constraint conflict.
            {
                let mut utx = uc.tx_open().unwrap();
                for r in &mut rows {
                    r.tx_insert_set_pk(&mut utx);
                }
                utx.tx_close().unwrap();

                // Read back written rows after tx commit.
                let last: RowA = RowA::get_last(&mut uc, "row_a_id").unwrap();

                // Cannot use PK id as Postgres has auto incrementing ID gaps for when tx fails.
                assert_eq!(last.some_fk, Some(3));


                let all = RowA::get_all(&mut uc);
                // Note: this does not work as the insert/update ts is never returned on insert (like the auto incremented primary key is).
                // assert_eq!(rows, all);
                assert_eq!(get_pks(&rows), get_pks(&all));
            }


            // Assert: Updates inside tx work.
            {
                let mut utx = uc.tx_open().unwrap();
                for (i, r) in (&mut rows).into_iter().enumerate() {
                    r.desc_x = format!("Desc updated for row index {}.", i).into();
                    r.tx_update_pk(&mut utx);
                }
                utx.tx_close().unwrap();

                // Assert: SQL server state is the same as local Rust struct state.
                let all: Vec<RowA> = RowA::get_all(&mut uc);
                assert_eq!(rows.len(), all.len());
                for (local, remote) in rows.iter().zip(all.iter()) {
                    assert_eq!(local.row_a_id, remote.row_a_id);
                    assert_eq!(local.desc_x, remote.desc_x);
                }
            }


            // This correctly fails (unique constraint).
            // {
            //     let utx = uc.tx_open().unwrap();
            //     for r in &mut rows {
            //         r.tx_insert_set_pk(&mut utx);
            //     }
            //     utx.tx_close().unwrap();
            // }

            // Assert: Deletes inside tx work.
            {
                let mut utx = uc.tx_open().unwrap();
                for r in &mut rows {
                    r.tx_delete(&mut utx, "fk_uuid");
                }
                utx.tx_close().unwrap();

                // Assert: Rows are gone.
                let all: Vec<RowA> = RowA::get_all(&mut uc);
                assert_eq!(all.len(), 0);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_drop_all_on_empty_db() {
        let mut all = get_ucs();
        for mut uc in all {

            // // #[should_panic] does not allow loops (only top level fn).
            // let result = std::panic::catch_unwind(|| {
            //     // Issue: the trait `UnwindSafe` is not implemented for `&mut uc_lib::UniCon`
            // });
            //
            // assert!(result.is_err());


            // Ok: Drop tables.
            DbTest::drop_all(&mut uc);

            // Panic: Try to drop non existent tables.
            DbTest::drop_all(&mut uc);

            // @todo/low Ensure panic for other db engines beyond the 1st iteration of the loop.
        }
    }


    #[test]
    #[should_panic]
    fn test_update_where_invalid_col() {
        let mut all = get_ucs();
        for mut uc in all {
            let a = RowA {
                a_bool: true,
                ..Default::default()
            };
            a.update(&mut uc, "col_that_does_not_exist");
        }
    }

    #[test]
    #[should_panic]
    fn test_only_json_objects_allowed() {
        let mut all = get_ucs();
        for mut uc in all {
            let a = RowD {
                json_direct: serde_json::from_str("null").unwrap(),
                ..Default::default()
            };

            a.insert(&mut uc);
        }
    }


    fn get_conn() -> (String, Connection) {
        let schema: Schema = DbTest::get_target_schema_sqlite();
        let f = get_temp_file(get_unique_id() + ".sqlite").unwrap();
        let conn = Connection::open(&f).unwrap();
        conn.execute_batch(&schema.get_create_tables_and_indexes().join(";\n")).unwrap();
        println!("Created test DB: {}", &f);
        (f, conn)
    }

    fn create_schema(uc: &mut UniCon, engine: &str, key: &str, id: &str) {
        uc.ensure_schema_and_tables_exist_and_writable::<DbTest>();
        println!("Created test {} DB, key:{}, id:{}", engine, key, id);
    }


    /// To test one engine: comment out others as needed.
    ///
    /// Note: Nesting:
    ///     - "database -> schema -> table -> row" = (Postgres, MSSQL, Oracle).
    ///     - "table -> row" = SQLite
    ///     - "database -> table -> row" = MySQL (treats SCHEMA the same as DATABASE)
    ///
    /// - @see https://stackoverflow.com/questions/11618277/difference-between-schema-database-in-mysql
    ///
    /// Cross database/schema queries:
    /// - MySQL allows cross database queries with `db_a.tbl_a, db_b.tbl_b`.
    ///     - Postgres allows this for schemas, NOT databases.
    /// - SQLite allows cross file queries, but transactions over files are not possible.
    fn get_ucs() -> Vec<UniCon> {
        let mut x = vec![];
        let schema_name = format!("rust_test_{}", get_time_str_seconds());


        // SQLite
        {
            let f = get_temp_file(get_unique_id() + ".sqlite").unwrap();
            let mut uc = UniCon::new(&UniConCreate {
                engine: Engine::SQLite(SQLiteCreate {
                    file: f.clone()
                })
            });
            create_schema(&mut uc, "sqlite", "local", &f);
            x.push(uc);
        }

        return x;

        // MySQL
        {
            // No TLS.
            let schema_local = format!("{}_local", &schema_name);
            let mut uc = UniCon::new(&UniConCreate {
                engine: Engine::MySQL(MySQLCreate {
                    addr: Addr::IP(IPPort {
                        ip: "127.0.0.1".to_string(),
                        port: 3306,
                    }),
                    user: Some("root".into()),
                    pass: Some("my-secret-pw".into()),
                    db_name: None,
                    schema_name: schema_local.clone().into(),
                    tls: None,
                })
            });
            create_schema(&mut uc, "mysql", "local", &schema_local);
            x.push(uc);

            // With TLS, local MySQL docker instance.
            let mut uc = UniCon::new(&UniConCreate {
                engine: Engine::MySQL(MySQLCreate {
                    addr: Addr::IP(IPPort {
                        ip: "08e59019d71f".to_string(),
                        port: 3306,
                    }),
                    user: Some("root".into()),
                    pass: Some("my-secret-pw".into()),
                    db_name: None,
                    schema_name: schema_name.clone().into(),
                    tls: Some(Tls {
                        ca_cert_file: Some(format!("{}/../sh/tls/dev-keys/ssl-cert-snakeoil.pem", env!("CARGO_MANIFEST_DIR"))),
                        verify_server_cert_hostname: None,
                    }),
                })
            });
            create_schema(&mut uc, "mysql", "local-tls", &schema_name);
            x.push(uc);

            // With TLS, remote GCP Cloud SQL instance.
            // let mut uc = UniCon::new(&UniConCreate {
            //     engine: Engine::MySQL(MySQLCreate {
            //         addr: Addr::IP(IPPort {
            //             ip: "35.246.65.1".to_string(),
            //             port: 3306,
            //         }),
            //         user: Some("root".into()),
            //         pass: Some("yLJGbJ089fJtcheN".into()),
            //         db_name: None,
            //         schema_name: schema_name.clone().into(),
            //         tls: Some(Tls {
            //             ca_cert_file: Some(format!("{}/../sh/tls/remote-keys/gcp-ca-mysql-ins-a.pem", env!("CARGO_MANIFEST_DIR"))),
            //             verify_server_cert_hostname: Some(false),
            //         }),
            //     })
            // });
            // create_schema(&mut uc, "mysql", "remote-tls-gcp", &schema_name);
            // x.push(uc);
        }

        // Postgres
        {
            // No TLS.
            let schema_local = format!("{}_local", &schema_name);
            let mut uc = UniCon::new(&UniConCreate {
                engine: Engine::Postgres(MySQLCreate {
                    addr: Addr::IP(IPPort {
                        ip: "127.0.0.1".to_string(),
                        port: 5432,
                    }),
                    user: Some("postgres".into()),
                    pass: Some("postgres".into()),
                    // db_name: "db_abc".to_string().into(),
                    db_name: None,
                    schema_name: schema_local.clone().into(),
                    // schema_name: "abc".to_string().into()
                    tls: None,
                })
            });
            create_schema(&mut uc, "postgres", "local", &schema_local);
            x.push(uc);

            // With TLS (self signed, dev/local only, connects to local Docker Postgres instance).
            let mut uc = UniCon::new(&UniConCreate {
                engine: Engine::Postgres(MySQLCreate {
                    addr: Addr::IP(IPPort {
                        // Prevents `IP address mismatch` from openssl (https://www.openssl.org/docs/man1.1.1/man3/X509_check_ip.html)
                        // - Add `127.0.0.1   08e59019d71f` to /etc/hosts.
                        // - This value comes from the cert field `Extension: Subject Alternative Name, DNS Name: X`.
                        // - See `tls/notes.md`.
                        ip: "08e59019d71f".to_string(),
                        port: 5432,
                    }),
                    user: Some("postgres".into()),
                    pass: Some("postgres".into()),
                    // db_name: "db_abc".to_string().into(),
                    db_name: None,
                    schema_name: schema_name.clone().into(),
                    // schema_name: "abc".to_string().into()
                    tls: Some(Tls {
                        ca_cert_file: Some(format!("{}/../sh/tls/dev-keys/ssl-cert-snakeoil.pem", env!("CARGO_MANIFEST_DIR"))),
                        verify_server_cert_hostname: None,
                    }),
                })
            });
            create_schema(&mut uc, "postgres", "local-tls", &schema_name);
            x.push(uc);


            // With TLS (remote, Supabase. Note: Invalid hostname when using `native-tls`/macOS, but `openssl` passes hostname checks. macOS must be stricter with these checks).
            let mut uc = UniCon::new(&UniConCreate {
                engine: Engine::Postgres(MySQLCreate {
                    addr: Addr::IP(IPPort {
                        ip: "db.otfzdikncxtokgnkhsut.supabase.co".to_string(),
                        port: 5432,
                    }),
                    user: Some("postgres".into()),
                    pass: Some("test-db-a-pass".into()),
                    // db_name: "db_abc".to_string().into(),
                    db_name: Some("postgres".into()),
                    schema_name: schema_name.clone().into(),
                    // schema_name: "abc".to_string().into()
                    tls: Some(Tls {
                        ca_cert_file: Some(format!("{}/../sh/tls/remote-keys/supabase-prod-ca-2021.crt", env!("CARGO_MANIFEST_DIR"))),
                        verify_server_cert_hostname: None,
                    }),
                })
            });
            create_schema(&mut uc, "postgres", "remote-tls-supabase", &schema_name);
            x.push(uc);


            // With TLS (remote, gcp cloud sql instance).
            // let mut uc = UniCon::new(&UniConCreate {
            //     engine: Engine::Postgres(MySQLCreate {
            //         addr: Addr::IP(IPPort {
            //             ip: "34.142.13.147".to_string(),
            //             port: 5432,
            //         }),
            //         user: Some("postgres".into()),
            //         pass: Some("IBcvj57tKLv15mI7".into()),
            //         // db_name: "db_abc".to_string().into(),
            //         db_name: Some("postgres".into()),
            //         schema_name: schema_name.clone().into(),
            //         // schema_name: "abc".to_string().into()
            //         tls: Some(Tls {
            //             ca_cert_file: Some(format!("{}/../sh/tls/remote-keys/gcp-ca-postgres-ins-a.pem", env!("CARGO_MANIFEST_DIR"))),
            //             verify_server_cert_hostname: Some(false),
            //         }),
            //     })
            // });
            // create_schema(&mut uc, "postgres", "remote-tls-gcp", &schema_name);
            // x.push(uc);

            // With TLS (remote, Cockroach Labs).
            let mut uc = UniCon::new(&UniConCreate {
                engine: Engine::Postgres(MySQLCreate {
                    addr: Addr::IP(IPPort {
                        // Prevents `IP address mismatch` from openssl (https://www.openssl.org/docs/man1.1.1/man3/X509_check_ip.html)
                        // - Add `127.0.0.1   08e59019d71f` to /etc/hosts.
                        // - This value comes from the cert field `Extension: Subject Alternative Name, DNS Name: X`.
                        // - See `tls/notes.md`.
                        ip: "free-tier5.gcp-europe-west1.cockroachlabs.cloud".to_string(),
                        port: 26257,
                    }),
                    user: Some("enzo".into()),
                    pass: Some("iAij3d3@X@KqUh!".into()),
                    // db_name: "db_abc".to_string().into(),
                    db_name: Some("clever-donkey-1857.defaultdb".into()),
                    schema_name: schema_name.clone().into(),
                    // schema_name: "abc".to_string().into()
                    tls: Some(Tls {
                        // ISRG root CA from Lets Encrypt is used and already in the OS level CA store.
                        ca_cert_file: None,
                        verify_server_cert_hostname: None,
                    }),
                })
            });
            create_schema(&mut uc, "postgres", "remote-tls-cockroachlabs", &schema_name);
            x.push(uc);
        }


        x
    }
}

// Ensure fresh state for tests by creating a unique schema name.
// E.g. 00_34_60
fn get_time_str_seconds() -> String {
    let now = SystemTime::now();
    let dt: DateTime<Utc> = now.clone().into();
    format!("{}", dt.format("%H_%M_%S_%f"))
}