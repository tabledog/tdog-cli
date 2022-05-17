
#![allow(warnings)]
#![allow(dead_code)]

use log::{info, trace, warn};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::sync::Arc;
use std::sync::Mutex;

use async_stream::try_stream;
use futures_core::stream::Stream;

use reqwest::{Client, RequestBuilder, Response, StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::time::Duration;

use crate::types::req_params::*;
use crate::types::responses::*;
use crate::types::types::*;

use super::trait_param_meta::*;

use chrono::{DateTime, Utc};

use futures_util::task::Spawn;
use std::cmp;
use std::future::Future;
use std::ops::Sub;
use tokio::sync::{RwLock, RwLockWriteGuard};
use tokio::time::delay_for;

#[derive(Clone, Debug)]
pub struct StripeClient {
    pub config: Config,
    pub client: Client,
    pub stripe_account: Option<StripeAccount>,
    // Arc Mutex so that this client can be cloned but a single Stripe account for a process logs stats in a central place.
    // - Also allows external observation of current 429 status (different thread or async tasks).
    pub stats: Arc<RwLock<Stats>>,
}

// Loosely means "Stripe dataset".
#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct StripeAccount {
    pub id: String,
    pub is_test: bool,
    pub account: Map<String, Value>,
}

#[derive(Debug, Clone)]
pub struct Config {
    // pub publishable_key: String,
    pub secret_key: String,
    pub is_test: bool,
    pub base: String,
    pub headers: Option<HashMap<String, String>>,
    pub proxy: Option<String>,
    pub timeout_ms: Option<u32>,
    // This may eventually be an async fn pointer (async closures not currently supported).
    pub retry: bool,

    // Logs request meta data (start, end, http code, bytes) into `stats`.
    // - Logs are part of this client (instead of implemented by app code) because:
    //      - The default retry logic cannot be observed otherwise. This client can do a number of auto-retries which are invisible to the end client.
    //      - This clients public function signatures represent a single request/response, but retries are a chain.
    //              - Streams (object lists) keep pagination meta data in their closure.
    //      - The only HTTP based indicator of a Stripe account ID is the auth header, which is not normally logged.
    //          - One client = one log = one stripe account.
    //              - 429 response/read rate can be determined grouped by Stripe account.
    pub log_requests: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    // Total 429 responses encountered for the lifetime of this client (and its clones for the same Stripe account).
    pub total_429_responses: u32,
    // Total requests that have seen one or more 429's in a row, and are retrying to hit a non-429 response.
    // - Loosely indicates "is the remote Stripe account currently locked due to rate limiting".
    pub cur_429_reqs_retrying: u32,

    // Number of HTTP requests that have started but not yet returned.
    pub running: u32,

    // Enables:
    // - Calculating aggregate stats to determine rate to guess at an ETA for whole account download.
    // - Outputting summary log lines to the end user at short intervals (e.g. every 15 seconds).
    //
    // Note: long running processes should implement "log rotation"/Garbage Collection by locking, retrieving and then clearing this.
    pub req_log: Vec<ReqLog>,
}

#[derive(Debug, Clone)]
pub struct ReqLog {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub duration_ms: u32,
    // pub url: String,
    // pub method: String,
    pub bytes_rx: u64,
    pub code: Option<u16>,
    pub net_error: bool,
}

// trait SendAndLog {
//     fn send_and_log() -> impl Future<Output = Result<Response, reqwest::Error>>
// }

// Issue: `reqwest::RequestBuilder`:
// - Is not `Clone`, and takes ownership on `builder.send()`. This means it cannot re-create another identical future/request.
// - Does not allow reading back the data you give it.
//      - `req.build()` can be used to get a `Request` which allows data access, but this is also !Clone and takes ownership which means you cannot read the data after you send the request (e.g. in a "request complete log the metadata" handler).
//
// Fix: A data-ony representation of the request, that can generate a `RequestBuilder` and other subset meta data structs (like RequestLog, which just needs a subset of the data).
// - This will also enable using different HTTP API's in the future, and possibly enable exernal queueing via Serde.
pub struct ReqData {
    req: Req,
    // May add meta data around a chain of retries in the future.
}

pub struct Req {
    url: String,
    method: String,
    headers: HashMap<String, String>,
    query: Vec<(String, String)>,
}
// to_builder(&self) -> RequestBuilder
// to_log(&self, start, end) -> ReqLog

impl Config {
    pub fn is_test_secret_key(k: &str) -> bool {
        // sk_test or rk_test
        k.contains("_test")
    }
}

trait SetHeadersMap {
    fn set_headers_map(self, h: &HashMap<String, String>) -> Self;
}

impl SetHeadersMap for RequestBuilder {
    /// Issue: RequestBuilder does not have a &mut x version so takes ownership on every header set.
    /// Fix: Use this fn to wrap the "reset var on each iteration" pattern.
    /// Note: Config headers may eventually come from a config at runtime so this needs to be dynamic.
    fn set_headers_map(self, h: &HashMap<String, String>) -> Self {
        let mut s = self;
        for (k, v) in h {
            s = s.header(k.as_str(), v.as_str());
        }
        s
    }
}

impl StripeClient {
    // @todo/low handle errors.
    pub fn new(config: Config) -> Self {
        let mut b = reqwest::Client::builder()
            // Note: The Stripe API seems to always use HTTP 1.1 with no compression.
            .gzip(true)
            .brotli(true)
            .trust_dns(true);

        // https://docs.rs/reqwest/0.11.6/reqwest/struct.ClientBuilder.html#method.tcp_keepalive
        // tcp_nodelay
        // tcp_time
        // .default_headers(headers)

        // `trust_dns`
        // - Pure Rust version of DNS (instead of the default which is to use the system DNS via system call).
        // - Has a DNS cache (default DNS resolover does not cache. Curl does cache).
        // - @see https://github.com/seanmonstar/reqwest/issues/296
        //
        // Fixes error:
        // - `Err(reqwest::Error { kind: Request, url: "https://api.stripe.com/v1/payment_methods?customer=cus_KOcuQQVytRa2zB&limit=100&type=sepa_debit", source: hyper::Error(Connect, ConnectError("dns error", Custom { kind: Uncategorized, error: "failed to lookup address information: nodename nor servname provided, or not known" })) })`
        // - This occurs when starting 100+ requests to the same domain (Mac OS M1, debug build).

        // Note: System proxy from env vars are used automatically, disable with b.no_proxy()
        if let Some(p) = &config.proxy {
            b = b.proxy(reqwest::Proxy::all(p.as_str()).unwrap());
        }

        if let Some(x) = config.timeout_ms {
            b = b.timeout(Duration::from_millis(x.into()));
        }

        let client = b.build().unwrap();

        Self {
            client,
            config,
            stripe_account: None,
            stats: Arc::new(RwLock::new(Stats {
                total_429_responses: 0,
                cur_429_reqs_retrying: 0,
                running: 0,
                req_log: vec![],
            })),
        }
    }

    pub fn get_api_version() -> &'static str {
        "2020-08-27"
    }

    // Gets the account for Stripe secret key being used.
    // - This does not seem to be documented in the Open API spec or in the web docs.
    //      - No concrete type returned.
    //
    // @see https://stackoverflow.com/a/40575022/4949386
    pub async fn get_account(
        &self,
    ) -> Result<(String, serde_json::Map<String, serde_json::Value>), UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder =
                self.client.get(&format!("{}/v1/account", self.config.base));

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<serde_json::Value>().await?;

                match x {
                    Value::Object(o) => match o.get("id").unwrap() {
                        Value::String(id) => {
                            return Ok((id.clone(), o));
                        }
                        _ => panic!("No id key in account JSON"),
                    },
                    _ => {
                        panic!("Stripe client, unknown JSON variant returned from account endpoint")
                    }
                }
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    // Intended to be called in an async context after creating the client.
    // - Allow functions to take a read-only ref to a Client, and client.stripe_account.unwrap().
    // - Assumptions:
    //      - Create a new client for each new Stripe secret key (do not mutate the secret key as it will not longer match this account).
    //      - Account will only need to be read once per process start up as it changes slowly/just the ID is needed.
    pub async fn get_account_set_cache(&mut self) {
        let (id, account) = self.get_account().await.unwrap();

        self.stripe_account = Some(StripeAccount {
            id,
            // Allow passing/cloning this as a single object (instead of having to pass the entire client around when only state about the account is required).
            is_test: self.config.is_test,
            account,
        });
    }

    fn set_headers(
        &self,
        r: RequestBuilder,
        this_req: &Option<HashMap<String, String>>,
    ) -> RequestBuilder {
        let mut r = r
                    .header("Accept", "application/json")
                    .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.102 Safari/537.36")
                    // Stripe version locks JSON format/paths.
                    // - /events always uses the Stripe account default version at the time of the event publish (it is like a webhook with a cache of 30 days).
                    // - Webhooks can be locked to a version via the API.
                    .header("Stripe-Version", Self::get_api_version());

        if let Some(h) = this_req {
            r = r.set_headers_map(h);
        }

        if let Some(h) = &self.config.headers {
            r = r.set_headers_map(h);
        }

        r
    }

    // Requests that have seen one or more 429 responses, and are waiting to retry.
    // - Requests are removed from this count when a non-429 response it received (possibly another error).
    pub async fn cur_reqs_waiting_for_429_resolve(&self) -> u32 {
        let mut stats = self.stats.write().await;
        stats.cur_429_reqs_retrying
    }

    // Issue: A connection that is kept alive seems to eventually fail (reqwest error: IncompleteMessage).
    // Fix: Try request again immediately after, which should create a new connection.
    //      - Assumption: HTTP requests that represent reads are safe to retry as they are not mutating state.
    //
    // - This is important for long running downloads that are inserted inside of a DB transaction.
    //      - A single failed connection should not fail the entire download operation.
    pub async fn retry<F: FnOnce() -> RequestBuilder + Copy>(
        &self,
        get_req: F,
    ) -> Result<Response, reqwest::Error> {
        // Note: `req.send()` returns `OK(Response)` for non-20x codes like `429`.

        // let mut a = get_req().send().await;
        let mut a = self.send_and_log(get_req()).await;

        // User may have their own retry logic.
        if a.is_200() || !self.config.retry {
            return a;
        }

        let mut net_err_count = 0;
        let mut non_429_err_count = 0;
        let mut rate_429_err_count = 0;

        // Assert: `cur_429_reqs_retrying` counts this loop only once.
        // - Mutex needed as two clousres take mut references (Mutex = dynamic runtime &mut ref).
        let mut this_429_counted_mt = Mutex::new(false);

        // Only for false => true transition.
        let mut seen_429 = |mut stats: RwLockWriteGuard<'_, Stats>| {
            let mut this_429_counted = this_429_counted_mt.lock().unwrap();

            stats.total_429_responses += 1;

            if !*this_429_counted {
                stats.cur_429_reqs_retrying += 1;
                *this_429_counted = true;
            }
        };

        // Only for true => false transition.
        let mut resolved_429 = |mut stats: RwLockWriteGuard<'_, Stats>| {
            let mut this_429_counted = this_429_counted_mt.lock().unwrap();

            if *this_429_counted {
                stats.cur_429_reqs_retrying -= 1;
                *this_429_counted = false;
            }
        };

        // Retry loop:
        // States:
        // - Network error: Retry immediately, then every 10 seconds up to 6 times.
        // - 429 error: Wait a random 1 - 30 seconds, retry up to 20 times.
        //      - Assumes external queue will observe 429 state and pause starting any more requests/futures.
        // - Non-429 HTTP error: Wait 2 seconds, retry up to 2 times.
        //      - Assumes read-only requests.
        //
        // Note:
        // - All will kill the process and assume no resolution after max retries.
        // - Any combination of the above can occur in on request retry loop.
        // - @todo/low Only HTTP 200 is a success, other non-200 success codes not handled (E.g. a write would be 201, a cached response 304).
        loop {
            let status = a.get_status();
            warn!("HTTP request failed: {:?}", &a);
            warn!("Retrying.");

            match status {
                None => {
                    // Assumption: Network error (not HTTP)?
                    net_err_count += 1;

                    // On first network error, retry immediately (server can randomly drop pooled connections).
                    // On >first network error, delay in case of temporary client internet connectivity issues.
                    if net_err_count > 1 {
                        delay_for(Duration::from_millis(1000 * 10)).await;
                    }

                    if net_err_count == 6 {
                        panic!("Network issues. Retried request {} times.", net_err_count);
                    }

                    // a = get_req().send().await;
                    a = self.send_and_log(get_req()).await;
                    if a.is_200() {
                        resolved_429(self.stats.write().await);
                        return a;
                    }
                }
                Some(x) => {
                    match x {
                        StatusCode::TOO_MANY_REQUESTS => {
                            seen_429(self.stats.write().await);

                            if rate_429_err_count > 20 {
                                panic!(
                                    "Retried 429 HTTP error {} times, assuming no resolution.",
                                    rate_429_err_count
                                );
                            }

                            // Assumption: There can be 2-100 requests being awaited to the same central Stripe account (rate limits applied per Stripe account).
                            // - Assumption: If one 429 is received, all requests will 429 too.
                            // - If there is a queue scheduling requests, it can observe the 429 status from `client.stats` and will pause until 429's have resolved (Stripe account unlocked).
                            // - The queue has a rate limit set, which **starts** X requests per second (no limit on total actively running; duration per queue item does not matter and can be multiple minutes).
                            //      - A request may take 3 seconds on average, so a 429 may be received after 3 seconds, with (3 seconds * X) requests currently in progress that will all return 429 too after the first one does.
                            //      - These 429-retrying-requests will be distributed over **at least** 30 seconds (even more so if 429 lock persists many minutes), which is 10x the time period, so a 10x reduction in request speed.
                            //          - If the user has set a rate limit that is too high for their Stripe account, the process will move between these two states: ("rate too fast due to config", "10x slower due to 429")
                            //              - It is up to the user to set the correct rate limit, and understand their systems tolerance to rate locking their Stripe account.

                            // let mut rng = rand::random() // Issue: Not Send or Sync
                            let mut rng = StdRng::seed_from_u64(1);
                            delay_for(Duration::from_millis(rng.gen_range(1000..30_000))).await;

                            // a = get_req().send().await;
                            a = self.send_and_log(get_req()).await;
                            if a.is_200() {
                                resolved_429(self.stats.write().await);
                                return a;
                            }
                        }
                        _ => {
                            non_429_err_count += 1;

                            if non_429_err_count > 2 {
                                panic!(
                                    "Retried non-429 HTTP error {} times, assuming no resolution.",
                                    non_429_err_count
                                );
                            }

                            delay_for(Duration::from_millis(2000)).await;
                            // a = get_req().send().await;
                            a = self.send_and_log(get_req()).await;
                            if a.is_200() {
                                resolved_429(self.stats.write().await);
                                return a;
                            }
                        }
                    }
                }
            }
        }
    }

    // - Logs the start, end, duration and code of every request **including retries** that are invisible to the calling code.
    // - Counts the number of currently running (started, not yet returned) HTTP requests.
    pub async fn send_and_log(&self, req: RequestBuilder) -> Result<Response, reqwest::Error> {
        let start: DateTime<Utc> = Utc::now();

        {
            let mut stats = self.stats.write().await;
            stats.running += 1;
        }

        let res = req.send().await;

        let mut stats = self.stats.write().await;
        stats.running -= 1;

        if !self.config.log_requests {
            return res;
        }

        let mut code = None;
        let mut net_error = false;
        let mut bytes_rx = 0;

        match &res {
            Ok(x) => {
                code = x.status().as_u16().into();
                bytes_rx = x.content_length().unwrap();
            }
            Err(x) => match x.status() {
                None => {
                    net_error = true;
                }
                Some(x2) => {
                    code = x2.as_u16().into();
                }
            },
        }

        let end: DateTime<Utc> = Utc::now();
        let x = ReqLog {
            start,
            end,
            duration_ms: cmp::max(0, end.sub(start).num_milliseconds()) as u32,
            bytes_rx,
            code,
            net_error,
        };

        // Ordered request end ASC.
        stats.req_log.push(x);

        return res;
    }
}

// @todo impl std::error::Error
// @see https://www.reddit.com/r/rust/comments/gj8inf/rust_structuring_and_handling_errors_in_2020/fqlmknt/
#[derive(Debug)]
pub enum UniErr {
    Net(reqwest::Error),
    App(Error),
}

// Implement Error trait so it can be used with Rusts error handling.
impl std::error::Error for UniErr {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            UniErr::Net(ref netErr) => Some(netErr),
            UniErr::App(_) => None,
        }
    }
}

impl std::fmt::Display for UniErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            UniErr::Net(ref netErr) => netErr.fmt(f),
            UniErr::App(_) => write!(f, "Error object returned from server"),
        }
    }
}

impl From<Error> for UniErr {
    fn from(e: Error) -> Self {
        UniErr::App(e)
    }
}

impl From<reqwest::Error> for UniErr {
    fn from(e: reqwest::Error) -> Self {
        UniErr::Net(e)
    }
}

// Reqwest returns Ok(Res) for non-200 codes like 429, which is confusing.
// - This trait adds util functions to get the status regardless of if the Result is Ok or Err.
//      - Should make logic more direct instead of multiple branches of match to get a boolean.
trait ResultStatus {
    fn is_200(&self) -> bool;
    fn get_status(&self) -> Option<StatusCode>;
}

impl ResultStatus for Result<Response, reqwest::Error> {
    fn is_200(&self) -> bool {
        match &self {
            Ok(res) => res.status() == StatusCode::OK,
            _ => false,
        }
    }

    fn get_status(&self) -> Option<StatusCode> {
        match &self {
            Ok(res) => res.status().into(),
            Err(e) => e.status(),
        }
    }
}

impl StripeClient {
    pub async fn v1_3d_secure_x_get(
        &self,
        three_d_secure: String,
        params: &Option<GetAccount>,
    ) -> Result<ThreeDSecure, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/3d_secure/{}",
                    self.config.base, three_d_secure
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ThreeDSecure>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_account_get(&self, params: &Option<GetAccount>) -> Result<Account, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/account", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Account>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_account_bank_accounts_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<UniPolymorphic70BAFA, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/account/bank_accounts/{}",
                    self.config.base, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<UniPolymorphic70BAFA>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_account_capabilities_get(
        &self,
        params: &Option<GetAccount>,
    ) -> Result<ListAccountCapability, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/account/capabilities", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ListAccountCapability>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_account_capabilities_x_get(
        &self,
        capability: String,
        params: &Option<GetAccount>,
    ) -> Result<AccountCapability, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/account/capabilities/{}",
                    self.config.base, capability
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<AccountCapability>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_account_external_accounts_get(
        &self,
        params: &Option<GetChargesChargeRefunds>,
    ) -> Result<ExternalAccountListADE54B, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/account/external_accounts",
                    self.config.base
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ExternalAccountListADE54B>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_account_external_accounts_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<UniPolymorphic70BAFA, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/account/external_accounts/{}",
                    self.config.base, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<UniPolymorphic70BAFA>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_account_people_get(
        &self,
        params: &Option<GetAccountPeople>,
    ) -> Result<GetAccountPeopleRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/account/people", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetAccountPeopleRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_account_people_get_st(
        &self,
        params: &GetAccountPeople,
    ) -> impl Stream<Item = Result<GetAccountPeopleRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_account_people_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_account_people_x_get(
        &self,
        person: String,
        params: &Option<GetAccount>,
    ) -> Result<Person, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/account/people/{}",
                    self.config.base, person
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Person>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_account_persons_get(
        &self,
        params: &Option<GetAccountPeople>,
    ) -> Result<GetAccountPeopleRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/account/persons", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetAccountPeopleRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_account_persons_get_st(
        &self,
        params: &GetAccountPeople,
    ) -> impl Stream<Item = Result<GetAccountPeopleRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_account_persons_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_account_persons_x_get(
        &self,
        person: String,
        params: &Option<GetAccount>,
    ) -> Result<Person, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/account/persons/{}",
                    self.config.base, person
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Person>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_accounts_get(
        &self,
        params: &Option<GetAccounts>,
    ) -> Result<GetAccountsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/accounts", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetAccountsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_accounts_get_st(
        &self,
        params: &GetAccounts,
    ) -> impl Stream<Item = Result<GetAccountsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_accounts_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_accounts_x_get(
        &self,
        account: String,
        params: &Option<GetAccount>,
    ) -> Result<Account, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/accounts/{}", self.config.base, account))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Account>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_accounts_x_bank_accounts_x_get(
        &self,
        account: String,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<UniPolymorphic70BAFA, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/accounts/{}/bank_accounts/{}",
                    self.config.base, account, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<UniPolymorphic70BAFA>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_accounts_x_capabilities_get(
        &self,
        account: String,
        params: &Option<GetAccount>,
    ) -> Result<ListAccountCapability, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/accounts/{}/capabilities",
                    self.config.base, account
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ListAccountCapability>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_accounts_x_capabilities_x_get(
        &self,
        account: String,
        capability: String,
        params: &Option<GetAccount>,
    ) -> Result<AccountCapability, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/accounts/{}/capabilities/{}",
                    self.config.base, account, capability
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<AccountCapability>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_accounts_x_external_accounts_get(
        &self,
        account: String,
        params: &Option<GetChargesChargeRefunds>,
    ) -> Result<ExternalAccountListADE54B, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/accounts/{}/external_accounts",
                    self.config.base, account
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ExternalAccountListADE54B>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_accounts_x_external_accounts_x_get(
        &self,
        account: String,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<UniPolymorphic70BAFA, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/accounts/{}/external_accounts/{}",
                    self.config.base, account, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<UniPolymorphic70BAFA>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_accounts_x_people_get(
        &self,
        account: String,
        params: &Option<GetAccountPeople>,
    ) -> Result<GetAccountPeopleRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/accounts/{}/people",
                    self.config.base, account
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetAccountPeopleRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_accounts_x_people_get_st(
        &self,
        account: String,
        params: &GetAccountPeople,
    ) -> impl Stream<Item = Result<GetAccountPeopleRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_accounts_x_people_get(account.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_accounts_x_people_x_get(
        &self,
        account: String,
        person: String,
        params: &Option<GetAccount>,
    ) -> Result<Person, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/accounts/{}/people/{}",
                    self.config.base, account, person
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Person>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_accounts_x_persons_get(
        &self,
        account: String,
        params: &Option<GetAccountPeople>,
    ) -> Result<GetAccountPeopleRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/accounts/{}/persons",
                    self.config.base, account
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetAccountPeopleRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_accounts_x_persons_get_st(
        &self,
        account: String,
        params: &GetAccountPeople,
    ) -> impl Stream<Item = Result<GetAccountPeopleRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_accounts_x_persons_get(account.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_accounts_x_persons_x_get(
        &self,
        account: String,
        person: String,
        params: &Option<GetAccount>,
    ) -> Result<Person, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/accounts/{}/persons/{}",
                    self.config.base, account, person
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Person>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_apple_pay_domains_get(
        &self,
        params: &Option<GetApplePayDomains>,
    ) -> Result<ApplePayDomainList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/apple_pay/domains", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ApplePayDomainList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_apple_pay_domains_get_st(
        &self,
        params: &GetApplePayDomains,
    ) -> impl Stream<Item = Result<ApplePayDomainList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_apple_pay_domains_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_apple_pay_domains_x_get(
        &self,
        domain: String,
        params: &Option<GetAccount>,
    ) -> Result<ApplePayDomain, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/apple_pay/domains/{}",
                    self.config.base, domain
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ApplePayDomain>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_application_fees_get(
        &self,
        params: &Option<GetApplicationFees>,
    ) -> Result<GetApplicationFeesRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/application_fees", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetApplicationFeesRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_application_fees_get_st(
        &self,
        params: &GetApplicationFees,
    ) -> impl Stream<Item = Result<GetApplicationFeesRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_application_fees_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_application_fees_x_refunds_x_get(
        &self,
        fee: String,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<FeeRefund, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/application_fees/{}/refunds/{}",
                    self.config.base, fee, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<FeeRefund>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_application_fees_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<PlatformFee, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/application_fees/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PlatformFee>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_application_fees_x_refunds_get(
        &self,
        id: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<FeeRefundListFDC0D1, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/application_fees/{}/refunds",
                    self.config.base, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<FeeRefundListFDC0D1>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_application_fees_x_refunds_get_st(
        &self,
        id: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<FeeRefundListFDC0D1, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_application_fees_x_refunds_get(id.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_balance_get(&self, params: &Option<GetAccount>) -> Result<Balance, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/balance", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Balance>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_balance_history_get(
        &self,
        params: &Option<GetBalanceHistory>,
    ) -> Result<BalanceTransactionsList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/balance/history", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<BalanceTransactionsList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_balance_history_get_st(
        &self,
        params: &GetBalanceHistory,
    ) -> impl Stream<Item = Result<BalanceTransactionsList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_balance_history_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_balance_history_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<BalanceTransaction, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/balance/history/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<BalanceTransaction>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_balance_transactions_get(
        &self,
        params: &Option<GetBalanceHistory>,
    ) -> Result<BalanceTransactionsList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/balance_transactions", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<BalanceTransactionsList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_balance_transactions_get_st(
        &self,
        params: &GetBalanceHistory,
    ) -> impl Stream<Item = Result<BalanceTransactionsList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_balance_transactions_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_balance_transactions_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<BalanceTransaction, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/balance_transactions/{}",
                    self.config.base, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<BalanceTransaction>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_bitcoin_receivers_get(
        &self,
        params: &Option<GetBitcoinReceivers>,
    ) -> Result<GetBitcoinReceiversRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/bitcoin/receivers", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetBitcoinReceiversRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_bitcoin_receivers_get_st(
        &self,
        params: &GetBitcoinReceivers,
    ) -> impl Stream<Item = Result<GetBitcoinReceiversRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_bitcoin_receivers_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_bitcoin_receivers_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<BitcoinReceiver, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/bitcoin/receivers/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<BitcoinReceiver>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_bitcoin_receivers_x_transactions_get(
        &self,
        receiver: String,
        params: &Option<GetBitcoinReceiversReceiverTransactions>,
    ) -> Result<BitcoinTransactionListC3C538, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/bitcoin/receivers/{}/transactions",
                    self.config.base, receiver
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<BitcoinTransactionListC3C538>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_bitcoin_receivers_x_transactions_get_st(
        &self,
        receiver: String,
        params: &GetBitcoinReceiversReceiverTransactions,
    ) -> impl Stream<Item = Result<BitcoinTransactionListC3C538, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_bitcoin_receivers_x_transactions_get(receiver.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_bitcoin_transactions_get(
        &self,
        params: &Option<GetBitcoinTransactions>,
    ) -> Result<BitcoinTransactionListC3C538, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/bitcoin/transactions", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<BitcoinTransactionListC3C538>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_bitcoin_transactions_get_st(
        &self,
        params: &GetBitcoinTransactions,
    ) -> impl Stream<Item = Result<BitcoinTransactionListC3C538, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_bitcoin_transactions_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_charges_get(
        &self,
        params: &Option<GetCharges>,
    ) -> Result<GetChargesRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/charges", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetChargesRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_charges_get_st(
        &self,
        params: &GetCharges,
    ) -> impl Stream<Item = Result<GetChargesRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_charges_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_charges_x_get(
        &self,
        charge: String,
        params: &Option<GetAccount>,
    ) -> Result<Charge, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/charges/{}", self.config.base, charge))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Charge>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_charges_x_dispute_get(
        &self,
        charge: String,
        params: &Option<GetAccount>,
    ) -> Result<Dispute, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/charges/{}/dispute",
                    self.config.base, charge
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Dispute>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_charges_x_refunds_get(
        &self,
        charge: String,
        params: &Option<GetChargesChargeRefunds>,
    ) -> Result<RefundListBBCF51, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/charges/{}/refunds",
                    self.config.base, charge
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<RefundListBBCF51>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_charges_x_refunds_get_st(
        &self,
        charge: String,
        params: &GetChargesChargeRefunds,
    ) -> impl Stream<Item = Result<RefundListBBCF51, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_charges_x_refunds_get(charge.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_charges_x_refunds_x_get(
        &self,
        charge: String,
        refund: String,
        params: &Option<GetAccount>,
    ) -> Result<Refund, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/charges/{}/refunds/{}",
                    self.config.base, charge, refund
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Refund>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_checkout_sessions_get(
        &self,
        params: &Option<GetCheckoutSessions>,
    ) -> Result<PaymentPagesCheckoutSessionList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/checkout/sessions", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PaymentPagesCheckoutSessionList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_checkout_sessions_get_st(
        &self,
        params: &GetCheckoutSessions,
    ) -> impl Stream<Item = Result<PaymentPagesCheckoutSessionList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_checkout_sessions_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_checkout_sessions_x_get(
        &self,
        session: String,
        params: &Option<GetAccount>,
    ) -> Result<Session, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/checkout/sessions/{}",
                    self.config.base, session
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Session>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_checkout_sessions_x_line_items_get(
        &self,
        session: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<PaymentPagesCheckoutSessionListLineItems404A63, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/checkout/sessions/{}/line_items",
                    self.config.base, session
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res
                    .json::<PaymentPagesCheckoutSessionListLineItems404A63>()
                    .await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_checkout_sessions_x_line_items_get_st(
        &self,
        session: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<PaymentPagesCheckoutSessionListLineItems404A63, UniErr>> + '_
    {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_checkout_sessions_x_line_items_get(session.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_country_specs_get(
        &self,
        params: &Option<GetCountrySpecs>,
    ) -> Result<GetCountrySpecsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/country_specs", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetCountrySpecsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_country_specs_get_st(
        &self,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<GetCountrySpecsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_country_specs_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_country_specs_x_get(
        &self,
        country: String,
        params: &Option<GetAccount>,
    ) -> Result<CountrySpec, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/country_specs/{}",
                    self.config.base, country
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<CountrySpec>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_coupons_get(
        &self,
        params: &Option<GetCoupons>,
    ) -> Result<GetCouponsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/coupons", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetCouponsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_coupons_get_st(
        &self,
        params: &GetCoupons,
    ) -> impl Stream<Item = Result<GetCouponsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_coupons_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_coupons_x_get(
        &self,
        coupon: String,
        params: &Option<GetAccount>,
    ) -> Result<Coupon, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/coupons/{}", self.config.base, coupon))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Coupon>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_credit_notes_get(
        &self,
        params: &Option<GetCreditNotes>,
    ) -> Result<CreditNotesList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/credit_notes", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<CreditNotesList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_credit_notes_get_st(
        &self,
        params: &GetCreditNotes,
    ) -> impl Stream<Item = Result<CreditNotesList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_credit_notes_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_credit_notes_preview_get(
        &self,
        params: &GetCreditNotesPreview,
    ) -> Result<CreditNote, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/credit_notes/preview", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<CreditNote>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_credit_notes_preview_lines_get(
        &self,
        params: &GetCreditNotesPreviewLines,
    ) -> Result<CreditNoteLinesList34EE1C, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/credit_notes/preview/lines",
                    self.config.base
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<CreditNoteLinesList34EE1C>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_credit_notes_preview_lines_get_st(
        &self,
        params: &GetCreditNotesPreviewLines,
    ) -> impl Stream<Item = Result<CreditNoteLinesList34EE1C, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_credit_notes_preview_lines_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_credit_notes_x_lines_get(
        &self,
        credit_note: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<CreditNoteLinesList34EE1C, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/credit_notes/{}/lines",
                    self.config.base, credit_note
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<CreditNoteLinesList34EE1C>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_credit_notes_x_lines_get_st(
        &self,
        credit_note: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<CreditNoteLinesList34EE1C, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_credit_notes_x_lines_get(credit_note.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_credit_notes_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<CreditNote, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/credit_notes/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<CreditNote>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_get(
        &self,
        params: &Option<GetCustomers>,
    ) -> Result<GetCustomersRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/customers", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetCustomersRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_customers_get_st(
        &self,
        params: &GetCustomers,
    ) -> impl Stream<Item = Result<GetCustomersRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_customers_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_customers_x_get(
        &self,
        customer: String,
        params: &Option<GetAccount>,
    ) -> Result<UniGetCustomersCustomerRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/customers/{}", self.config.base, customer))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<UniGetCustomersCustomerRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_x_balance_transactions_get(
        &self,
        customer: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<CustomerBalanceTransactionList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/balance_transactions",
                    self.config.base, customer
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<CustomerBalanceTransactionList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_customers_x_balance_transactions_get_st(
        &self,
        customer: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<CustomerBalanceTransactionList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_customers_x_balance_transactions_get(customer.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_customers_x_balance_transactions_x_get(
        &self,
        customer: String,
        transaction: String,
        params: &Option<GetAccount>,
    ) -> Result<CustomerBalanceTransaction, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/balance_transactions/{}",
                    self.config.base, customer, transaction
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<CustomerBalanceTransaction>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_x_bank_accounts_get(
        &self,
        customer: String,
        params: &Option<GetChargesChargeRefunds>,
    ) -> Result<BankAccountList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/bank_accounts",
                    self.config.base, customer
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<BankAccountList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_customers_x_bank_accounts_get_st(
        &self,
        customer: String,
        params: &GetChargesChargeRefunds,
    ) -> impl Stream<Item = Result<BankAccountList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_customers_x_bank_accounts_get(customer.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_customers_x_bank_accounts_x_get(
        &self,
        customer: String,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<BankAccount, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/bank_accounts/{}",
                    self.config.base, customer, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<BankAccount>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_x_cards_get(
        &self,
        customer: String,
        params: &Option<GetChargesChargeRefunds>,
    ) -> Result<CardList81180B, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/cards",
                    self.config.base, customer
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<CardList81180B>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_customers_x_cards_get_st(
        &self,
        customer: String,
        params: &GetChargesChargeRefunds,
    ) -> impl Stream<Item = Result<CardList81180B, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_customers_x_cards_get(customer.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_customers_x_cards_x_get(
        &self,
        customer: String,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<Card, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/cards/{}",
                    self.config.base, customer, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Card>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_x_discount_get(
        &self,
        customer: String,
        params: &Option<GetAccount>,
    ) -> Result<Discount, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/discount",
                    self.config.base, customer
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Discount>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_x_sources_get(
        &self,
        customer: String,
        params: &Option<GetCustomersCustomerSources>,
    ) -> Result<ApmsSourcesSourceListF0771E, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/sources",
                    self.config.base, customer
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ApmsSourcesSourceListF0771E>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_x_sources_x_get(
        &self,
        customer: String,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<UniPolymorphic, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/sources/{}",
                    self.config.base, customer, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<UniPolymorphic>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_x_subscriptions_get(
        &self,
        customer: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<SubscriptionList5B5899, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/subscriptions",
                    self.config.base, customer
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<SubscriptionList5B5899>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_customers_x_subscriptions_get_st(
        &self,
        customer: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<SubscriptionList5B5899, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_customers_x_subscriptions_get(customer.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_customers_x_subscriptions_x_get(
        &self,
        customer: String,
        subscription_exposed_id: String,
        params: &Option<GetAccount>,
    ) -> Result<Subscription, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/subscriptions/{}",
                    self.config.base, customer, subscription_exposed_id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Subscription>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_x_subscriptions_x_discount_get(
        &self,
        customer: String,
        subscription_exposed_id: String,
        params: &Option<GetAccount>,
    ) -> Result<Discount, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/subscriptions/{}/discount",
                    self.config.base, customer, subscription_exposed_id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Discount>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_customers_x_tax_ids_get(
        &self,
        customer: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<TaxIDsListAFDA6E, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/tax_ids",
                    self.config.base, customer
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TaxIDsListAFDA6E>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_customers_x_tax_ids_get_st(
        &self,
        customer: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<TaxIDsListAFDA6E, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_customers_x_tax_ids_get(customer.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_customers_x_tax_ids_x_get(
        &self,
        customer: String,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<TaxId, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/customers/{}/tax_ids/{}",
                    self.config.base, customer, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TaxId>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_disputes_get(
        &self,
        params: &Option<GetDisputes>,
    ) -> Result<GetDisputesRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/disputes", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetDisputesRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_disputes_get_st(
        &self,
        params: &GetDisputes,
    ) -> impl Stream<Item = Result<GetDisputesRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_disputes_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_disputes_x_get(
        &self,
        dispute: String,
        params: &Option<GetAccount>,
    ) -> Result<Dispute, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/disputes/{}", self.config.base, dispute))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Dispute>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_events_get(
        &self,
        params: &Option<GetEvents>,
    ) -> Result<NotificationEventList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/events", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<NotificationEventList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_events_get_st(
        &self,
        params: &GetEvents,
    ) -> impl Stream<Item = Result<NotificationEventList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_events_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_events_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<NotificationEvent, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/events/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<NotificationEvent>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_exchange_rates_get(
        &self,
        params: &Option<GetCountrySpecs>,
    ) -> Result<GetExchangeRatesRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/exchange_rates", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetExchangeRatesRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_exchange_rates_get_st(
        &self,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<GetExchangeRatesRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_exchange_rates_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_exchange_rates_x_get(
        &self,
        rate_id: String,
        params: &Option<GetAccount>,
    ) -> Result<ExchangeRate, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/exchange_rates/{}",
                    self.config.base, rate_id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ExchangeRate>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_file_links_get(
        &self,
        params: &Option<GetFileLinks>,
    ) -> Result<GetFileLinksRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/file_links", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetFileLinksRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_file_links_get_st(
        &self,
        params: &GetFileLinks,
    ) -> impl Stream<Item = Result<GetFileLinksRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_file_links_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_file_links_x_get(
        &self,
        link: String,
        params: &Option<GetAccount>,
    ) -> Result<FileLink, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/file_links/{}", self.config.base, link))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<FileLink>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_files_get(&self, params: &Option<GetFiles>) -> Result<GetFilesRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/files", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetFilesRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_files_get_st(
        &self,
        params: &GetFiles,
    ) -> impl Stream<Item = Result<GetFilesRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_files_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_files_x_get(
        &self,
        file: String,
        params: &Option<GetAccount>,
    ) -> Result<File, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/files/{}", self.config.base, file))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<File>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_invoiceitems_get(
        &self,
        params: &Option<GetInvoiceitems>,
    ) -> Result<GetInvoiceitemsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/invoiceitems", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetInvoiceitemsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_invoiceitems_get_st(
        &self,
        params: &GetInvoiceitems,
    ) -> impl Stream<Item = Result<GetInvoiceitemsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_invoiceitems_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_invoiceitems_x_get(
        &self,
        invoiceitem: String,
        params: &Option<GetAccount>,
    ) -> Result<InvoiceItem, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/invoiceitems/{}",
                    self.config.base, invoiceitem
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<InvoiceItem>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_invoices_get(
        &self,
        params: &Option<GetInvoices>,
    ) -> Result<InvoicesList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/invoices", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<InvoicesList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_invoices_get_st(
        &self,
        params: &GetInvoices,
    ) -> impl Stream<Item = Result<InvoicesList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_invoices_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_invoices_upcoming_get(
        &self,
        params: &Option<GetInvoicesUpcoming>,
    ) -> Result<Invoice, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/invoices/upcoming", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Invoice>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_invoices_upcoming_lines_get(
        &self,
        params: &Option<GetInvoicesUpcomingLines>,
    ) -> Result<InvoiceLinesList9B8534, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/invoices/upcoming/lines", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<InvoiceLinesList9B8534>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_invoices_upcoming_lines_get_st(
        &self,
        params: &GetInvoicesUpcomingLines,
    ) -> impl Stream<Item = Result<InvoiceLinesList9B8534, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_invoices_upcoming_lines_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_invoices_x_get(
        &self,
        invoice: String,
        params: &Option<GetAccount>,
    ) -> Result<Invoice, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/invoices/{}", self.config.base, invoice))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Invoice>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_invoices_x_lines_get(
        &self,
        invoice: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<InvoiceLinesList9B8534, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/invoices/{}/lines",
                    self.config.base, invoice
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<InvoiceLinesList9B8534>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_invoices_x_lines_get_st(
        &self,
        invoice: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<InvoiceLinesList9B8534, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_invoices_x_lines_get(invoice.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_issuer_fraud_records_get(
        &self,
        params: &Option<GetIssuerFraudRecords>,
    ) -> Result<RadarIssuerFraudRecordList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/issuer_fraud_records", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<RadarIssuerFraudRecordList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_issuer_fraud_records_get_st(
        &self,
        params: &GetIssuerFraudRecords,
    ) -> impl Stream<Item = Result<RadarIssuerFraudRecordList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_issuer_fraud_records_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_issuer_fraud_records_x_get(
        &self,
        issuer_fraud_record: String,
        params: &Option<GetAccount>,
    ) -> Result<IssuerFraudRecord, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/issuer_fraud_records/{}",
                    self.config.base, issuer_fraud_record
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<IssuerFraudRecord>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_issuing_authorizations_get(
        &self,
        params: &Option<GetIssuingAuthorizations>,
    ) -> Result<GetIssuingAuthorizationsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/issuing/authorizations", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetIssuingAuthorizationsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_issuing_authorizations_get_st(
        &self,
        params: &GetIssuingAuthorizations,
    ) -> impl Stream<Item = Result<GetIssuingAuthorizationsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_issuing_authorizations_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_issuing_authorizations_x_get(
        &self,
        authorization: String,
        params: &Option<GetAccount>,
    ) -> Result<IssuingAuthorization, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/issuing/authorizations/{}",
                    self.config.base, authorization
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<IssuingAuthorization>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_issuing_cardholders_get(
        &self,
        params: &Option<GetIssuingCardholders>,
    ) -> Result<GetIssuingCardholdersRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/issuing/cardholders", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetIssuingCardholdersRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_issuing_cardholders_get_st(
        &self,
        params: &GetIssuingCardholders,
    ) -> impl Stream<Item = Result<GetIssuingCardholdersRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_issuing_cardholders_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_issuing_cardholders_x_get(
        &self,
        cardholder: String,
        params: &Option<GetAccount>,
    ) -> Result<IssuingCardholder, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/issuing/cardholders/{}",
                    self.config.base, cardholder
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<IssuingCardholder>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_issuing_cards_get(
        &self,
        params: &Option<GetIssuingCards>,
    ) -> Result<GetIssuingCardsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/issuing/cards", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetIssuingCardsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_issuing_cards_get_st(
        &self,
        params: &GetIssuingCards,
    ) -> impl Stream<Item = Result<GetIssuingCardsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_issuing_cards_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_issuing_cards_x_get(
        &self,
        card: String,
        params: &Option<GetAccount>,
    ) -> Result<IssuingCard, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/issuing/cards/{}", self.config.base, card))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<IssuingCard>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_issuing_disputes_get(
        &self,
        params: &Option<GetIssuingDisputes>,
    ) -> Result<IssuingDisputeList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/issuing/disputes", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<IssuingDisputeList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_issuing_disputes_get_st(
        &self,
        params: &GetIssuingDisputes,
    ) -> impl Stream<Item = Result<IssuingDisputeList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_issuing_disputes_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_issuing_disputes_x_get(
        &self,
        dispute: String,
        params: &Option<GetAccount>,
    ) -> Result<IssuingDispute, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/issuing/disputes/{}",
                    self.config.base, dispute
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<IssuingDispute>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_issuing_settlements_get(
        &self,
        params: &Option<GetCoupons>,
    ) -> Result<GetIssuingSettlementsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/issuing/settlements", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetIssuingSettlementsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_issuing_settlements_get_st(
        &self,
        params: &GetCoupons,
    ) -> impl Stream<Item = Result<GetIssuingSettlementsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_issuing_settlements_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_issuing_settlements_x_get(
        &self,
        settlement: String,
        params: &Option<GetAccount>,
    ) -> Result<IssuingSettlement, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/issuing/settlements/{}",
                    self.config.base, settlement
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<IssuingSettlement>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_issuing_transactions_get(
        &self,
        params: &Option<GetIssuingTransactions>,
    ) -> Result<GetIssuingTransactionsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/issuing/transactions", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetIssuingTransactionsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_issuing_transactions_get_st(
        &self,
        params: &GetIssuingTransactions,
    ) -> impl Stream<Item = Result<GetIssuingTransactionsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_issuing_transactions_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_issuing_transactions_x_get(
        &self,
        transaction: String,
        params: &Option<GetAccount>,
    ) -> Result<IssuingTransaction, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/issuing/transactions/{}",
                    self.config.base, transaction
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<IssuingTransaction>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_mandates_x_get(
        &self,
        mandate: String,
        params: &Option<GetAccount>,
    ) -> Result<Mandate, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/mandates/{}", self.config.base, mandate))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Mandate>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_order_returns_get(
        &self,
        params: &Option<GetOrderReturns>,
    ) -> Result<GetOrderReturnsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/order_returns", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetOrderReturnsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_order_returns_get_st(
        &self,
        params: &GetOrderReturns,
    ) -> impl Stream<Item = Result<GetOrderReturnsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_order_returns_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_order_returns_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<OrderReturn, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/order_returns/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<OrderReturn>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_orders_get(&self, params: &Option<GetOrders>) -> Result<GetOrdersRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/orders", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetOrdersRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_orders_get_st(
        &self,
        params: &GetOrders,
    ) -> impl Stream<Item = Result<GetOrdersRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_orders_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_orders_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<Order, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/orders/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Order>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_payment_intents_get(
        &self,
        params: &Option<GetPaymentIntents>,
    ) -> Result<PaymentFlowsPaymentIntentList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/payment_intents", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PaymentFlowsPaymentIntentList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_payment_intents_get_st(
        &self,
        params: &GetPaymentIntents,
    ) -> impl Stream<Item = Result<PaymentFlowsPaymentIntentList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_payment_intents_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_payment_intents_x_get(
        &self,
        intent: String,
        params: &Option<GetSourcesSource>,
    ) -> Result<PaymentIntent, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/payment_intents/{}",
                    self.config.base, intent
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PaymentIntent>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_payment_methods_get(
        &self,
        params: &GetPaymentMethods,
    ) -> Result<PaymentFlowsPaymentMethodList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/payment_methods", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PaymentFlowsPaymentMethodList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_payment_methods_get_st(
        &self,
        params: &GetPaymentMethods,
    ) -> impl Stream<Item = Result<PaymentFlowsPaymentMethodList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_payment_methods_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_payment_methods_x_get(
        &self,
        payment_method: String,
        params: &Option<GetAccount>,
    ) -> Result<PaymentMethod, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/payment_methods/{}",
                    self.config.base, payment_method
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PaymentMethod>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_payouts_get(&self, params: &Option<GetPayouts>) -> Result<PayoutList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/payouts", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PayoutList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_payouts_get_st(
        &self,
        params: &GetPayouts,
    ) -> impl Stream<Item = Result<PayoutList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_payouts_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_payouts_x_get(
        &self,
        payout: String,
        params: &Option<GetAccount>,
    ) -> Result<Payout, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/payouts/{}", self.config.base, payout))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Payout>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_plans_get(&self, params: &Option<GetPlans>) -> Result<PlanList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/plans", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PlanList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_plans_get_st(
        &self,
        params: &GetPlans,
    ) -> impl Stream<Item = Result<PlanList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_plans_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_plans_x_get(
        &self,
        plan: String,
        params: &Option<GetAccount>,
    ) -> Result<Plan, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/plans/{}", self.config.base, plan))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Plan>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_prices_get(&self, params: &Option<GetPrices>) -> Result<PriceList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/prices", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PriceList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_prices_get_st(
        &self,
        params: &GetPrices,
    ) -> impl Stream<Item = Result<PriceList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_prices_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_prices_x_get(
        &self,
        price: String,
        params: &Option<GetAccount>,
    ) -> Result<Price, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/prices/{}", self.config.base, price))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Price>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_products_get(
        &self,
        params: &Option<GetProducts>,
    ) -> Result<GetProductsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/products", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetProductsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_products_get_st(
        &self,
        params: &GetProducts,
    ) -> impl Stream<Item = Result<GetProductsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_products_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_products_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<Product, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/products/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Product>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_promotion_codes_get(
        &self,
        params: &Option<GetPromotionCodes>,
    ) -> Result<GetPromotionCodesRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/promotion_codes", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetPromotionCodesRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_promotion_codes_get_st(
        &self,
        params: &GetPromotionCodes,
    ) -> impl Stream<Item = Result<GetPromotionCodesRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_promotion_codes_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_promotion_codes_x_get(
        &self,
        promotion_code: String,
        params: &Option<GetAccount>,
    ) -> Result<PromotionCode, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/promotion_codes/{}",
                    self.config.base, promotion_code
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PromotionCode>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_radar_early_fraud_warnings_get(
        &self,
        params: &Option<GetIssuerFraudRecords>,
    ) -> Result<RadarEarlyFraudWarningList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/radar/early_fraud_warnings",
                    self.config.base
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<RadarEarlyFraudWarningList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_radar_early_fraud_warnings_get_st(
        &self,
        params: &GetIssuerFraudRecords,
    ) -> impl Stream<Item = Result<RadarEarlyFraudWarningList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_radar_early_fraud_warnings_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_radar_early_fraud_warnings_x_get(
        &self,
        early_fraud_warning: String,
        params: &Option<GetAccount>,
    ) -> Result<RadarEarlyFraudWarning, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/radar/early_fraud_warnings/{}",
                    self.config.base, early_fraud_warning
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<RadarEarlyFraudWarning>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_radar_value_list_items_get(
        &self,
        params: &GetRadarValueListItems,
    ) -> Result<GetRadarValueListItemsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/radar/value_list_items", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetRadarValueListItemsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_radar_value_list_items_get_st(
        &self,
        params: &GetRadarValueListItems,
    ) -> impl Stream<Item = Result<GetRadarValueListItemsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_radar_value_list_items_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_radar_value_list_items_x_get(
        &self,
        item: String,
        params: &Option<GetAccount>,
    ) -> Result<RadarListListItem, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/radar/value_list_items/{}",
                    self.config.base, item
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<RadarListListItem>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_radar_value_lists_get(
        &self,
        params: &Option<GetRadarValueLists>,
    ) -> Result<GetRadarValueListsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/radar/value_lists", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetRadarValueListsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_radar_value_lists_get_st(
        &self,
        params: &GetRadarValueLists,
    ) -> impl Stream<Item = Result<GetRadarValueListsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_radar_value_lists_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_radar_value_lists_x_get(
        &self,
        value_list: String,
        params: &Option<GetAccount>,
    ) -> Result<RadarListList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/radar/value_lists/{}",
                    self.config.base, value_list
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<RadarListList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_recipients_get(
        &self,
        params: &Option<GetRecipients>,
    ) -> Result<GetRecipientsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/recipients", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetRecipientsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_recipients_get_st(
        &self,
        params: &GetRecipients,
    ) -> impl Stream<Item = Result<GetRecipientsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_recipients_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_recipients_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<UniGetRecipientsIdRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/recipients/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<UniGetRecipientsIdRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_refunds_get(
        &self,
        params: &Option<GetRefunds>,
    ) -> Result<GetRefundsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/refunds", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetRefundsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_refunds_get_st(
        &self,
        params: &GetRefunds,
    ) -> impl Stream<Item = Result<GetRefundsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_refunds_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_refunds_x_get(
        &self,
        refund: String,
        params: &Option<GetAccount>,
    ) -> Result<Refund, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/refunds/{}", self.config.base, refund))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Refund>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_reporting_report_runs_get(
        &self,
        params: &Option<GetCoupons>,
    ) -> Result<GetReportingReportRunsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/reporting/report_runs", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetReportingReportRunsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_reporting_report_runs_get_st(
        &self,
        params: &GetCoupons,
    ) -> impl Stream<Item = Result<GetReportingReportRunsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_reporting_report_runs_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_reporting_report_runs_x_get(
        &self,
        report_run: String,
        params: &Option<GetAccount>,
    ) -> Result<ReportingReportRun, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/reporting/report_runs/{}",
                    self.config.base, report_run
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ReportingReportRun>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_reporting_report_types_get(
        &self,
        params: &Option<GetAccount>,
    ) -> Result<FinancialReportingFinanceReportTypeList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/reporting/report_types", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res
                    .json::<FinancialReportingFinanceReportTypeList>()
                    .await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_reporting_report_types_x_get(
        &self,
        report_type: String,
        params: &Option<GetAccount>,
    ) -> Result<ReportingReportType, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/reporting/report_types/{}",
                    self.config.base, report_type
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ReportingReportType>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_reviews_get(
        &self,
        params: &Option<GetCoupons>,
    ) -> Result<GetReviewsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/reviews", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetReviewsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_reviews_get_st(
        &self,
        params: &GetCoupons,
    ) -> impl Stream<Item = Result<GetReviewsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_reviews_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_reviews_x_get(
        &self,
        review: String,
        params: &Option<GetAccount>,
    ) -> Result<RadarReview, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/reviews/{}", self.config.base, review))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<RadarReview>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_setup_attempts_get(
        &self,
        params: &GetSetupAttempts,
    ) -> Result<PaymentFlowsSetupIntentSetupAttemptList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/setup_attempts", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res
                    .json::<PaymentFlowsSetupIntentSetupAttemptList>()
                    .await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_setup_attempts_get_st(
        &self,
        params: &GetSetupAttempts,
    ) -> impl Stream<Item = Result<PaymentFlowsSetupIntentSetupAttemptList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_setup_attempts_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_setup_intents_get(
        &self,
        params: &Option<GetSetupIntents>,
    ) -> Result<PaymentFlowsSetupIntentList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/setup_intents", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<PaymentFlowsSetupIntentList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_setup_intents_get_st(
        &self,
        params: &GetSetupIntents,
    ) -> impl Stream<Item = Result<PaymentFlowsSetupIntentList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_setup_intents_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_setup_intents_x_get(
        &self,
        intent: String,
        params: &Option<GetSourcesSource>,
    ) -> Result<SetupIntent, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/setup_intents/{}", self.config.base, intent))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<SetupIntent>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_sigma_scheduled_query_runs_get(
        &self,
        params: &Option<GetCountrySpecs>,
    ) -> Result<GetSigmaScheduledQueryRunsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/sigma/scheduled_query_runs",
                    self.config.base
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetSigmaScheduledQueryRunsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_sigma_scheduled_query_runs_get_st(
        &self,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<GetSigmaScheduledQueryRunsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_sigma_scheduled_query_runs_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_sigma_scheduled_query_runs_x_get(
        &self,
        scheduled_query_run: String,
        params: &Option<GetAccount>,
    ) -> Result<ScheduledQueryRun, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/sigma/scheduled_query_runs/{}",
                    self.config.base, scheduled_query_run
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ScheduledQueryRun>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_skus_get(&self, params: &Option<GetSkus>) -> Result<GetSkusRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/skus", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetSkusRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_skus_get_st(
        &self,
        params: &GetSkus,
    ) -> impl Stream<Item = Result<GetSkusRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_skus_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_skus_x_get(
        &self,
        id: String,
        params: &Option<GetAccount>,
    ) -> Result<UniGetSkusIdRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/skus/{}", self.config.base, id))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<UniGetSkusIdRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_sources_x_get(
        &self,
        source: String,
        params: &Option<GetSourcesSource>,
    ) -> Result<Source, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/sources/{}", self.config.base, source))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Source>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_sources_x_mandate_notifications_x_get(
        &self,
        mandate_notification: String,
        source: String,
        params: &Option<GetAccount>,
    ) -> Result<SourceMandateNotification, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/sources/{}/mandate_notifications/{}",
                    self.config.base, source, mandate_notification
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<SourceMandateNotification>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_sources_x_source_transactions_get(
        &self,
        source: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<ApmsSourcesSourceTransactionList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/sources/{}/source_transactions",
                    self.config.base, source
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<ApmsSourcesSourceTransactionList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_sources_x_source_transactions_get_st(
        &self,
        source: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<ApmsSourcesSourceTransactionList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_sources_x_source_transactions_get(source.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_sources_x_source_transactions_x_get(
        &self,
        source: String,
        source_transaction: String,
        params: &Option<GetAccount>,
    ) -> Result<SourceTransaction, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/sources/{}/source_transactions/{}",
                    self.config.base, source, source_transaction
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<SourceTransaction>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_subscription_items_get(
        &self,
        params: &GetSubscriptionItems,
    ) -> Result<GetSubscriptionItemsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/subscription_items", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetSubscriptionItemsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_subscription_items_get_st(
        &self,
        params: &GetSubscriptionItems,
    ) -> impl Stream<Item = Result<GetSubscriptionItemsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_subscription_items_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_subscription_items_x_get(
        &self,
        item: String,
        params: &Option<GetAccount>,
    ) -> Result<SubscriptionItem, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/subscription_items/{}",
                    self.config.base, item
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<SubscriptionItem>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_subscription_items_x_usage_record_summaries_get(
        &self,
        subscription_item: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/subscription_items/{}/usage_record_summaries",
                    self.config.base, subscription_item
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res
                    .json::<GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRes>()
                    .await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_subscription_items_x_usage_record_summaries_get_st(
        &self,
        subscription_item: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<
        Item = Result<GetSubscriptionItemsSubscriptionItemUsageRecordSummariesRes, UniErr>,
    > + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_subscription_items_x_usage_record_summaries_get(subscription_item.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_subscription_schedules_get(
        &self,
        params: &Option<GetSubscriptionSchedules>,
    ) -> Result<GetSubscriptionSchedulesRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/subscription_schedules", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetSubscriptionSchedulesRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_subscription_schedules_get_st(
        &self,
        params: &GetSubscriptionSchedules,
    ) -> impl Stream<Item = Result<GetSubscriptionSchedulesRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_subscription_schedules_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_subscription_schedules_x_get(
        &self,
        schedule: String,
        params: &Option<GetAccount>,
    ) -> Result<SubscriptionSchedule, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/subscription_schedules/{}",
                    self.config.base, schedule
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<SubscriptionSchedule>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_subscriptions_get(
        &self,
        params: &Option<GetSubscriptions>,
    ) -> Result<GetSubscriptionsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/subscriptions", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetSubscriptionsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_subscriptions_get_st(
        &self,
        params: &GetSubscriptions,
    ) -> impl Stream<Item = Result<GetSubscriptionsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_subscriptions_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_subscriptions_x_get(
        &self,
        subscription_exposed_id: String,
        params: &Option<GetAccount>,
    ) -> Result<Subscription, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/subscriptions/{}",
                    self.config.base, subscription_exposed_id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Subscription>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_tax_rates_get(
        &self,
        params: &Option<GetTaxRates>,
    ) -> Result<GetTaxRatesRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/tax_rates", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetTaxRatesRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_tax_rates_get_st(
        &self,
        params: &GetTaxRates,
    ) -> impl Stream<Item = Result<GetTaxRatesRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_tax_rates_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_tax_rates_x_get(
        &self,
        tax_rate: String,
        params: &Option<GetAccount>,
    ) -> Result<TaxRate, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/tax_rates/{}", self.config.base, tax_rate))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TaxRate>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_terminal_locations_get(
        &self,
        params: &Option<GetCountrySpecs>,
    ) -> Result<TerminalLocationLocationList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/terminal/locations", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TerminalLocationLocationList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_terminal_locations_get_st(
        &self,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<TerminalLocationLocationList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_terminal_locations_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_terminal_locations_x_get(
        &self,
        location: String,
        params: &Option<GetAccount>,
    ) -> Result<TerminalLocationLocation, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/terminal/locations/{}",
                    self.config.base, location
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TerminalLocationLocation>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_terminal_readers_get(
        &self,
        params: &Option<GetTerminalReaders>,
    ) -> Result<TerminalReaderRetrieveReader, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/terminal/readers", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TerminalReaderRetrieveReader>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_terminal_readers_get_st(
        &self,
        params: &GetTerminalReaders,
    ) -> impl Stream<Item = Result<TerminalReaderRetrieveReader, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_terminal_readers_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_terminal_readers_x_get(
        &self,
        reader: String,
        params: &Option<GetAccount>,
    ) -> Result<TerminalReaderReader, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/terminal/readers/{}",
                    self.config.base, reader
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TerminalReaderReader>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_tokens_x_get(
        &self,
        token: String,
        params: &Option<GetAccount>,
    ) -> Result<Token, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/tokens/{}", self.config.base, token))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Token>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_topups_get(&self, params: &Option<GetTopups>) -> Result<TopupList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/topups", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TopupList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_topups_get_st(
        &self,
        params: &GetTopups,
    ) -> impl Stream<Item = Result<TopupList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_topups_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_topups_x_get(
        &self,
        topup: String,
        params: &Option<GetAccount>,
    ) -> Result<Topup, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/topups/{}", self.config.base, topup))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Topup>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_transfers_get(
        &self,
        params: &Option<GetTransfers>,
    ) -> Result<TransferList, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/transfers", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TransferList>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_transfers_get_st(
        &self,
        params: &GetTransfers,
    ) -> impl Stream<Item = Result<TransferList, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_transfers_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_transfers_x_reversals_get(
        &self,
        id: String,
        params: &Option<GetCountrySpecs>,
    ) -> Result<TransferReversalList620BF1, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/transfers/{}/reversals",
                    self.config.base, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TransferReversalList620BF1>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_transfers_x_reversals_get_st(
        &self,
        id: String,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<TransferReversalList620BF1, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_transfers_x_reversals_get(id.clone(),&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_transfers_x_get(
        &self,
        transfer: String,
        params: &Option<GetAccount>,
    ) -> Result<Transfer, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/transfers/{}", self.config.base, transfer))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<Transfer>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_transfers_x_reversals_x_get(
        &self,
        id: String,
        transfer: String,
        params: &Option<GetAccount>,
    ) -> Result<TransferReversal, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/transfers/{}/reversals/{}",
                    self.config.base, transfer, id
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<TransferReversal>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub async fn v1_webhook_endpoints_get(
        &self,
        params: &Option<GetCountrySpecs>,
    ) -> Result<GetWebhookEndpointsRes, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!("{}/v1/webhook_endpoints", self.config.base))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<GetWebhookEndpointsRes>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }

    pub fn v1_webhook_endpoints_get_st(
        &self,
        params: &GetCountrySpecs,
    ) -> impl Stream<Item = Result<GetWebhookEndpointsRes, UniErr>> + '_ {
        let mut has_more = true;
        let mut p = (*params).clone();

        try_stream! {
             while has_more {
                let res = self.v1_webhook_endpoints_get(&p.clone().into()).await?;

                has_more = res.get_has_more();
                if let Some((_, last)) = res.get_from_to() {
                    p.set_after(last);
                }

                yield res;
             }
        }
    }

    pub async fn v1_webhook_endpoints_x_get(
        &self,
        webhook_endpoint: String,
        params: &Option<GetAccount>,
    ) -> Result<NotificationWebhookEndpoint, UniErr> {
        let get_req = || -> RequestBuilder {
            let mut req: RequestBuilder = self
                .client
                .get(&format!(
                    "{}/v1/webhook_endpoints/{}",
                    self.config.base, webhook_endpoint
                ))
                .query(&params.to_query_kv());

            req = req.header("Content-Type", "application/x-www-form-urlencoded");
            req = self.set_headers(req, &None);
            req
        };

        // let res = req.send().await?;
        let res = self.retry(get_req).await?;

        match res.status() {
            StatusCode::OK => {
                let x = res.json::<NotificationWebhookEndpoint>().await?;
                Ok(x)
            }
            _ => {
                let err = res.json::<Error>().await?;
                Err(err.into())
            }
        }
    }
}
