#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
//use unicon::{UniCon, ConMetaSQLite, SQLiteCreate, Engine, UniConCreate};
use serde::{Deserialize, Serialize};
//use unicon::uc::Engine;

use unicon::uc::{*};
use crate::util::{get_temp_file, get_unique_id};

#[allow(unused)]
mod fns;

#[allow(unused)]
pub mod util;

#[allow(unused)]
pub mod providers;


#[cfg(test)]
mod tests;


// Note: `enum` used here in anticipation of many different network interfaces (Web API, CLI).
// - Those interfaces should map to this enum (instead of using a web/cli framework that generates incompatible enums).

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
// Use fn/args as it is intuitive for developers - they already have a mental model of "functions take arguments, a different function takes different arguments".
// - Also allows adding a broad range of features in the future, and explicitly selects an enum variant (as opposed to "untagged" enums choosing the first one that matches the data shape).
#[serde(tag = "fn", content = "args")]
pub enum Cmd {
    #[serde(rename = "download")]
    Download(Download)
}


impl Cmd {
    pub async fn run(&self) {
        match self {
            Cmd::Download(dl) => {
                if dl.options.watch {
                    dl.watch().await;

                    // if let Some(f) = after_run {
                    //     f(None, 123);
                    // }
                } else {
                    dl.download_all().await;
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Download {
    pub from: FromAPI,
    pub to: Engine,

    #[serde(default)]
    pub options: Options,

    // #[serde(skip)]
    // pub run_complete_cb: Option<Box<dyn FnMut(&mut UniCon, i64)>>
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum FromAPI {
    // #[serde(rename = "github_repo")]
    // GithubRepo(GithubRepo),

    #[serde(rename = "stripe")]
    Stripe(Stripe)
}

// #[derive(Serialize, Deserialize)]
// #[derive(Debug, Clone)]
// pub enum To {
//     #[serde(rename = "sqlite")]
//     SQLite(SQLiteCreate),
//
//     #[serde(rename = "mysql")]
//     MySQL(MySQL),
//
//     #[serde(rename = "excel")]
//     Excel(Excel),
// }

// impl To {
// Returns an active connection.
// fn into_unicon(&self) -> UniCon {
//     UniCon::new(&UniConCreate { engine: self.clone() })
// }
// }


// @todo/low,security Do not `impl` directly on these structures - explicitly map to safe structures to prevent unsafe data from JSON.
#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct GithubRepo {
    pub user: String,
    pub repo: String,
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Stripe {
    // pub publishable_key: String,
    pub secret_key: String,

    pub max_requests_per_second: Option<u32>,

    #[serde(default = "default_exit_on_429")]
    pub exit_on_429: bool,

    // HTTP proxy config directly on Stripe config.
    // - Global HTTP setting may set this if it is None in the future.
    // - Makes it clear what the HTTP proxy effects (just Stripe, not DB connections).
    // - Reqwest will fall back onto auto using system proxy.
    // - Proxy config data is local to the `create_stripe_client(x)`, so less passing data around, easier to keep code modular/library like.
    // - Allow HTTP header/option overrides specifically for Stripe.
    pub http: Option<HttpOpts>,
}

fn default_exit_on_429() -> bool {
    false
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct SQLite {
    #[serde(default = "get_temp_sqlite_file")]
    pub file: String,
}

// @todo/important Ensure the user can give their own file path.
// - E.g. if used in a webserver the user request should not be able to decide where the file is.
// Assume writing to SQLite file first and then converting to Excel or other formats later.
fn get_temp_sqlite_file() -> String {
    get_temp_file(get_unique_id() + ".sqlite").unwrap()
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Excel {}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct MySQL {}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Options {
    #[serde(default = "default_bool_false")]
    pub watch: bool,

    // When: watch = false, the user expects to download the account and exit afterwards.
    // - In this case, default to applying the events up until now on top of the initial download.
    // - This makes the file up to date until now (any writes occurring since the start of the download will be applied).
    // - When false, this option is useful in testing to simulate the case when events have been deleted after 30 days by Stripe.
    //      - Testing needs to simulate a download with no events applied so that the state of the DB can be checked for consistency.
    #[serde(default = "default_bool_true")]
    pub apply_events_after_one_shot_dl: bool,

    // Allow the user to customize this to self-fix possible issues:
    // - Stripe may rate limit polling, or the user may be running many instances of TD against the same Stripe account.
    // - The DB may have issues with many small transactions, but be OK with one per minute.
    // - The user may not need "real time" polling.
    pub poll_freq_ms: Option<u32>,
}

fn default_bool_false() -> bool { false }
fn default_bool_true() -> bool { true }

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct HttpOpts {
    // Roughly equal to Linux env var `http_proxy`
    pub proxy: Option<HttpProxyOpts>,

    // Connection options for proxy->api-server here.
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct HttpProxyOpts {
    // Connection options for td-client->proxy here.
    pub url: Option<String>,
}



