//use unicon_proc_macro::{Insert, Table, SQLiteString, SQLiteStringSchema, SQLiteFuncRusqlite};
//use unicon_proc_macro::{PlaceholderString, PlaceholderFuncStd};
use std::collections::HashMap;
//use unicon::dt3::DT3;
use std::hash::BuildHasherDefault;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use mysql::Params;
use mysql::prelude::Queryable;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use stripe_client::types::types::GetId;
use twox_hash::XxHash;
use unicon::{*};
use unicon::dt::{*};
//use unicon::{*};
use unicon::dt::{*};
use unicon::dt3::{*};
use unicon::dt3::{*};
use unicon::engines::mysql::{*};
use unicon::engines::postgres::{*};
use unicon::engines::placeholder::{*};
use unicon::engines::sqlite::{*};
use unicon::table::{*};
use unicon::traits::{*};
use unicon::uc::{*};
use unicon::utx::{*};
use unicon_proc_macro::{*};

use crate::fns::{get_utc_dt, get_utc_dt_from_3ms};
use crate::providers::stripe::schema::types::GetObjType;
use stripe_client::http::http::{StripeClient, StripeAccount};


// When there are 0 events, an observing process should be able to distinguish:
// - DB up to date, no events to apply, TD process is actively polling.
// - DB not up to date, possibly missing data, no events have been applied, TD process has failed.
//
// Avoid inserting a new `td_run` row for each poll of events which would cause a lot of data storage to be used. Update a single value instead.
#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
#[table_name_plural(false)]
pub struct TdMetadata {
    #[primary_key]
    pub id: Option<i64>,
    pub cli_version: String,
    pub stripe_version: String,

    // - Avoid merging two different accounts data into a single DB (E.g. if the user changes the config Stripe secret key to a different account but keeps the same target database).
    // - Allow using the account settings (E.g. may be used to write custom invoice code using default current, statement descriptor, colours etc).
    pub stripe_account_id: String,
    pub stripe_account: Value,
    pub stripe_is_test: bool,

    // Timestamp of last_run, regardless if any events were processed.
    // - Allows distinguishing if no new events means (A. td process is not running OR B. no new events).
    pub heartbeat_ts: Option<DT3>,
}


// "1.2.3" => (1, 2, 3)
pub fn get_semver_ints(x: &str) -> (u32, u32, u32) {
    let d: Vec<String> = x.split(".").map(|x| x.to_string()).collect();
    assert_eq!(d.len(), 3);
    (
        d[0].parse().unwrap(),
        d[1].parse().unwrap(),
        d[2].parse().unwrap()
    )
}


impl TdMetadata {
    // Overwrites a single ts cell, instead of adding a new row.
    // - Reduces disk space needed.
    // - Allows processes to check if they are operating on an up to date DB without having to RPC other systems (like a log system to determine a `checking for events` entry).
    pub fn set_heartbeat_now(uc: &mut UniCon) {
        match uc {
            UniCon::Rusqlite(x) => {
                let mut stmt = x.c.prepare_cached("UPDATE td_metadata SET heartbeat_ts=STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW')").unwrap();
                assert_eq!(stmt.execute([]).unwrap(), 1, "Failed to write heartbeat to SQL.");
            }
            UniCon::MySQL(x) => {
                x.c.exec_drop("UPDATE td_metadata SET heartbeat_ts=UTC_TIMESTAMP(6)", Params::Empty).unwrap();
                assert_eq!(x.c.affected_rows(), 1, "Failed to write heartbeat to SQL.");
            }
            UniCon::Postgres(x) => {
                let changes = x.c.execute("UPDATE td_metadata SET heartbeat_ts=timezone('utc', now())", &[]).unwrap();
                assert_eq!(changes, 1, "Failed to write heartbeat to SQL.");
            }
            UniCon::PlaceholderLibA(_) => {}
        }
    }

    pub fn check_cli_and_stripe_versions_match(uc: &mut UniCon, sa: &StripeAccount) -> TdMetadata {
        let x: TdMetadata = Self::get_last(uc, "id").expect("Metadata should be created with the DB schema.");
        assert_eq!(x.id.unwrap(), 1);
        let (cli, stripe) = get_versions();

        let to_use_newer_msg = || {
            error!("To use the newer CLI version, backup and drop the existing database so it can be re-created (incrementally migrating the SQL DB schema is not currently supported).");
        };

        if stripe != x.stripe_version {
            error!("The Stripe version this CLI uses ({}) does not match the one that originally wrote the db ({}).", stripe, x.stripe_version);
            to_use_newer_msg();
            panic!();
        }

        let a = get_semver_ints(cli);
        let db = get_semver_ints(&x.cli_version);

        // Only the last semver version is allowed to differ (indicates no Stripe or SQL schema changes).
        // semver, loose meanings:
        // A.B.C
        // - A. A new Stripe version/client (new Open API spec generates a new client which is not backwards compatible).
        // - B. Same Stripe version, but possible changes in SQL schema/event processing etc.
        // - C. Minor changes - user can run against previous version of DB.
        let ok = (a.0 == db.0 && a.1 == db.1);
        if !ok {
            error!("This CLI version ({}) does not match the one that originally wrote the db ({}).", cli, x.cli_version);
            to_use_newer_msg();
            panic!();
        }


        // Stripe account must match the one previously used.
        // - One DB = one Stripe account = one dataset.
        // - Note: One account ID can have both test and prod data sets.
        // - One account can have many different secret keys.
        if x.stripe_account_id != sa.id {
            error!("Stripe account ID does not match the one previously used. DB has {}, trying to use {}.", x.stripe_account_id, sa.id);
            panic!();
        }

        if x.stripe_is_test != sa.is_test {
            error!("Stripe is_test does not match; cannot mix live and test data. DB has {}, trying to use {}.", x.stripe_is_test, sa.is_test);
            panic!();
        }

        x
    }

    pub fn insert_cli_and_stripe_versions(uc: &mut UniCon, sa: &StripeAccount) {
        let mut utx = uc.tx_open().unwrap();
        let (cli, stripe) = get_versions();

        let mut row_1 = TdMetadata {
            id: None,
            cli_version: cli.to_string(),
            stripe_version: stripe.to_string(),
            stripe_account_id: sa.id.clone(),
            stripe_account: sa.account.clone().into(),
            stripe_is_test: sa.is_test,
            heartbeat_ts: None,
        };

        assert_eq!(row_1.tx_insert_set_pk(&mut utx), 1, "Expecting a single meta data row, found more than one.");
        utx.tx_close();
    }

    // Update the Stripe account JSON on start up (in case end users want to read those settings).
    pub fn update_stripe_account(&mut self, uc: &mut UniCon, sa: &StripeAccount) {
        assert_eq!(sa.id, self.stripe_account_id);
        assert_eq!(sa.is_test, self.stripe_is_test);

        self.stripe_account = sa.account.clone().into();
        self.update(uc, self.get_key_pk());
    }
}

fn get_versions() -> (&'static str, &'static str) {
    (
        get_cli_version(),
        StripeClient::get_api_version()
    )
}

// Gets the lib-app crate Cargo.toml version.
// Cannot use `cli` crate as Cargo does not allow cycling dependencies (cli <-> lib_app)
// const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const VERSION: &'static str = "0.5.0";

pub fn get_cli_version() -> &'static str {
    VERSION
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct TdRun {
    #[primary_key]
    pub run_id: Option<i64>,

    // Only Stripe supported; DB is likely to be for dedicated to a single Stripe account (one DB per different TD/Stripe account).
    // pub from_api: String,
    pub r#type: String,
    pub start_ts: Option<DT3>,
    pub end_ts: Option<DT3>,
}

impl TdRun {
    /// Returns `None` when there are no rows (freshly created db; first run).
    /// This is either 28 days since the last run (either the first `download` or last `apply_events`).
    pub fn is_apply_events_possible(mut uc: &mut UniCon) -> Option<bool> {
        let row: TdRun = Self::get_last(&mut uc, "end_ts")?;

        let now = Utc::now().naive_utc();
        let end: NaiveDateTime = row.end_ts.unwrap().into();
        let days = end.signed_duration_since(now).num_days();

        // Stripes API only returns events that are less than 30 days old.
        (days < 28).into()
    }

    /// The first run is always a full download (as Stripe events are only stored for 30 days), subsequent runs are incremental `apply_events`.
    pub fn get_last_run_tx(utx: &mut UniTx) -> Option<Self> {
        let row: TdRun = Self::tx_get_last(utx, "end_ts")?;
        row.into()
    }

    pub fn get_last_run(uc: &mut UniCon) -> Option<Self> {
        let row: TdRun = Self::get_last(uc, "end_ts")?;
        row.into()
    }

    pub fn is_download(&self) -> bool {
        self.run_id.unwrap() == 1 && self.r#type == "download"
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
/// This is needed on the first "apply_events" to determine which event writes were included in the first object download (and can be skipped).
#[index("CREATE INDEX obj_id ON self (obj_id)")]
#[index("CREATE INDEX run_id ON self (run_id)")]
pub struct TdStripeWrite {
    // @todo/maybe Use i128 as this is the sum of all rows written.
    #[primary_key]
    pub write_id: Option<i64>,

    pub run_id: i64,

    // Stripe `object`, matches the API exposed object types exactly (none are TD made up).
    pub obj_type: String,
    pub obj_id: String,
    pub table_name: String,
    pub write_type: String,

    #[insert_ts]
    pub insert_ts: Option<DT3>,
}


impl TdStripeWrite {
    pub fn get_all(uc: &UniCon) -> Vec<Self> {
        // language=sql
        let std_sql = r###"
                select * from td_stripe_writes
        "###;

        match uc {
            UniCon::Rusqlite(x) => {
                let c = &x.c;

                let mut stmt = c.prepare_cached(std_sql).unwrap();
                let mut rows = stmt.query_map([], |r| {
                    Ok(Self {
                        write_id: r.get(0).unwrap(),
                        run_id: r.get(1).unwrap(),
                        obj_type: r.get(2).unwrap(),
                        obj_id: r.get(3).unwrap(),
                        table_name: r.get(4).unwrap(),
                        write_type: r.get(5).unwrap(),
                        insert_ts: r.get(6).unwrap(),
                    })
                }).unwrap();

                let mut set = vec![];
                for r in rows {
                    set.push(r.unwrap());
                }
                set
            }
            _ => unimplemented!("Called from tests only.")
        }
    }

    pub fn get_insert_count_by_obj_type(utx: &mut UniTx) -> HashMap<String, i64> {
        let mut hm = HashMap::new();

        // language=sql
        let std_sql = r###"
                select obj_type, count(*) from td_stripe_writes where write_type='c' and run_id=1 group by obj_type order by obj_type asc
        "###;

        match utx {
            UniTx::Rusqlite(tx) => {
                let mut stmt = tx.prepare_cached(&std_sql).unwrap();
                let mut rows = stmt.query([]).unwrap();
                while let Some(row) = rows.next().unwrap() {
                    hm.insert(row.get(0).unwrap(), row.get(1).unwrap());
                }
            }
            UniTx::MySQL(tx) => {
                tx.query_map(&std_sql, |(obj_type, count)| {
                    hm.insert(obj_type, count);
                }).unwrap();
            }
            UniTx::Postgres(tx) => {
                for x in tx.query(std_sql, &[]).unwrap() {
                    hm.insert(x.get(0), x.get(1));
                }
            }
            UniTx::PlaceholderLibA(_) => {}
        }

        hm
    }

    // Used for billing quota.
    pub fn get_write_count_excluding_deletes(utx: &mut UniTx, run_id: i64) -> i64 {
        let mut count = 0;

        // language=sql
        let std_sql = format!("select count(*) from td_stripe_writes where (write_type='c' or write_type='u') and run_id={}", run_id);

        match utx {
            UniTx::Rusqlite(tx) => {
                let mut stmt = tx.prepare_cached(&std_sql).unwrap();
                let mut rows = stmt.query([]).unwrap();
                if let Some(row) = rows.next().unwrap() {
                    count = row.get(0).unwrap();
                }
            }
            UniTx::MySQL(tx) => {
                let x: Vec<i64> = tx.exec(&std_sql, Params::Empty).unwrap();
                count = x[0]
            }
            UniTx::Postgres(tx) => {
                let res = tx.query(std_sql.as_str(), &[]).unwrap();
                let first = res.first().unwrap();
                count = first.get(0);
            }
            UniTx::PlaceholderLibA(_) => {}
        }

        count
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
#[derive(Insert)]
pub struct TdStripeApplyEvent {
    #[primary_key]
    pub apply_id: Option<i64>,

    pub run_id: i64,

    #[unique]
    pub event_id: String,

    pub action: String,

    pub write_ids: Option<Value>,

    #[insert_ts]
    pub insert_ts: Option<DT3>,
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ResActionsTaken {
    pub apply_id: i64,
    pub data_object_id: String,
    pub data_object_object: String,
    pub r#type: String,
    pub action: String,
}


impl TdStripeApplyEvent {
    pub fn test_get_actions_taken(uc: &UniCon) -> Vec<ResActionsTaken> {
        // language=sql
        let std_sql = r###"
                select apply_id, data_object_id, data_object_object, ne.type, a.action
                from td_stripe_apply_events a left join notification_events ne on(a.event_id=ne.id)
                order by apply_id asc
        "###;

        match uc {
            UniCon::Rusqlite(x) => {
                let c = &x.c;

                let mut stmt = c.prepare_cached(std_sql).unwrap();
                let mut rows = stmt.query_map([], |r| {
                    Ok(ResActionsTaken {
                        apply_id: r.get(0).unwrap(),
                        data_object_id: r.get(1).unwrap(),
                        data_object_object: r.get(2).unwrap(),
                        r#type: r.get(3).unwrap(),
                        action: r.get(4).unwrap(),
                    })
                }).unwrap();

                let mut set = vec![];
                for r in rows {
                    set.push(r.unwrap());
                }
                set
            }
            _ => unimplemented!()
        }
    }
}

// pub trait GetIdRw {
//     fn get_id(&self) -> String;
// }

use std::time::{Instant, Duration};
use std::thread;

use std::sync::Mutex;
lazy_static! {
    // Issue, summary:
    // - When each DB write takes longer than 1ms (E.g. remote SQL database):
    //      - The download rate can be much faster than the DB insert rate because each HTTP API list gets converted to 100's of inserts.
    //      - The DB insert API is sync because it may be a SQLite connection which is not Send, and it is applied against a single open transaction which means serial writes for most SQL engines.
    //      - Blocking database writes eventually block the event loop which causes HTTP requests to timeout.
    //
    // Fix, temporary:
    // - Warn the user of high DB latency, only support local databases for now.
    //
    // Fix, eventual:
    // - Download at the maximum API speed, queue writes in RAM (5us per insert), batch apply them to target database (reduce round trips by 1000x).
    //
    // See `blocking-db-api-issues.md`
    static ref HIGH_LATENCY_WARN: Mutex<bool> = Mutex::new(false);
}

pub trait LogWrite: Insert + TableStatic + GetObjType + GetId {
    fn tx_insert_set_pk_log_write(&mut self, utx: &mut UniTx, run_id: i64) -> i64 {
        let now = Instant::now();
        // thread::sleep(Duration::from_millis(30));

        self.tx_insert_set_pk(utx);

        let mut write = TdStripeWrite {
            write_id: None,
            run_id,
            obj_type: self.get_obj_type().to_string(),
            obj_id: self.get_id(),
            table_name: Self::get_table_name_static().to_string(),
            write_type: "c".to_string(),
            insert_ts: None,
        };

        let x = write.tx_insert_set_pk(utx);

        let elapsed = now.elapsed();
        let micros_per_insert = (elapsed.as_micros() / 2);
        if micros_per_insert > 3000 {
            let mut x = HIGH_LATENCY_WARN.lock().unwrap();
            if !*x {
                warn!("Database inserts are too slow ({}μs per insert). Fix: Run the CLI on the same LAN or machine as your database server. HTTP requests may timeout due to not being inserted fast enough (and to avoid queueing writes in RAM). You can also set a lower `max_requests_per_second` config value.", micros_per_insert);
                *x = true;
            }
        }

        debug!("tx_insert_set_pk_log_write, elapsed: {}μs, {}", elapsed.as_micros(), self.get_id());
        x
    }

    fn tx_update_log_write(&mut self, utx: &mut UniTx, run_id: i64, w: &'static str) -> i64 {
        let now = Instant::now();
        // thread::sleep(Duration::from_millis(30));

        assert_eq!(self.tx_update(utx, &w), 1);

        let mut write = TdStripeWrite {
            write_id: None,
            run_id,
            obj_type: self.get_obj_type().to_string(),
            obj_id: self.get_id(),
            table_name: Self::get_table_name_static().to_string(),
            write_type: "u".to_string(),
            insert_ts: None,
        };

        let x = write.tx_insert_set_pk(utx);

        debug!("tx_update_log_write, elapsed: {}μs, {}", now.elapsed().as_micros(), self.get_id());
        x
    }

    fn tx_delete_log_write(&mut self, utx: &mut UniTx, run_id: i64, w: &'static str) -> i64 {
        let now = Instant::now();
        let changes = self.tx_delete(utx, &w);

        // "delete if exists".
        assert!((changes == 0 || changes == 1));

        let mut write = TdStripeWrite {
            write_id: None,
            run_id,
            obj_type: self.get_obj_type().to_string(),
            obj_id: self.get_id(),
            table_name: Self::get_table_name_static().to_string(),
            write_type: "d".to_string(),
            insert_ts: None,
        };

        let x = write.tx_insert_set_pk(utx);

        debug!("tx_delete_log_write, elapsed: {}μs, {}", now.elapsed().as_micros(), self.get_id());
        x
    }
}


/// @todo/low Move into general `uc_lib`.
pub trait DeleteStatic: TableStatic {
    fn tx_delete_static(utx: &mut UniTx, id: &str) -> i64 {
        let sql = format!("DELETE FROM {} WHERE id = :id", Self::get_table_name_static());
        let sql_pg = format!("DELETE FROM {} WHERE id = $1", Self::get_table_name_static());

        match utx {
            UniTx::Rusqlite(tx) => {
                let mut stmt = tx.prepare_cached(&sql).unwrap();
                let changes = stmt.execute_named(&[(":id", &id)]).unwrap();
                return changes as i64;
            }
            UniTx::MySQL(tx) => {
                // @todo/next test this
                tx.exec_drop(sql, Params::Positional(vec![id.clone().into()])).unwrap();
                return tx.affected_rows() as i64;
            }
            UniTx::Postgres(tx) => {
                let changes = tx.execute(sql_pg.as_str(), &[&id]).unwrap();
                return changes as i64;
            }
            UniTx::PlaceholderLibA(x) => {}
        }
        unreachable!();
    }
}

pub trait DeleteStaticLogWrite: GetObjType + DeleteStatic {
    fn tx_delete_static_log_write(utx: &mut UniTx, run_id: i64, id: &str) -> i64 {
        let changes = Self::tx_delete_static(utx, &id);

        // Target row must exist.
        assert_eq!(changes, 1);

        let mut write = TdStripeWrite {
            write_id: None,
            run_id,
            obj_type: Self::get_obj_type_static().into(),
            obj_id: id.into(),
            table_name: Self::get_table_name_static().to_string(),
            write_type: "d".to_string(),
            insert_ts: None,
        };

        write.tx_insert_set_pk(utx)
    }
}

impl<T> DeleteStatic for T where T: TableStatic {}

impl<T> DeleteStaticLogWrite for T where T: GetObjType + DeleteStatic {}


/// Implement this for all "Rust struct rows".
/// - Any SQL writes are logged when applied via `tx_insert_set_pk_log_write`.
impl<T> LogWrite for T where T: Insert + TableStatic + GetObjType + GetId {}


pub trait GetInferredDeletes: TableStatic {
    /// When: child (!has_direct_dl && !has_direct_events), delete can be inferred by the parent not including it in the next update object list (assuming that list is complete and not limited to 10 items).
    ///     - Or: a child type is listed and owned exclusively through one parent: the parent gets a delete event, but none of the children have delete events triggered.
    ///
    /// Note: This could lead to a `cudc` pattern.
    /// - E.g. (dl, first_apply_events).
    ///     - The first_apply_events will be applying events for child.items before the latest download.
    ///         - This will delete new items that are in head but where not in the parent.update event.
    ///         - And then it will re-create them for the last update.
    ///             - At tx commit, the state is still correct as the events are applied from before until after the dl point, applying events until the latest write.
    ///             - The issue is that the write log will have [c, u, d, c, u] for child items, which may be counter intuitive for readers expecing a clean [c, u, u, u, d] lifecycle.
    ///     - Q: Is this an issue for objects that can be deleted?
    ///         - Maybe:
    ///             - A deleted object:
    ///                 - Will not be in the download.
    ///                 - May have just the "deleted" event, with no prior create/update (because of the 28 day window).
    ///                     - Deletes are "delete if exists", so not an issue.
    ///
    fn get_inferred_deleted_items(utx: &mut UniTx, p_col: &str, p_id: &str, active: Vec<&str>) -> Vec<String> {
        let tbl = Self::get_table_name_static();
        let active_ids: String = active.iter().map(|x| format!("'{}'", x)).collect::<Vec<String>>().join(", ");
        let mut std_sql = format!("SELECT id FROM {} WHERE {} = :p_id", tbl, p_col);
        let mut pg_sql = format!("SELECT id FROM {} WHERE {} = $1", tbl, p_col);

        // SQL `IN ()` is an error in MySQL (but not SQLite).
        if active.len() > 0 {
            std_sql = format!("{} AND id NOT IN ({})", std_sql, active_ids);
            pg_sql = format!("{} AND id NOT IN ({})", pg_sql, active_ids);
        }

        match utx {
            UniTx::Rusqlite(tx) => {
                /// Note: SQL `IN` cannot be used without `carray` SQLite extension (dynamic num of params not supported in Rusqlite).
                /// @see https://github.com/rusqlite/rusqlite/issues/430
                /// @see https://sqlite.org/carray.html
                let mut stmt = tx.prepare_cached(&std_sql).unwrap();
                let mut rows = stmt.query_named(&[(":p_id", &p_id)]).unwrap();

                let mut ids = vec![];
                while let Some(row) = rows.next().unwrap() {
                    ids.push(row.get(0).unwrap());
                }
                return ids;
            }
            UniTx::MySQL(tx) => {
                use mysql::params;
                let params = params! {
                    "p_id" => &p_id
                };

                let res: Vec<String> = tx.exec(std_sql, params).unwrap();
                return res;
            }
            UniTx::Postgres(tx) => {
                return tx.query(pg_sql.as_str(), &[&p_id]).unwrap().into_iter().map(|x| x.get(0)).collect();
            }
            UniTx::PlaceholderLibA(_) => {}
        }

        unreachable!()
    }
}

impl<T> GetInferredDeletes for T where T: TableStatic {}
