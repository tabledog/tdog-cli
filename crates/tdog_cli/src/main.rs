use std::{env, fs};
use anyhow::{anyhow, Context, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use unicon::uc::{*};
use tdog_core::{Cmd, FromAPI};
use tdog_core::providers::stripe::schema_meta::get_cli_version;
use tdog_core::util::{Redact, REDACT_PLACEHOLDER, is_debug_build};
use crate::log_config::{init_log, LevelString, LogConfig};

mod test_parse_json;
pub mod log_config;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Config {
    pub cmd: Cmd,

    #[serde(default = "default_log")]
    pub log: LevelStringOrConfig,
}

impl Config {
    // Allow the user to pass a config that is missing keys (or keys are set to null).
    // - Serde does not allow you to configure defaults for structs defined in other crates.
    // - Some defaults depend on other values in the config (E.g. default target schema may depend on API source name).
    pub fn set_defaults(&mut self) {
        match &mut self.cmd {
            Cmd::Download(ref mut dl) => {
                let schema_name_default = match dl.from {
                    FromAPI::Stripe(_) => "stripe".to_string()
                };

                // Validation.
                match &dl.to {
                    Engine::MySQL(x) => {
                        if let Some(_db_name) = &x.db_name {
                            error!("MySQL: Use `schema_name` instead of `db_name`.");
                            panic!();
                        }
                    }
                    _ => {}
                }

                // Set defaults.
                match &mut dl.to {
                    Engine::SQLite(_) => {}
                    Engine::MySQL(x) => {
                        if x.schema_name.is_none() {
                            x.schema_name = Some(schema_name_default.clone())
                        }
                    }
                    Engine::Postgres(x) => {
                        if x.schema_name.is_none() {
                            x.schema_name = Some(schema_name_default.clone())
                        }
                    }
                }
            }
        }
    }
}

fn default_log() -> LevelStringOrConfig {
    LevelStringOrConfig::AppLevel(LevelString::Info)
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
#[serde(untagged)]
pub enum LevelStringOrConfig {
    // Casual user, short logs for successful operation to monitor high level operations.
    AppLevel(LevelString),

    // Full detailed logs to remote debug any errors, possibly deep in crate deps.
    Config(LogConfig),
}


// Allow users to use native CLI args like this:
// - `tdog --stripe-key abc --target db.sqlite --watch`
// - Less messing around with JSON, easy to type.
// - JSON config enables using JSON specification and more complicated configurations if needed.
fn from_native_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    let from = args.iter().position(|s| s == "--stripe-key");
    let to = args.iter().position(|s| s == "--target");
    let watch = args.iter().position(|s| s == "--watch").is_some();

    if let (Some(from_i), Some(to_i)) = (from, to) {
        if let (Some(from_v), Some(to_v)) = (args.get(from_i + 1), args.get(to_i + 1)) {
            let value = json!({
                "cmd": {
                    "fn": "download",
                    "args": {
                        "from": {
                            "stripe": {
                                "secret_key": from_v,
                            }
                        },
                        "to": {
                            "sqlite": {
                                "file": to_v
                            }
                        },
                        "options": {
                            "watch": watch
                        }
                    }
                }
            });
            return serde_json::to_string(&value).ok();
        }
    }
    None
}

// Use a single JSON object for args:
// - It easily maps to Rust structs using Serde.
// - Developers find it easier to read/write JSON than CLI args (both in text editors and language API's).
// - When you have >4 flags, JSON is easier to read (especially when using its nested structure).
// - Reduce the "weight" of code dependencies (reduce: compile time, attack surface area, backwards compatible behavior).
//
// @see https://www.reddit.com/r/rust/comments/8i5k3l/looking_for_a_tiniest_args_parser/
// - clap = 30k lines
// - getopt = 2k lines
fn get_json_str() -> Result<String> {
    let args: Vec<String> = env::args().collect();

    if let Some(s) = from_native_args() {
        return Ok(s);
    }

    for (i, x) in args.iter().enumerate() {
        match x.as_str() {
            // Secrets can be passed from `ENV vars -> bash JSON string -> TD memory` without persisting them to logs or disk.
            "--json" => {
                if let Some(s) = args.get(i + 1) {
                    return Ok(s.clone());
                }
            }
            // Strings in bash can be awkward.
            // JSON highlighting works against .json files in editors.
            // Users may have a large list of (from, to) targets, in which case JSON files may be better for organising.
            // Memory based temp files can also be used for secrets.
            "--json-file" => {
                if let Some(s) = args.get(i + 1) {
                    return Ok(fs::read_to_string(s).with_context(|| format!("Could not read --json-file `{}`", s))?);
                }
            }
            "--version" | "-v" => {
                println!("Version {}\nhttps://table.dog", get_cli_version());
                std::process::exit(0);
            }
            _ => {}
        }
    }

    Err(anyhow!("Missing JSON config. Pass `--json $json_string` or `--json-file $absolute_path_to_file` to the CLI."))
}


impl Config {
    fn from_cli_args() -> Result<Config> {
        let json_str = get_json_str()?;
        let x: Config = serde_json::from_str(&json_str).with_context(|| "Invalid JSON config.")?;
        Ok(x)
    }

    // Allow logging a Config instance, but API private keys or database passwords.
    // - Logging an instance may be useful if the CLI has many accounts/databases, and is reading the logs separate from the VM instance that is running the CLI.
    fn to_json_redact_private_keys(&self) -> String {
        let mut x: Config = (*self).clone();

        // let redact = "****redacted****";
        // let re = Regex::new("^(.{14}).+?(.{4})$").unwrap();

        match x.cmd {
            Cmd::Download(ref mut x) => {
                match x.from {
                    FromAPI::Stripe(ref mut x) => {
                        // x.secret_key = re.replace(x.secret_key.as_str(), format!("$1{}$2", &redact).as_str()).parse().unwrap();
                        x.secret_key = x.secret_key.as_str().redact(14, 2);
                    }
                }

                match x.to {
                    Engine::SQLite(_) => {}
                    Engine::MySQL(ref mut x) | Engine::Postgres(ref mut x) => {
                        if x.pass.is_some() {
                            x.pass = Some(REDACT_PLACEHOLDER.to_string());
                        }
                    }
                }
            }
        }

        serde_json::to_string(&x).unwrap()
    }


    fn get_log_config(&self) -> LogConfig {
        match &self.log {
            LevelStringOrConfig::AppLevel(app_level) => LogConfig::friendly_casual_logs(app_level.into()),
            LevelStringOrConfig::Config(c) => c.clone()
        }
    }
}


// #[tokio::main(threaded_scheduler)]
#[tokio::main]
async fn main() -> Result<()> {
    let mut x = Config::from_cli_args()?;
    init_log(x.get_log_config());

    x.set_defaults();

    info!("CLI version: {}", get_cli_version());
    if is_debug_build() {
        info!("This is a Rust debug build.");
    }
    info!("Using config: {}", &x.to_json_redact_private_keys());
    info!("To support continued development please consider sponsoring: https://github.com/sponsors/emadda");


    // let x2 = x.clone();
    // let f: Option<Box<dyn Fn(Option<&mut UniCon>, i64)>> = Some(Box::new(move |a, b| {
    //     warn!("Callback ran, license is {:?}", &x2.license);
    // }));

    // - Issue: cannot have cyclic Cargo crates, so cannot pass Config down.
    x.cmd.run().await;

    Ok(())
}
