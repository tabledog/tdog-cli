use std::{env, fs};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Once;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env::var;

use rusqlite::Connection;
use regex::Regex;
// use unicon::DbStatic;

use rusqlite::types::Value;
use serde_json::{json, Map, Number};
use stripe_client::types::types as API;
use unicon::traits::{*};
use unicon::uc::{*};

use crate::*;
use crate::providers::stripe::schema::Db;
use crate::providers::stripe::schema_meta::{ResActionsTaken, TdStripeWrite};

static INIT: Once = Once::new();

pub fn init_log_output() {
    INIT.call_once(|| {
        // let mut builder = Builder::init_from();
        // builder.target(env_logger::Target::Stdout); // Stderr by default?
        // builder.init();
        env_logger::init_from_env(
            env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info")
        );
    });
}


lazy_static! {
    pub static ref TD_STRIPE_SECRET_KEY_TEST: String = var("TD_STRIPE_SECRET_KEY_TEST").unwrap();
}


pub fn path_from_cargo(s: &str) -> String {
    let mut from = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    from.push(s);
    from.to_str().unwrap().to_string()
}


/// Runs a download on the account, writing to the given db file.
pub async fn download_account(secret_key: &str, db_file: &str) {
    let dl = json!({
                "download": {
                    "from": {
                        "stripe": {
                            "secret_key": secret_key
                        }
                    },
                    "to": {
                        "sqlite": {
                            "file": &db_file
                        }
                    },
                    "options": {
                        "watch": false
                    }
                }
            });


    let m: Cmd = serde_json::from_str(&dl.to_string()).expect("Ok");
    m.run().await;
}

pub fn get_unicon(file: &str) -> UniCon {
    UniCon::Rusqlite(ConMetaSQLite {
        create: SQLiteCreate {
            file: file.to_string()
        },
        c: Connection::open(file).unwrap(),
    })
}

pub fn get_db_snapshot(db: &str) -> String {
    let file = path_from_cargo(db);
    if !Path::new(&file).exists() {
        panic!("DB snapshot does not exist: {}. Be sure to create it with the Stripe Simulator timeline tool.", &file)
    }
    file
}

pub fn read_file(file: &str) -> String {
    let file = path_from_cargo(file);
    fs::read_to_string(file.as_str()).unwrap()
}

/// Usage: `cargo test x -- --ignored g1`
pub fn get_group_from_cli_arg() -> String {
    let args: Vec<String> = env::args().collect();
    let re = Regex::new(r"^g\d$").unwrap();
    args.iter().find(|x| re.is_match(x)).unwrap().clone()
}


/// @todo/next remove all unused functions.
/// Usage: `cargo test x -- --ignored g1`
pub fn get_cli_timeline_walk() -> (String, String) {
    let args: Vec<String> = env::args().collect();
    let re = Regex::new(r"^walk_\d$").unwrap();
    let walk = args.iter().find(|x| re.is_match(x)).unwrap().clone();


    let re = Regex::new(r"^\d{2}$").unwrap();
    let timeline = args.iter().find(|x| re.is_match(x)).unwrap().clone();

    (timeline, walk)
}


/// Converts `k1:v1 k2:v2` into a Hashmap.
/// - Usage: `cargo test x -- --ignored k1:v1 k2:v2`
/// @todo/low Use a single JSON string instead?
pub fn get_cli_stripe_account_target_file() -> HashMap<String, String> {
    let args: Vec<String> = env::args().collect();

    // E.g. `k1:v1 k2:v2` (no white space in values or keys).
    let re: Regex = Regex::new(r"([^\s]+):([^\s]+)").unwrap();
    let one_str = args.join(" ");
    let caps = re.captures_iter(&one_str);
    let mut hm = HashMap::new();
    for c in caps {
        hm.insert(c[1].to_string(), c[2].to_string());
    }
    hm
}


pub async fn download_stripe_account_to_db_file(key_sec: &'static str, db: String) -> String {
    let file = path_from_cargo(db.as_str());

    if Path::new(&file).exists() {
        info!("Deleting existing Stripe snapshot file, {}", file);
        fs::remove_file(&file).unwrap();
    }

    info!("Downloading Stripe account to db snapshot file, {}", file);
    download_account(key_sec, &file).await;
    // Note: may need to `PRAGMA wal_checkpoint` to merge -wal files into `.sqlite` file.
    file
}


pub fn cp_to_temp_and_get_uc(from: &String) -> (String, UniCon) {
    let to = get_temp_file(get_unique_id() + ".sqlite").unwrap();
    // Note: May need to use backup API to prevent issues with WAL files not being applied.
    fs::copy(&from, &to).unwrap();
    let uc = get_unicon(&to);
    (to, uc)
}

/// Some functions can be tested without calling the Stripe server.
/// - In these cases pass incorrect keys which would cause an error for the server-contacting code paths.
pub fn unused_stripe_keys() -> Stripe {
    Stripe { secret_key: "not_used".to_string(), max_requests_per_second: None, exit_on_429: false, http: None }
}


/// Usage: `cargo test x -- --ignored 04 walk_1`
/// - Run from node/exec to download a Stripe account at a given point.
/// @todo/next delete this function if no longer used.
#[tokio::main]
#[test]
#[ignore]
pub async fn download_stripe_acc_timeline_walk() {
    let (timeline, walk) = get_cli_timeline_walk();

    let file = format!("src/tests/stripe/timelines/{}/data/walks/{}/d.sqlite", &timeline, &walk);
    download_stripe_account_to_db_file(&TD_STRIPE_SECRET_KEY_TEST, file).await;
}

/// For debugging why cargo will re-compile everything when an env var is different.
pub fn write_env_to_file() {
    let mut o: String = "".to_string();
    for (k, v) in env::vars() {
        o.push_str(&format!("{}={}\n", k, v));
    }

    let f = format!("/tmp/del-env-{}.txt", get_epoch_ms());
    let mut file = File::create(&f).unwrap();
    file.write_all(&o.into_bytes()).unwrap();
}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}




lazy_static! {
        static ref GRP: Regex = Regex::new(r"g(\d+)\.json$").unwrap();
    }

/// Reads a dir of files into Rust state for easy use during test writing.
#[derive(Debug)]
pub struct TimelineWalk {
    pub timelineId: String,
    pub walkId: String,
    pub db_temp: String,
    pub uc: UniCon,
    pub events: Vec<Vec<API::NotificationEvent>>,
}

impl TimelineWalk {
    pub fn read_from_files(timeline: &str, walk: &str) -> TimelineWalk {
        let path = format!("src/tests/stripe/timelines/{}/data/walks/{}/d.sqlite", &timeline, &walk);
        let db = get_db_snapshot(&path);
        let (temp, uc) = cp_to_temp_and_get_uc(&db);
        info!("Timeline {}/{} using db file {}.", &timeline, &walk, &temp);

        let events_by_group = TimelineWalk::read_event_groups(&timeline, &walk);

        TimelineWalk {
            timelineId: timeline.into(),
            walkId: walk.into(),
            db_temp: temp,
            uc,
            events: events_by_group,
        }
    }

    /// Reads files `g0.json, g1.json, ...`,
    /// - Outputs `[g0, g1, ...]`
    pub fn read_event_groups(timeline: &str, walk: &str) -> Vec<Vec<API::NotificationEvent>> {
        let walk_dir = path_from_cargo(&format!("src/tests/stripe/timelines/{}/data/walks/{}/", &timeline, &walk));
        let paths = fs::read_dir(&walk_dir).unwrap();

        let mut groups = vec![];
        for path in paths {
            let file = path.unwrap().path().display().to_string();
            let caps = GRP.captures(&file);
            if caps.is_some() {
                let index: i64 = caps.unwrap().get(1).unwrap().as_str().parse().unwrap();
                groups.push((index, file));
            }
        }

        groups.sort_by(|a, b| a.0.cmp(&b.0));
        let mut events = vec![];
        for g in groups {
            // Assumption: files are numbered 0..x.
            let ev: Vec<API::NotificationEvent> = serde_json::from_str(read_file(&g.1).as_str()).unwrap();
            events.push(ev);
        }

        events
    }
}


/// Ensure Stripe events map to the correct SQL write events.
/// - The important part is the `type=action` mapping.
/// - Assumption: A row in `td_stripe_apply_events` means that the data is correctly written to the given types table.
/// - Assumption: First download and first apply events merge will leave relations consistent.
pub fn assert_type_action_map(a: &Vec<ResActionsTaken>, b: &Vec<ResActionsTaken>) {
    for (i, a_1) in a.iter().enumerate() {
        let a_2 = &b[i];

        /// Ignore object id's as they will be different when re-creating the snapshot.
        assert_eq!(
            (&a_1.r#type, &a_1.action),
            (&a_2.r#type, &a_2.action)
        );
    }
}


// (td_stripe_apply_events.data_data_object, notification_events.type, td_stripe_apply_events.action)
pub type EventTypeToAction = (&'static str, &'static str, &'static str);

pub fn assert_type_action_map_tuple(a: &Vec<EventTypeToAction>, b: &Vec<ResActionsTaken>) {
    for (i, correct) in a.iter().enumerate() {
        let actual = &b[i];
        let (_, r#type, action) = correct;

        /// Ignore object id's as they will be different when re-creating the snapshot.
        assert_eq!(
            (&r#type.to_string(), &action.to_string()),
            (&actual.r#type, &actual.action)
        );
    }
}


// td_stripe_writes: (run_id, table_name, write_type)
pub type StripeWrite = (i64, &'static str, &'static str);

pub fn assert_writes_eq(a: &Vec<StripeWrite>, b: &Vec<TdStripeWrite>) {
    for (i, correct) in a.iter().enumerate() {
        let actual = &b[i];

        let c = (
            correct.0,
            &correct.1.to_string(),
            &correct.2.to_string()
        );

        assert_eq!(
            c,
            (actual.write_id.unwrap(), &actual.table_name, &actual.write_type)
        );
    }
}


/// Map to serde value which is more general, and can easily be ported to JSON.
pub fn to_serde_value(v: Value) -> serde_json::Value {
    match v {
        Value::Null => serde_json::Value::Null,
        Value::Integer(v) => serde_json::Value::Number(v.into()),
        Value::Real(v) => serde_json::Value::Number(Number::from_f64(v).unwrap()),
        Value::Text(v) => {
            // If SQL string is JSON parse and return that.
            if (v.starts_with("{") && v.ends_with("}")) || v.starts_with("[") && v.ends_with("]") {
                if let Ok(val) = serde_json::from_str(&v) {
                    return val;
                }
            }
            serde_json::Value::String(v)
        }
        Value::Blob(_) => serde_json::Value::Null,
    }
}


/// Converts a SQLite database into a Rust HashMap to easily test state transitions.
/// - Does not require listing all the columns or hand mapping a column to a given type.
/// - Keys are JS variables from the life cycle JS function.
///
/// - Stripe `metadata` used on objects to store the variable ID.
/// - JS variable IDs are stable across different runs
/// - Static types are not used as this checks from the users perspective what state is visible.
///     - SQL is not typed; user code will be dynamically reading from SQL without type defs.
///     - The benefits of SQL is the ability to write SQL programs that (join, filter) in an efficient way without having to bring the whole dataset into the application.
///         - This function treats the SQL database as a single large dynamic hash.
///         - SQL programs will be built by end users that leverage this state.
///
/// @todo/low Strongly type this by using the RowStruct serde traits.
/// - This will make client code cleaner as it does not need to destructure every scalar value from `Value` with `match`.
/// - Need to use `Value` instead of `String` for JSON fields.
/// - `HashMap<String, DbSchema>`
pub fn get_db_as_hm_by_test_id(uc: &mut UniCon) -> HashMap<(String, String), Map<String, serde_json::Value>> {
    let mut hm = HashMap::new();
    match uc {
        UniCon::Rusqlite(x) => {
            let con = &mut x.c;

            let tx = con.transaction().unwrap();
            let tbls = Db::get_table_names();

            for t in tbls {
                let mut stmt = tx.prepare_cached(format!("SELECT * FROM {}", t).as_str()).unwrap();
                let mut rows = stmt.query([]).unwrap();
                // let mut one_tbl = vec![];
                while let Some(row) = rows.next().unwrap() {
                    let mut one_rw = Map::new();
                    for (i, name) in row.column_names().iter().enumerate() {
                        one_rw.insert(name.to_string(), to_serde_value(row.get(i).unwrap()));
                    }

                    // Indexed by `test_id`, if Stripe metadata is set.
                    if let Some(md) = one_rw.get("metadata") {
                        if let serde_json::Value::Object(o) = md {
                            if let Some(v) = o.get("tid") {
                                if let serde_json::Value::String(test_id) = v {
                                    // Note: Two different object types can have the same ID ((source, x) === (payment_method, x)).
                                    let old_val = hm.insert((t.to_string(), test_id.to_string()), one_rw);
                                    assert_eq!(old_val, None, "test_id used more than once.");
                                }
                            }
                        }
                    }
                }
            }

            return hm;
        }
        _ => unreachable!("Unsupported test DB engine.")
    }
}


/// Same as above but uses Stripe ID.
/// - Note: keys are (type, id) because the same id can be used for many types.
pub fn get_db_as_hm_by_stripe_id(uc: &mut UniCon) -> HashMap<(String, String), Map<String, serde_json::Value>> {
    let mut hm = HashMap::new();
    match uc {
        UniCon::Rusqlite(x) => {
            let con = &mut x.c;

            let tx = con.transaction().unwrap();
            let tbls = Db::get_table_names();

            for t in tbls {
                let mut stmt = tx.prepare_cached(format!("SELECT * FROM {}", t).as_str()).unwrap();
                let mut rows = stmt.query([]).unwrap();
                // let mut one_tbl = vec![];
                while let Some(row) = rows.next().unwrap() {
                    let mut one_rw = Map::new();
                    for (i, name) in row.column_names().iter().enumerate() {
                        one_rw.insert(name.to_string(), to_serde_value(row.get(i).unwrap()));
                    }

                    if let Some(x) = one_rw.get("id") {
                        if let serde_json::Value::String(x2) = x {
                            let old_val = hm.insert((t.to_string(), x2.clone().to_string()), one_rw);
                            assert_eq!(old_val, None, "Stripe id used more than once.");
                        }
                    }
                }
            }

            return hm;
        }
        _ => unreachable!("Unsupported test DB engine.")
    }
}