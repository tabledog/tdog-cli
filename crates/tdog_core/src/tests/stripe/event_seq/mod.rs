use std::{sync::Mutex, time};
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::{Path};


use regex::Regex;
use rusqlite::{backup, Connection};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use stripe_client::types::types as API;

use unicon::traits::{*};
use unicon::uc::{*};

use crate::providers::stripe::apply_events::apply_events;
use crate::providers::stripe::schema::Db;

use crate::tests::stripe::util::{cp_to_temp_and_get_uc, init_log_output, path_from_cargo, read_file};
use stripe_client::http::http::{StripeClient, Config};

pub mod all;


type TagSeq = Vec<String>;


/// Snapshot the state at each step for easy debugging.
///
/// - It makes understanding the current state much easier than trying to analyse/visualize the write log.
/// - Adding a `dbg!()` to get a copy manually incurs the 30 second compile time.
///     - Snapshotting every state allows for analysing any state.
pub fn copy_db_file(db: &str, state: &str) -> String {
    let dest_dir = get_snapshot_dir(&db);

    fs::create_dir_all(&dest_dir).unwrap();

    let dest_file = format!("{}/{}.sqlite", &dest_dir, state);

    let from = Connection::open(db).unwrap();
    let mut to = Connection::open(&dest_file).unwrap();
    let backup = backup::Backup::new(&from, &mut to).unwrap();
    backup.run_to_completion(5, time::Duration::from_millis(20), None).unwrap();

    dest_file.into()
    // Note: When using WAL mode, the transaction that was just committed has not yet been merged into file.
    // Fix: Use backup API to make WAL-aware DB copies.
    // fs::copy(&db, &dest_file).unwrap();
}

fn get_snapshot_dir(db: &str) -> String {
    // E.g. `/private/var/folders/51/t7h4hg315p1gv56gzfg0jm1c0000gn/T/td-data/3c17acde-b151-4130-8a98-87ced5b5666d.sqlite`
    let path = Path::new(&db);

    // `3c17acde-b151-4130-8a98-87ced5b5666d`
    let file_stem = path.file_stem().unwrap();

    // `/private/var/folders/51/t7h4hg315p1gv56gzfg0jm1c0000gn/T/td-data/`
    let parent = path.parent().unwrap();


    // `/private/var/folders/51/t7h4hg315p1gv56gzfg0jm1c0000gn/T/td-data/3c17acde-b151-4130-8a98-87ced5b5666d`
    let dest_dir = format!("{}/{}", parent.to_string_lossy(), file_stem.to_string_lossy());

    dest_dir
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct EventSeq {
    pub key: String,
    pub events: Vec<API::NotificationEvent>,
    pub meta: Meta,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Meta {
    steps: Vec<Step>,
}

impl Meta {
    /// Alternative: Use Rust's range: `1..=3 = [1,2,3]`
    /// - Issue:
    ///     - Storing the start/end ranges leaves out information about inclusive=true/false.
    ///     - Groups of indexes will always have less than 5 digits making it easy to visually scan.
    ///     - indexes.map is more direct than constructing a range.
    fn get_indexes_for_ag_range_inclusive(&self, from: u32, to: u32) -> (Vec<u32>, Vec<u32>) {
        let mut ag_ids = vec![];
        let mut event_indexes = vec![];
        let range = from..=to;
        for ag in self.steps.iter().filter(|ag| range.contains(&ag.i)) {
            ag_ids.push(ag.i);
            event_indexes.extend(ag.event_indexes.clone());
        }

        (ag_ids, event_indexes)
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Step {
    i: u32,
    tags: Vec<String>,
    // DL at every ag is not possible (e.g. charge.succeed happens 3 seconds after a source is created; its not possible to complete a download deterministically in that time).
    download: Option<Dl>,
    event_indexes: Vec<u32>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Dl {
    file: String,
}

#[derive(Debug)]
struct Exec {
    // key? Human friendly ref?
    es: EventSeq,
    tag_seq: TagSeq,
    path: String,
    step: u32,
    cur_pos: VecDeque<AgSeq>,
    db_file: String,
    uc: UniCon,
}


struct PathParts {
    dl: String,
    fa_s: Option<String>,
    fa_e: Option<String>,
    rest: Option<Vec<String>>,
}


/// `x-(y).sqlite` in mac os iterm output will not allow clicking to open in DB GUI.
fn remove_iterm_issue_chars(s: &str) -> String {
    s.replace("(", "").replace(")", "")
}


fn assert_relations(db_file: &str) {
    let uc = UniCon::Rusqlite(ConMetaSQLite {
        create: SQLiteCreate {
            file: db_file.to_string()
        },
        c: Connection::open(&db_file).unwrap(),
    });

    let missing = Db::get_missing_owner_all(&uc);
    if missing.len() > 0 {
        dbg!(&missing);
    }
    assert_eq!(missing.len(), 0, "Foreign key constraints violated (note: not native SQL FK constraint).");
}

impl Exec {
    pub fn dl(&mut self, path_part: &str) {
        assert_eq!(self.step, 0);
        let ag_g = self.cur_pos.pop_front().unwrap();

        assert_eq!(ag_g.path_part, path_part);
        info!("DB: Running tests against {}", &self.db_file);

        // Check relations here instead of at the end of `download_all` with `#[cfg(test)]` so that:
        // - Download completes whilst writing/reading from Stripe account.
        //      - Error is thrown during the actual test.
        //      - DB can be inspected for missing relations.
        assert_relations(&self.db_file);

        let copy = copy_db_file(&self.db_file, format!("{}-dl-{}", self.step, remove_iterm_issue_chars(&path_part)).as_str());
        // dbg!(&copy);
        info!("DB: Snapshot: db file copied **after** DL was applied: {}", &copy);

        self.step += 1;
    }

    /// Moving from dl->first_apply has specific logic for merging from the dl state to the apply_events state.
    /// - Tag the AST to enable using IDE based find-all-usages.
    /// - Easier to read tests on first scan (dl, first_apply, apply_n).
    pub async fn first_apply(&mut self, path_part: &str) {
        assert_eq!(self.step, 1);
        self.apply(path_part).await;
    }


    pub async fn apply(&mut self, path_part: &str) {
        assert!(self.step > 0);
        let ag_g = self.cur_pos.pop_front().unwrap();
        assert_eq!(ag_g.path_part, path_part);


        let stripe_dummy = StripeClient::new(Config {
            secret_key: "dummy test".to_string(),
            is_test: true,
            base: "dummy test".to_string(),
            headers: None,
            proxy: None,
            timeout_ms: None,
            retry: false,
            log_requests: false,
        });

        let events = ag_g.event_indexes.iter().map(|ei| self.es.events[*ei as usize].clone()).collect();

        apply_events(&stripe_dummy, &mut self.uc, Some(events)).await;
        let copy = copy_db_file(&self.db_file, format!("{}-apply-{}", self.step, &path_part).as_str());
        // dbg!(&copy);
        info!("DB: Snapshot: db file copied **after** events were applied: {}", &copy);


        self.step += 1;
    }


    pub fn from_path(es: EventSeq, ts: TagSeq, path: &str) -> Self {
        let tag_indexes = es.get_tag_indexes(&ts);
        let ag_seq = Self::get_ag_seq(&es, &ts, &tag_indexes, path);

        // This is a specially added group to contain just the download file after all events for testing purposes - it contains no events.
        let last_ag = es.meta.steps.last().unwrap().i;

        let ag_id_for_dl = match ag_seq[0].ag_ids.last() {
            // Empty DL file.
            None => 0,
            Some(ag_id) => if *ag_id < last_ag {
                ag_id + 1
            } else {
                // When: DL contains all events (including the last special atomic group containing just the DL).
                last_ag
            }
        };
        let dl_file_opt = &es.meta.steps[ag_id_for_dl as usize].download;

        assert!(dl_file_opt.is_some(), "No download file at ag_id {}. Note: Downloads can be missing due to async events happening 3s after others that makes a full download impossible to complete (like the charge.succeeded for source creation). Fix: Do not use paths that reference this download point.", ag_id_for_dl);
        let dl_file = &(dl_file_opt.as_ref().unwrap().file);

        let (db_file, uc) = cp_to_temp_and_get_uc(&dl_file);


        Exec {
            es,
            tag_seq: ts,
            path: path.to_string(),
            step: 0,
            cur_pos: ag_seq.into(),
            db_file,
            uc,
        }
    }

    /// Converts a string path representing a given execution into parts.
    /// Example, valid paths:
    /// - (),c,u,d
    /// - (c),u,d
    /// - (c),u-u,d
    /// - (u),d
    /// - (u),d-d
    /// - (d),d
    pub fn parse_path(path: &str) -> PathParts {
        let re_tag = Regex::new(r"^[a-z0-9_]+$").unwrap();

        let parts: Vec<&str> = path.split(',').collect();

        let re = Regex::new(r"^\(([a-z0-9_]*)\)$").unwrap();
        let caps = re.captures(&parts[0]).unwrap();
        let dl = caps.get(1).unwrap().as_str().to_string();

        let mut fa_s = None;
        let mut fa_e = None;
        let mut rest: Option<Vec<String>> = None;

        if let Some(fa) = parts.get(1) {
            if fa.contains("-") {
                let x: Vec<&str> = fa.split("-").collect();
                assert!(re_tag.is_match(x[0]));
                assert!(re_tag.is_match(x[1]));
                fa_s = Some(x[0].to_string());
                fa_e = Some(x[1].to_string());
            } else {
                assert!(re_tag.is_match(fa));
                fa_e = Some(fa.to_string());
            }
        }

        if parts.len() > 2 {
            let r = parts[2..].to_vec();
            for x in &r {
                assert!(re_tag.is_match(x));
            }
            rest = Some(r.into_iter().map(|x| x.to_string()).collect());
        }

        // dbg!(&dl);
        // dbg!(&fa_s);
        // dbg!(&fa_e);
        // dbg!(&rest);

        PathParts {
            dl,
            fa_s,
            fa_e,
            rest,
        }
    }


    /// Converts a (tag_seq, path) into atomic group and event indexes
    /// - E.g. ([c, u, d], `(c),u,d`) = [[`(c)`, [0,1], [0,1,2,3,4]], ...]
    pub fn get_ag_seq(es: &EventSeq, _ts: &TagSeq, tag_indexes: &HashMap<String, (u32, u32)>, path: &str) -> Vec<AgSeq> {
        /// @todo/high Move this explanation into a .md file.
        /// @see https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=286e91cc20a8d121a3be092b6643cc04
        /// Valid matches:
        /// - (),c,u,d
        /// - (c),u,d
        /// - (c),u-u,d
        /// - (u),d
        /// - (u),d-d
        /// - (d),d
        ///
        /// Note:
        /// - `(x)` means "start with the download file at the point in time of after the last event in tag span x.
        ///     - It means "all writes for the events of x are in the database file".
        ///     - `()` means blank db (run on empty Stripe account).
        /// - `,` means "apply_events". Each comma represents a point where an assertion function can run to validate some state transition.
        /// - `b-c` means "start from tag b start index, to the *end* of tag c's span".
        ///     - This means that the meaning of the letter changes if its used as the start or end of the range.
        ///     - This can only be used for the first_apply events in order to remove any events (only ones which happened before teh download).
        ///         - This simulates downloading objects from a Stripe account where the events have been removed due to being older than 30 days (but the objects still exist for direct download).
        ///     - A first_apply without the range means "all events from 0 to the end of x".
        /// - `x` means "all events from the end of the prev tag used until the end of x".
        ///
        /// E.g. Given a tag_seq of [c, u, d]
        ///     - Invalid
        ///         - `(),d,c` - order incorrect
        ///         - `(c),d-d` - missing gap of `u` (download contains events c, first_apply tries to apply events d, which misses out set u.
        ///
        ///     - Valid, questionable:
        ///         - `(),c,d`
        ///             - With a tag_seq of c, u, d, `u` is missing.
        ///             - But if each letter means "everything from the end of previous to the end of this tag":
        ///                 - Given these events:
        ///                     - c = 0,1,2
        ///                     - u = 3,4
        ///                     - d = 5,6
        ///                 - Then:
        ///                     - 1. DB is empty
        ///                     - 2. `c` contains 0,1,2
        ///                     - 3. `d` contains 3,4,5,6 (everything from prev up until end).
        ///
        // let re = Regex::new(r"^\((?P<dl>[a-z0-9_]*)\),(((?P<fa_s>[a-z0-9_]+?)-)?(?P<fa_e>[a-z0-9_]+?))(?P<a>(,[a-z0-9_]+?){0,})$").unwrap();
        // let caps = re.captures(&path).unwrap();

        let pp = Exec::parse_path(path);
        let mut v = vec![];


        // Download
        {
            let mut ag_ids = vec![];
            let mut event_indexes = vec![];

            // let dl = &caps.name("dl").unwrap().as_str();
            if pp.dl.len() > 0 {
                // E.g. `(u)` will take events from 0 to the end of u.
                let (_ag_id_start, ag_id_end) = tag_indexes.get(&pp.dl.clone()).unwrap();
                let x = es.meta.get_indexes_for_ag_range_inclusive(0, *ag_id_end);
                ag_ids = x.0;
                event_indexes = x.1;
            }

            v.push(AgSeq {
                path_part: format!("({})", &pp.dl),
                ag_ids,
                event_indexes,
            });
        }


        // First apply
        {
            // fa_e does not have to be set, E.g. a test path can just test a download `(x)` that is the last path part.
            if let Some(fa_end) = &pp.fa_e {
                let mut path_part = "".into();
                let mut ag_ids = vec![];
                let mut event_indexes = vec![];


                // Can only skip leading events if they are already in the download file.
                // E.g. Given a seq of [0,1,2,3,4], you cannot start from the dl file at 2, skip 3 and then start applying at 4 onwards.
                // Assert: (download, first_apply, apply...) There are never messing events from the download->onwards, but events may be missing from download backwards.
                let dl = v.last().unwrap();
                let skippable = if let Some(ag_id) = dl.ag_ids.last() {
                    Some(ag_id)
                } else {
                    // Empty DB.
                    None
                };


                // Always set. Both `(),a-b` and `(),b` means first-apply-end = b
                let (_c, d) = tag_indexes.get(&fa_end.clone()).unwrap();
                let mut x = es.meta.get_indexes_for_ag_range_inclusive(0, *d);
                path_part = format!("{}", &fa_end);

                if let Some(fa_start) = &pp.fa_s {
                    let (a, _b) = tag_indexes.get(&fa_start.clone()).unwrap();
                    x = es.meta.get_indexes_for_ag_range_inclusive(*a, *d);
                    path_part = format!("{}-{}", &fa_start, &fa_start);

                    match skippable {
                        None => assert_eq!(*a, 0, "Empty DB file but first_apply range does not start at event seq 0; cannot leave event gaps."),
                        Some(last_ag_in_dl) => assert!((a - last_ag_in_dl) <= 1, "Cannot leave event gaps; when skipping leading events, all skipped events must be in the database (via direct download).")
                    }
                }

                ag_ids = x.0;
                event_indexes = x.1;


                v.push(AgSeq {
                    path_part,
                    ag_ids,
                    event_indexes,
                });
            }
        }

        // Rest of apply_events
        {
            if let Some(tags) = pp.rest {
                for t in tags {

                    // Each tag takes events greedily from the front, *but always ends at its own end point according to the tag_seq*.
                    // - E.g.
                    // - tag_seq=c,u,d
                    // - events=[-2,-1,c,0,1,2,u,3,4,d,5,6]
                    // - path=`(),c,d`
                    // - set c = -2,-1, 0, 1, 2 (ends here because u in tag_seq).
                    // - set d = 3, 4, 5, 6 (greedily takes from u because it is missing from path).
                    // This behavior is the default because there cannot be any gaps in the applied events (so the tags must greedily take from before OR after).
                    let (_a, b) = tag_indexes.get(&t.clone()).unwrap();
                    let prev_ag_id = v.last().unwrap().ag_ids.last().unwrap();
                    assert!(b > prev_ag_id, "Tag must represent an atomic group index that comes after the previously applied index. {} > {}", b, prev_ag_id);

                    let x = es.meta.get_indexes_for_ag_range_inclusive(prev_ag_id + 1, *b);

                    v.push(AgSeq {
                        path_part: t.clone(),
                        ag_ids: x.0,
                        event_indexes: x.1,
                    });
                }
            }
        }

        // Assert: This (tag_seq, path) combo uses all events.
        // - None skipped.
        // - All events assigned right to the end.
        // - If removing leading events, all remove events are in the DL.
        // - Note: path needs to be complete, but during testing they do not all need to be applied if not needed.
        let mut total_events = vec![];
        for x in &v {
            total_events.extend(x.event_indexes.clone());
        }
        total_events.sort();
        total_events.dedup();

        if es.events.len() != total_events.len() {
            dbg!(&v);
        }

        assert_eq!(es.events.len(), total_events.len());


        v
    }

    /// Get another connection to read state (simulate another process observing the DB) during tests.
    pub fn fork_uc(&self) -> UniCon {
        UniCon::new(&UniConCreate {
            engine: Engine::SQLite(SQLiteCreate {
                file: self.db_file.clone()
            })
        })
    }
}


/// A sequence of atomic groups (group of atomic groups).
/// - E.g. A tree structure of (atomic-group-groups -> atomic_group -> event_indexes).
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct AgSeq {
    path_part: String,
    ag_ids: Vec<u32>,
    event_indexes: Vec<u32>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct WalksCUD {}


/// Walk = an alias/key/name for a given (tag_seq, path) that can be used consistently between tests without having to re-read the (tag_seq, path) for each test.
/// - E.g. This tests a typical object lifecycle of (create, update, delete).
/// - Key.
///     - dl = download at this point.
///     - a = apply events.
///     - C, U, D = advance over these events.
///
/// 1.
///     - dl-C-a-U-a-D-a
/// 2-a.
///     - C-dl-U-a-D-a
/// 2-b.
///     - C-dl-U-a-D-a
/// 3-a.
///     - C-U-dl-D-a
/// 3-b.
///     - C-U-dl-D-a
/// 4.
///     - C-U-D-dl-a
///
/// Note: 2-b and 3-b have no leading events up until the dl on the first_apply (to test the logic of Stripe removing events after 30 days).
impl WalksCUD {
    fn get_tag_seq() -> TagSeq {
        vec![
            "c".into(),
            "u".into(),
            "d".into(),
        ]
    }
    fn get_walk_1(es: EventSeq) -> Exec {
        Exec::from_path(es, Self::get_tag_seq(), "(),c,u,d")
    }

    fn get_walk_2_a(es: EventSeq) -> Exec {
        Exec::from_path(es, Self::get_tag_seq(), "(c),u,d")
    }
    fn get_walk_2_b(es: EventSeq) -> Exec {
        // `u-u` means "start from u the end of u (remove all events before the dl point).
        Exec::from_path(es, Self::get_tag_seq(), "(c),u-u,d")
    }

    fn get_walk_3_a(es: EventSeq) -> Exec {
        Exec::from_path(es, Self::get_tag_seq(), "(u),d")
    }
    fn get_walk_3_b(es: EventSeq) -> Exec {
        Exec::from_path(es, Self::get_tag_seq(), "(u),d-d")
    }

    fn get_walk_4(es: EventSeq) -> Exec {
        Exec::from_path(es, Self::get_tag_seq(), "(d),d")
    }


    /// Start from an empty DB, apply all events in one go.
    fn get_walk_blank_db_apply_all(es: EventSeq) -> Exec {
        Exec::from_path(es, Self::get_tag_seq(), "(),d")
    }

    fn get_walk_blank_db_apply_u(es: EventSeq) -> Exec {
        Exec::from_path(es, Self::get_tag_seq(), "(),u,d")
    }
}


/// `cargo test` runs many tests in threads.
/// - A single Stripe test account is used to simplify admin.
///     - Assert: When creating lifecycles others wait until the Stripe account is free.
// static LOCK_STRIPE_ACC: Lazy<Mutex<()>> = Lazy::new(Mutex::default);
lazy_static! {
    static ref LOCK_STRIPE_ACC: Mutex<()> = Mutex::new(());
}

impl EventSeq {
    /// Checks the file system to see if the event seq has been saved as dir of JSON/SQLite files.
    /// - If it does not exist (first run, or deleted in order to trigger re-create), it will be created by writing to a Stripe test account.
    pub fn from_local_dir(es_key: &str) -> Self {
        init_log_output();

        let es_dir = path_from_cargo(&format!("src/tests/stripe/event_seq/data/{}", &es_key));
        assert!(Path::new(&es_dir).exists(), "Timeline key must match a local dir with JSON data in it. Create them with the `stripe-simulator` repo.");
        Self::from_dir(&es_dir, &es_key)
    }


    pub fn from_dir(dir: &str, es_key: &str) -> Self {
        let e: Map<String, Value> = serde_json::from_str(read_file(&format!("{}/events.json", &dir)).as_str()).unwrap();
        let events: Vec<API::NotificationEvent> = serde_json::from_value(e.get("events").unwrap().clone()).unwrap();

        let mut meta: Meta = serde_json::from_str(read_file(&format!("{}/meta.json", &dir)).as_str()).unwrap();


        // Make DB files absolute.
        for ag in &mut meta.steps {
            if let Some(dl) = &mut ag.download {
                dl.file = format!("{}/{}", &dir, &dl.file);
            }
        }

        EventSeq {
            key: es_key.into(),
            events,
            meta,
        }
    }

    // Gets the start and end indexes of the given tag seq.
    pub fn get_tag_indexes(&self, ts: &TagSeq) -> HashMap<String, (u32, u32)> {
        let mut hm = HashMap::new();
        let tag_seq = ts.clone();

        let indexes = tag_seq.iter().map(|tag| {
            let mut all = vec![];
            for ag in &self.meta.steps {
                if ag.tags.contains(&tag) {
                    all.push(ag.i);
                }
            }
            assert_eq!(all.len(), 1, "Tag can only be used once.");
            (tag, all[0])
        });

        let last_ag_id = self.meta.steps.last().unwrap().i;

        let i_clone: Vec<_> = indexes.collect();
        for (i, (tag, ag_id)) in i_clone.iter().enumerate() {
            let mut end_ag_id = last_ag_id;


            if let Some(next_tag) = i_clone.get(i + 1) {
                end_ag_id = next_tag.1 - 1
            }

            // This tag spans from ag_id-end_ag_id, including both (ag_id and end_ag_id).
            // Note: The first tag starts at its ag_id, but the last tag will span right to the end of the event seq.
            hm.insert(tag.to_string(), (*ag_id, end_ag_id));
        }

        hm
    }
}

