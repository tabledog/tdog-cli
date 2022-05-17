use std::{env, time};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rusqlite::{backup, Connection, params};
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
use uuid::Uuid;

use crate::*;
use crate::providers::stripe::schema::Db;
use crate::providers::stripe::watch::{once, poll};
use crate::util::*;
use stripe_client::http::http::{StripeClient, Config};

//use unicon::dt3::DT3;


#[allow(dead_code)]

// @see
// https://developer.github.com/v4/explorer/
// - Useful to test queries.


// https://github.com/graphql-rust/graphql-client
// - Use instead of Github Rust lib?

// @todo/low Look into `anyhow` error handling.

// See `Repository` in gql schema, start there? contains a lot of fields.
impl Download {
    pub async fn watch(&self) {
        let mut uc = UniCon::new(&UniConCreate {
            engine: self.to.clone()
        });
        let mut sc = self.get_stripe_client();
        sc.get_account_set_cache().await;
        let poll_freq_ms: u64 = self.options.poll_freq_ms.unwrap_or(400).into();

        let watch = match &self.from {
            FromAPI::Stripe(s) => {
                poll(&sc, &mut uc, poll_freq_ms, &self).await;
            }
        };
    }
    pub async fn download_all(&self) {
        let mut uc = UniCon::new(&UniConCreate {
            engine: self.to.clone()
        });
        let mut sc = self.get_stripe_client();
        sc.get_account_set_cache().await;


        match &self.from {
            FromAPI::Stripe(s) => {
                once(&sc, &mut uc, &self).await;
            }
        };
    }

    pub fn get_stripe_client(&self) -> StripeClient {
        info!("This CLI uses Stripe API version {}.", StripeClient::get_api_version());
        info!("Version default for account: https://dashboard.stripe.com/developers");
        info!("All valid versions: https://stripe.com/docs/upgrades#api-changelog");
        if let FromAPI::Stripe(x) = &self.from {
            let mut c: Config = x.into();
            return StripeClient::new(c);
        }
        unreachable!("Called get_stripe_client when `from` is not a Stripe config.");
    }

    pub fn get_stripe_from(&self) -> &Stripe {
        match &self.from {
            FromAPI::Stripe(s) => return s,
            _ => panic!("From does not equal Stripe.")
        }
    }
}


pub(crate) fn get_temp_file(f: String) -> std::io::Result<String> {
    let mut dir = env::temp_dir();
    dir.push("td-data");
    fs::create_dir_all(dir.clone().into_os_string().into_string().unwrap().as_str())?;
    dir.push(f);
    let d2 = dir.clone();

    // Assert: is writable.
    let f = File::create(dir)?;
    f.sync_all()?;

    Ok(d2.canonicalize()?.into_os_string().into_string().unwrap())
}

pub fn get_unique_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}

/// UTC `2021-01-24 19:06:26.256`
pub fn now_3() -> DT3 {
    Utc::now().naive_utc().into()
}

/// UTC `2021-01-24 19:06:26.256`
pub fn iso_now() -> String {
    let dt: DateTime<Utc> = Utc::now();

    // println!("{}", dt.format("%+"));
    // 2021-01-24T19:06:26.256932+00:00

    // println!("{}", dt.format("%Y-%m-%d %H:%M:%f"));
    // 2021-01-24 19:06:256932000

    // println!("{}", dt.format("%Y-%m-%d %H:%M:%S.%3f"));
    // 2021-01-24 19:06:26.256
    // Note: identical to SQLites STRFTIME("%Y-%m-%d %H:%M:%f")

    dt.format(get_date_fmt_3ms()).to_string()
}


pub fn get_date_fmt_3ms() -> &'static str {
    "%Y-%m-%d %H:%M:%S.%3f"
}

/// @see https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=60fbec79b2f6dba148c079e4f1929644
/// @see https://stackoverflow.com/questions/50312999/how-do-i-go-from-a-naivedate-to-a-specific-timezone-with-chrono
pub fn get_utc_dt_from_3ms(s: &String) -> DateTime<Utc> {
    let dt = NaiveDateTime::parse_from_str(s, get_date_fmt_3ms()).unwrap();
    Utc.from_utc_datetime(&dt)
}


/// UTC `2021-01-24 19:06:26`
/// - Default used by the Stripe API.
pub fn get_utc_dt(s: &String) -> DateTime<Utc> {
    let dt = NaiveDateTime::parse_from_str(s, &"%Y-%m-%d %H:%M:%S").unwrap();
    Utc.from_utc_datetime(&dt)
}



