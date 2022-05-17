use lazy_static::lazy_static;
use regex::{Regex, Captures};
use anyhow::{Result};
use colored::*;
use serde::{Deserialize, Serialize};
use log::{Record, Level, Metadata};
use std::collections::HashMap;
use std::path::{Path, Component};
use tdog_core::providers::stripe::schema::ToJSONKey;
use serde_json::Value;
use atty::Stream;

// static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger {
    config: LogConfig,

    // Cached computed fields to avoid re-computing on every log entry.
    // Maximum level in any of the config's.
    max_level: log::Level,
}

impl ConsoleLogger {
    fn include_in_output(&self, record: &Record) -> bool {
        let mut inc_log = false;
        let config = &self.config;
        assert!(config.level.is_some() || config.groups.len() > 0, "Must provide a logging level.");

        // Global
        if let Some(ref x) = config.level {
            // @todo/low Store log:Level instead of LevelString to avoid computing it on every entry.
            let level: Level = x.into();
            inc_log = record.level() <= level
        }

        // Specific mod group - overrides global if key is set. (can have global=info, but mod-group-x=error)
        if config.groups.len() > 0 {
            if let Some(x) = config.groups.get(&record.into()) {
                if let Some(ref x2) = x {
                    let level: Level = x2.into();
                    inc_log = record.level() <= level
                } else {
                    // Key exists, is null/None = do not log this group.
                    inc_log = false;
                }
            }
        }

        inc_log
    }

    fn get_line(&self, record: &Record) -> String {
        let mod_key: String = if self.config.friendly_mod_names {
            let g: Group = record.into();

            match g {
                // E.g: `"tdog"`
                Group::App | Group::AppLib => g.to_json_key(),

                // Keep full mod for cargo deps.
                Group::Dep => record.target().to_string()
            }
        } else {
            // E.g: `"hyper::proto::h1::role"`
            record.target().to_string()
        };


        // @see https://github.com/daboross/fern/issues/62#issuecomment-638660876
        // @see https://github.com/daboross/fern/issues/63

        let ms = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");

        // Trailing JSON in message:
        // - Human user (terminal), pretty print for easier reading.
        // - Machine user (log collecting daemon, redirect to file etc), keep trailing JSON as a single line to prevent breaking it up into many log collecting messages (human user can pretty print later when reading).
        let mut msg = format!("{}", record.args());
        let is_human_at_terminal = atty::is(Stream::Stdout);
        if is_human_at_terminal {
            msg = format_json_at_end_of_string(msg.as_str())
        }

        format!(
            "[{} {} {}] {}",
            ms,
            level_to_string_color(record.level()),
            mod_key,
            msg
        )
    }
}

// Matches JSON objects (array or object) at the end of a log line, and then parses and pretty prints it.
// - Leaves the original log message on its own line, formatted json starts on a new line.
//      - This will probably break log collecting daemons; only use when the output is a terminal.
// - The intent is to make it easier to scan read the CLI output when a human is casually viewing the output.
pub fn format_json_at_end_of_string(s: &str) -> String {
    lazy_static! {
        // Note: Will not work with multiple top level JSON objects in one message, or JSON with JSON strings inside it.
        static ref RE_OBJ: Regex = Regex::new(r###" (\{".+?})$"###).unwrap();
        static ref RE_AR: Regex = Regex::new(r###" (\[.+?])$"###).unwrap();
    }

    let replacer = |caps: &Captures| {
        let x = caps.get(1).unwrap();
        let r: Result<Value, _> = serde_json::from_str(x.as_str());
        if let Ok(json) = r {
            let buf = Vec::new();

            // 4 space indent; easier to read.
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
            let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
            json.serialize(&mut ser).unwrap();
            return format!("\n{}", String::from_utf8(ser.into_inner()).unwrap());

            // return serde_json::to_string_pretty(&json).unwrap();
        }

        format!(" {}", x.as_str().to_string())
    };


    if s.ends_with("}") {
        let result = RE_OBJ.replace(s, replacer);
        return result.to_string();
    }

    if s.ends_with("]") {
        let result = RE_AR.replace(s, replacer);
        return result.to_string();
    }


    // No change.
    s.into()
}

fn level_to_string_color(l: Level) -> String {
    let s = l.clone().to_string().bold();
    match l {
        Level::Error => s.red(),
        Level::Warn => s.red(),
        Level::Info => s.green(),
        Level::Debug => s.green(),
        Level::Trace => s.green()
    }.to_string()
}


impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.max_level
    }

    fn log(&self, record: &Record) {
        let inc = self.enabled(record.metadata()) && self.include_in_output(record);

        if inc {
            // eprintln exists for errors/progress, outputs to stderr

            // stdout
            println!("{}", self.get_line(record));
        }
    }

    fn flush(&self) {}
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Default)]
#[serde(default)]
pub struct LogConfig {
    // Global log level (for all modules inc deps).
    pub level: Option<LevelString>,

    // Log level for specific groups of modules.
    // - If key missing, default to global log level.
    // - If key present, override global.
    pub groups: HashMap<Group, Option<LevelString>>,

    // Rename modules for casual end users of the CLI (they do not need to know exactly what module high level logs come from, only what the CLI is doing in general at a high level).
    // - E.g: `tdog_core::providers::stripe::watch` => `tdog`
    // - This is an option as there are two "use cases" of logging:
    //      - 1. Casual user - tell me what the CLI is doing during successful operation at a high level in a concise way.
    //      - 2. Remote debugging (tdog developer) - bug report from end user that is only reproducible in their exact environment (that only they can access).
    //          - Enables "turn on full logging, send the full log" option - logs can be filtered after the fact (instead of using complex logging options here).
    pub friendly_mod_names: bool,
}


impl LogConfig {
    // Get the max log level in this config (Note: Trace is highest, Error is lowest).
    fn get_max_log_level(&self) -> Option<log::Level> {
        let mut all: Vec<log::Level> = vec![];

        if let Some(x) = &self.level {
            all.push(x.into());
        }

        for (_grp, level_opt) in self.groups.iter() {
            if let Some(x) = level_opt {
                all.push(x.into());
            }
        }

        all.into_iter().max()
    }

    pub fn friendly_casual_logs(app_level: log::Level) -> LogConfig {
        let mut groups = HashMap::new();
        groups.insert(Group::App, Some((&app_level).into()));
        groups.insert(Group::AppLib, Some(LevelString::Warn));
        groups.insert(Group::Dep, Some(LevelString::Warn));

        LogConfig {
            level: None,
            groups,
            friendly_mod_names: true,
        }
    }
}


// Group modules by level of abstraction to allow:
// 1. Log filtering so only the highest `app` level is output for end users during successful operation.
//      - Goal: Enable a pleasant CLI interaction with minimal user-eye-parsing-logs/unneeded information.
// 2. Global configuration for debugging issues.
//      - Goal: Quickly find the source of any bugs in remote systems using only the logs.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
pub enum Group {
    // Highest level code that is the "control plane" of the libraries below.
    #[serde(rename = "tdog")]
    App,

    // Custom general libraries that enable the `app` level.
    // - Self written for direct use in `app`; bugs may occur between the interface between this level and `app`, so logging may reveal this.
    #[serde(rename = "tdog_lib")]
    AppLib,

    // Anything from Cargo, likely to be the lowest layer/most general.
    #[serde(rename = "dep")]
    Dep,
}

impl From<&Record<'_>> for Group {
    fn from(x: &Record<'_>) -> Self {


        if let Some(x2) = x.file() {
            let p = Path::new(x2);

            // "cli/src/main.rs" = current workspace = app
            if !p.is_absolute() {
                return Group::App;
            } else {
                let is_cargo_crate = p.components().any(|x| match x {
                    Component::Normal(s) => s == ".cargo",
                    _ => false
                });


                // "/Users/x/.cargo/registry/*" = dep = cargo dependency (networking, files etc).
                if is_cargo_crate {
                    return Group::Dep;
                }


                // "/Users/x/Dev/y/*" = app-lib = general libraries, possibly created for TD.
                // Examples: (unicon, stripe-client).
                return Group::AppLib;
            }
        }

        unreachable!("uncategorized log record {:?}", &x);
    }
}


// `log::Level` does not implement Serde traits.
// - Cannot use serde `remote` attr as it only works for types without containers (like generics Option or HashMap).
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum LevelString {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "trace")]
    Trace,
}

impl From<&LevelString> for Level {
    fn from(x: &LevelString) -> Self {
        match x {
            LevelString::Error => Level::Error,
            LevelString::Warn => Level::Warn,
            LevelString::Info => Level::Info,
            LevelString::Debug => Level::Debug,
            LevelString::Trace => Level::Trace
        }
    }
}


impl From<&Level> for LevelString {
    fn from(x: &Level) -> Self {
        match x {
            Level::Error => LevelString::Error,
            Level::Warn => LevelString::Warn,
            Level::Info => LevelString::Info,
            Level::Debug => LevelString::Debug,
            Level::Trace => LevelString::Trace
        }
    }
}


// Logging.
// - Log to stdout, allow scheduler to route logs to a log collector.
//
// - Single format for now (ts, level, message), JSON lines format in the future.
//      - Possibly re-write each `log!(x)` fn call to allow passing structured JSON-like meta data.
//      - A log is a breakable interface.
//          - No users should expect the log format to stay the same. They should not be using log messages as implicit events.
//          - Can be improved in later versions.
// - "fail and reboot".
//      - Instead of bubbling errors into (a ancestor function OR a neat message) and trying to keep the process alive, just allow the process to exit.
//          - Moving unrecoverable errors from their line of origin may actually be harder to debug.
//          - This also saves time as `unwrap` can be used in place.
//      - systemd/docker can restart processes that fail every 5 seconds.
//          - Instead of re-trying network/db connections, just allow the process to exit and retry.
//              - Uptime of these connections is up to the end user, not TD.
//      - Log the function stack trace (it is probably easier to debug than a single message).
//      - Restart to a known valid state (Erlang-like)
//
// - It is not possible to get the args passed to `log(x, ..args)`, so they cannot be placed into a `meta` key for JSON lines logging.
//      - @see https://stackoverflow.com/questions/68201732/getting-the-pieces-that-make-up-an-arguments-instance
//
// - @todo/next
//      - Uptime monitoring.
//          - Human: When `Applied x events` is missing for 5 minutes, page someone.
//              - @ee https://cloud.google.com/monitoring/alerts/policies-in-json#json-metric-absence
//          - Downstream processes.
//              - Heartbeat SQL column (last ts of last check, regardless of events processed - 0 indicates no events to process but TD process is still operational/up to date).
pub fn init_log(config: LogConfig) {
    let max_level = config.get_max_log_level().expect("Must provide at least one log level.");

    let cl = ConsoleLogger {
        config,
        max_level,
    };


    // Static, will not work when using runtime state.
    // log::set_logger(&CONSOLE_LOGGER);

    // Works with runtime config state.
    // @see https://docs.rs/log/0.4.14/log/#use-with-std
    log::set_boxed_logger(Box::new(cl)).unwrap();
    log::set_max_level(max_level.to_level_filter());
}


fn get_test_config() -> LogConfig {
    let app_only = {
        let mut groups = HashMap::new();
        groups.insert(Group::App, Some(LevelString::Info));
        groups.insert(Group::AppLib, Some(LevelString::Warn));
        groups.insert(Group::Dep, Some(LevelString::Warn));

        let x = LogConfig {
            level: None,
            groups,
            friendly_mod_names: true,
        };
        x
    };

    let _full = {
        let groups = HashMap::new();

        let x = LogConfig {
            level: Some(LevelString::Info),
            groups,
            friendly_mod_names: false,
        };
        x
    };

    let config = app_only;

    config
}
