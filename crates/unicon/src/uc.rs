#![allow(warnings)]

use log::{trace, debug, info, warn, error};

use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::hash::BuildHasherDefault;

use std::path::Path;
use std::path::PathBuf;

use mysql::*;
use mysql::chrono::NaiveDateTime;
use mysql::prelude::*;
use regex::Regex;
use rusqlite::{Connection, NO_PARAMS, params, Result, Row, ToSql, Transaction, TransactionBehavior};
use rusqlite::ffi::Error;
use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef};
use serde::{Deserialize, Deserializer, Serialize};
use twox_hash::XxHash;
use crate::engines::placeholder::{PlaceholderString, PlaceholderFuncStd, PlaceholderFuncStdStatic};
use crate::engines::mysql::{MySQLFuncX, MySQLFuncXStatic};
use crate::engines::sqlite::{SQLiteFuncRusqlite, SQLiteFuncRusqliteStatic};
use crate::table::{Table, ObjType, CreateSQLObj, StaticSQLStrings};
use crate::utx::UniTx;
use crate::traits::DbStatic;
use mysql::consts::CapabilityFlags;
use std::{thread, fs};
use std::time::Duration;
use postgres::{Client, NoTls};
use core::fmt;
use crate::engines::postgres::PostgresFuncXStatic;


use postgres::config::SslMode;


use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::{MakeTlsConnector as MakeTlsConnectorOpenSSL, MakeTlsConnector};
use std::borrow::Cow;


impl From<Vec<StaticSQLStrings>> for Schema {
    fn from(x: Vec<StaticSQLStrings>) -> Self {
        let mut tables = vec![];
        for x2 in x {
            tables.push(x2.into())
        }
        Schema {
            // Not set for SQLite, for MySQL this comes from the runtime/config/user (for MySQL use mut and write schema when it is known from runtime value).
            schema: None,
            tables,
        }
    }
}


// pub struct FK {
//     pub from_tbl: String,
//     pub from_cols: Vec<String>,
//     pub to_tbl: String,
//     pub to_cols: Vec<String>,
// }
//
// impl FK {
//     /// Roughly standard SQL that `EngineString` traits can call.
//     /// - Provides default dialect, allow `EngineString` trait to create their own string *from the data* if needed.
//     pub fn to_std_sql(&self) -> String {
//         format!(
//             "FOREIGN KEY({}) REFERENCES {}({})",
//             self.to_cols.join(","),
//             self.from_tbl,
//             self.from_cols.join(","),
//         )
//     }
// }


#[derive(Debug)]
pub enum UniCon {
    Rusqlite(ConMetaSQLite),
    MySQL(ConMetaMySQL),
    Postgres(ConMetaPostgres),
    PlaceholderLibA(u32),
}

// Use `pub field_x` to allow direct read/write (instead of constructor fn args).
// - Allow easy read/write to JSON-like configs.
// - Use the `enum UniConCreate` in place of many fn args (so it is referencable as an obj pointer to allow passing/storing).
// - Do not depend on lib-specific types that often use constructors and hide their internal data once it passed through the constructor.
#[derive(Debug)]
pub struct UniConCreate {
    pub engine: Engine,

    // ... any other fields/options that are shared among all connections.
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Engine {
    #[serde(rename = "sqlite")]
    SQLite(SQLiteCreate),

    #[serde(rename = "mysql")]
    MySQL(MySQLCreate),

    #[serde(rename = "postgres")]
    Postgres(MySQLCreate),

    // Excel?
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct SQLiteCreate {
    pub file: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct MySQLCreate {
    // Copied from `InnerOpts` of `mysql` crate.
    pub addr: Addr,

    pub user: Option<String>,
    pub pass: Option<String>,


    // Tree structure of database objects:
    // - database/schema/table (Postgres, Oracle etc).
    // - schema/table (MySQL)
    // - table (SQLite)
    //
    // MySQL treats database and schema as the same thing.
    // - UniCon maps MySQL `database` to the same meaning as `schema` in the other engines.
    //      - This is because the `database` would be the most general term, where as `schema` is more specific and would be more relevant to a query writing end user:
    //          - E.g. tdog/stripe/table_x (MySQL would ignore `tdog` and create `stripe` as the containing db-schema).


    // MySQL ignores this.
    pub db_name: Option<String>,

    // @todo/low What if one `UniCon` is used to read/write many schemas? Fix, temp: One connection per db/schema.
    // These are `Option` so that a user provided config can optionally set them (if they are not set, the application should choose a default; UniCon expects them to be set).
    pub schema_name: Option<String>,

    // Postgres
    // - @see https://stackoverflow.com/questions/2370525/default-database-named-postgres-on-postgresql-server
    //      - `postgres` is the default database for a connection, which matches the user.
    // - @see https://stackoverflow.com/questions/2370525/default-database-named-postgres-on-postgresql-server
    //      - One database, many schemas preferred as you cannot join across databases.


    // If None, do not set up a TLS, send data over unencrypted connection (OK for LAN databases).
    // If Some, always use TLS.
    pub tls: Option<Tls>,
}

impl MySQLCreate {
    // Basic logic for testing if this is a valid config (beyond just types).
    pub fn assert_is_valid(&self) {
        match &self.tls {
            None => {}
            Some(x) => {
                if let Some(f) = &x.ca_cert_file {
                    assert!(Path::new(f.as_str()).exists(), "`ca_cert_file` - file does not exist.");
                }
            }
        }
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Tls {
    // - @todo/maybe Add an optional string of the ca_cert later to try to keep the JSON config self contained.
    //      - Allows HTTP POST'ing the config over the network to the CLI.
    //      - Container OS configs (cloud-init) can just use the JSON value (instead of extra steps of copying the CA files).
    //      - JSON strings, when pretty printed, typically take a single long line in the editor so will not impact readability.

    // Similar DB CLI client args:
    // - MySQL CLI: `--ssl-ca`
    //      - @see https://dev.mysql.com/doc/mysql-shell/8.0/en/mysql-shell-encrypted-connections.html
    // - Postgres CLI: `--sslrootcert`
    //      - @see https://www.postgresql.org/docs/9.1/libpq-connect.html

    // Path to a `.pem` or `.crt` file.
    // - During the TLS handshake, the client uses this CA certificate to determine if the certificate returned by the server should be trusted (was previously signed by the CA private key).
    // @see https://stackoverflow.com/a/590169/4949386

    // Possible issue: Many CA certs?
    // Note: Self signed certs can be both the CA cert and the server cert.
    // If None, just use the systems CA certs (server cert must be signed by a real public CA).
    pub ca_cert_file: Option<String>,

    // Whether or not to verify that the TLS connection target server is the same one identified in the server certificate.
    // - Note: When false, `MITM` attacks are possible if the CA cert is used to sign server certs owned/operated by other people other than yourself.
    // - None | true = Roughly equal to Postgres CLI `verify-full` or MySQL CLI `VERIFY_IDENTITY`.
    // - false = Roughly equal to `verify-ca`
    //
    // - When starting a TLS connection, the server returns a TLS certificate.
    //      - This is signed by the CA cert (basic verification = trust).
    //      - The server certificate also contains a hostname it is valid for.
    //          - This should match the target machine (IP) of the TLS connection being created (hostname verification = correct target machine, SSL server certificate has not been moved to a new IP).
    //          - TLS happens at the application/user-space layer, the hostname check is against the IP from layer 3.
    //              - This extends "assert" logic from layer 7 to layer 3.
    // - Why this might be disabled.
    //      - Some cloud database instances do not return server certificates with a Common Name that works with hostname verification.
    //          - E.g. A GCP SQL server returns `td-dev-123:del-mysql-ins-a`, which is not a domain or IP.
    //              - They instruct you to set `verify-ca` for both MySQL and Postgres (instead of `verify-full` and `VERIFY_IDENTITY` which check hostnames).
    //              - This is OK as the CA used is created per-db-instance.
    //                  - The CA has signed a single server cert, so it must be for the target hostname.
    //                      - It is not possible that another user is in control of a CA-signed server key/cert pair that they use on a different domain (MITM); so there is no use checking the domain for CA-signed server certs.
    //      - Using TLS with macOS (via `native-tls`) is very strict.
    //          - E.g. Supabase CA and server certs are valid for `openssl` with hostname checks (`openssl s_client -verify_hostname x`), but fail when using `native-tls` on macOS.
    //              - macOS must be doing extra hostname checks.
    //                  - A better fix is to add the server cert to macOS keychain and set Always Trust.
    // - Rust booleans default to `false` which is why it is wrapped in an Option as this is a bad default.
    #[serde(default = "default_verify_server_cert_hostname")]
    pub verify_server_cert_hostname: Option<bool>,
}


fn default_verify_server_cert_hostname() -> Option<bool> {
    Some(true)
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
#[serde(untagged)]
// Use structs instead of tuple/string to allow Serde's untagged enums to guess which variant it is based on data shape.
// - Reduces the amount of JSON levels needed in the config making them shorter - no chance of selecting the wrong one.
pub enum Addr {
    IP(IPPort),
    Socket(Socket),
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct IPPort {
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Socket {
    pub socket: String,

    // May have options for windows sockets in the future.
}

#[derive(Debug)]
pub struct ConMetaSQLite {
    pub create: SQLiteCreate,
    pub c: Connection,
}

#[derive(Debug)]
pub struct ConMetaMySQL {
    pub create: MySQLCreate,
    pub c: Conn,
}

pub struct ConMetaPostgres {
    // Use `MySQLCreate` until the create options need to be customised (generally means "network based SQL server").
    pub create: MySQLCreate,

    // Client is used as the implementation uses Tokio and sync blocks on each promise.
    pub c: Client,
}

// Note: postgres::Client does not impl trait Debug.
impl fmt::Debug for ConMetaPostgres {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?})", self.create)
    }
}


impl UniCon {
    pub fn new(ucc: &UniConCreate) -> Self {

        // error!("error Creating UniCon");
        // warn!("warn Creating UniCon");
        // info!("info Creating UniCon");
        // debug!("debug Creating UniCon");
        // trace!("trace Creating UniCon");

        match &ucc.engine {
            Engine::SQLite(create) => {
                let c = Connection::open(&create.file).unwrap();

                c.busy_timeout(Duration::from_millis(30 * 1000));

                // c.busy_handler(Some(|times| -> bool {
                //     if times > 5 {
                //         // Return SQLITE_BUSY to application.
                //         return false;
                //     }
                //
                //     let wait_ms = match times {
                //         0 => 5,
                //         1 => 10,
                //         2 => 50,
                //         3 => 100,
                //         4 => 1000,
                //         5 => 10000,
                //         _ => unreachable!()
                //     };
                //
                //     info!("SQLite file locked, will retry after {}ms.", wait_ms);
                //     thread::sleep(Duration::from_millis(wait_ms));
                //     true
                // })).unwrap();

                UniCon::Rusqlite(ConMetaSQLite {
                    create: (*create).clone(),
                    c,
                })
            }
            Engine::MySQL(create) => {
                create.assert_is_valid();
                assert!(create.db_name.is_none(), "MySQL: use `schema_name` instead of `db_name`. These have identical meanings in MySQL, but Postgres and other SQL engines use a database/schema/table nesting.");
                assert!(create.schema_name.is_some(), "MySQL: `schema_name` must be set.");

                let get_ops_builder = || {
                    let mut x = OptsBuilder::new();
                    let mut x2 = create.clone();

                    x = x.user(x2.user);
                    x = x.pass(x2.pass);
                    // x = x.db_name(x2.db_name.into());

                    match x2.addr {
                        Addr::IP(x3) => {
                            x = x.ip_or_hostname(x3.ip.into());
                            x = x.tcp_port(x3.port);
                        }
                        Addr::Socket(x3) => {
                            x = x.socket(x3.socket.into());
                        }
                    }


                    if let Some(tls) = &create.tls {
                        // - Uses `native-tls` (stricter CA cert checking on macOS).
                        let mut opts = SslOpts::default();

                        if let Some(file) = &tls.ca_cert_file {
                            let mut cow = Cow::from(PathBuf::from(&file));
                            opts = opts.with_root_cert_path(Some(cow));
                        } else {
                            // Use the systems keychain (official CA's like Lets Encrypt).
                        }

                        if let Some(verify) = tls.verify_server_cert_hostname {
                            if !verify {
                                opts = opts.with_danger_skip_domain_validation(true);
                            }
                        }

                        // Issue: macOS has very strict server certificate requirements (`mysql` crate only uses `native-tls`, not option for using `openssl`).
                        // - Strict certificate requirements.
                        //      - https://github.com/sfackler/rust-native-tls/issues/143
                        //      - https://support.apple.com/en-us/HT210176
                        //          - macOS Console: `Trust evaluate failure: [leaf ValidityPeriodMaximums]`
                        //              - Caused by: `TLS server certificates must have a validity period of 825 days or fewer`
                        //              - Google Cloud SQL server certificates are valid for 10 years.
                        // - So even with the CA cert added to keychain, set to Always Trust, and ignore domain validation, the TLS connection still fails due to macOS rejecting the server certificate.
                        //      - Dates checked with `/opt/homebrew/opt/openssl@1.1/bin/openssl s_client -starttls mysql -connect 35.246.65.1:3306 -showcerts -CAfile /Users/enzo/Downloads/gcp-ca-mysql-ins-a.pem | openssl x509 -noout -dates`
                        //
                        // Fix: Add the *server* cert to keychain, set Always Trust
                        // - Get the server cert with  `echo "" | /opt/homebrew/opt/openssl@1.1/bin/openssl s_client -starttls mysql -connect 35.246.65.1:3306 -showcerts -CAfile /Users/enzo/Downloads/gcp-ca-mysql-ins-a.pem`

                        // During development, ignore all SSL cert/keys issues (insecure, trusts any server).
                        if cfg!(debug_assertions) && cfg!(target_os = "macos") {
                            warn!("MySQL TLS connections will accept invalid certificates (only for Rust debug builds to allow self-signed certs).");

                            // The MySQL crate uses `native-tls`. On macOS this requires adding the CA cert to the keychain and setting Always Trust.
                            // - Avoiding this as the `snakeoil` private key is public (from the Docker image).
                            opts = opts.with_danger_accept_invalid_certs(true);
                        } else {
                            // To use self signed certs user will have to add CA cert to their OS's trusted certificate store.
                        }

                        x = x.ssl_opts(opts);
                    }


                    // By default MySQL `affected_rows()` means "rows with at least one column with a new value".
                    // - This means an UPDATE with the same values as the existing rows will return 0, which differs from the other engines.
                    // - CLIENT_FOUND_ROWS changes `affected_rows()` to mean "count of rows matched in the where clause".
                    //      - This means an update with the same values essentially says "ensure this is the current state".
                    //          - An `affected_rows()=0` is an error as the row does not exist in this case.
                    //
                    // This flag is needed as `tx_update` returns affected rows, and SQLites `changes()` means `FOUND_ROWS`
                    x = x.additional_capabilities(CapabilityFlags::CLIENT_FOUND_ROWS);

                    x
                };

                // Note: `Conn::new` panics when db_name is set but does not exist.
                let mut x = get_ops_builder();
                let mut c = Conn::new(x).unwrap();

                // When strings > 255 are inserted into `varchar(255)`, throw error.
                // - See `ToSQLString<MySQL> for Table` comment.
                // @see https://dev.mysql.com/doc/refman/5.7/en/sql-mode.html#sqlmode_strict_all_tables
                c.query_drop("SET SESSION sql_mode = 'STRICT_ALL_TABLES'").unwrap();
                c.query_drop("SET SESSION default_storage_engine = INNODB;").unwrap();
                c.query_drop("SET SESSION innodb_strict_mode = 'ON'").unwrap();

                UniCon::MySQL(ConMetaMySQL {
                    create: (*create).clone(),
                    c,
                })
            }
            Engine::Postgres(create) => {
                create.assert_is_valid();
                assert!(create.schema_name.is_some(), "Postgres: `schema_name` must be set.");

                let c2 = create.clone();
                let mut config = Client::configure();

                config.user(c2.user.unwrap().as_str());
                config.password(c2.pass.unwrap().as_str());

                match c2.addr {
                    Addr::IP(x) => {
                        config.host(x.ip.as_str());
                        config.port(x.port);
                    }
                    Addr::Socket(x) => {
                        // Issue: Type error when compiling for `x86_64-pc-windows-gnu` (method is only included for unix).
                        assert!(cfg!(unix), "Postgres socket is only supported on Unix. Use IP and port instead. Incorrect socket usage: {}", x.socket.as_str());

                        #[cfg(unix)]
                            {
                                config.host_path(x.socket.as_str());
                            }
                    }
                }

                // Assumption: User has created this db ahead of time.
                if let Some(db_name) = &create.db_name {
                    config.dbname(db_name);
                } else {
                    // - Rust-postgres defaults to using a db name the same as the user name.
                    // - The Postgres CLI uses the `postgres` db by default that seems to always exist on a fresh Postgres server.
                }


                let mut get_con = || {
                    if let Some(tls) = &create.tls {
                        // Using openssl instead of `native-tls` as macOS is much stricter in rejecting ca certs.
                        let mut builder = SslConnector::builder(SslMethod::tls_client()).unwrap();

                        if let Some(file) = &tls.ca_cert_file {
                            builder.set_ca_file(file.as_str()).unwrap();
                        } else {
                            // Use the systems keychain (official CA's like Lets Encrypt).
                        }


                        // Do not offer this as a CLI option to force only secure configs (or else explicitly opt out of TLS).
                        // builder.set_verify(SslVerifyMode::NONE);

                        // https://cloud.google.com/sql/docs/postgres/connect-admin-ip#connect-ssl
                        // - Note: "An SSL mode of verify-full is not required; verify-ca is enough because the CA is instance-specific."
                        //      - This means that the IP of the DB can change, and will be trusted as long as the server proves it has a private key signed by the provided CA cert.

                        // https://www.postgresql.org/docs/9.1/libpq-ssl.html
                        // - "If a public CA is used, verify-ca allows connections to a server that somebody else may have registered with the CA. In this case, verify-full should always be used. If a local CA is used, or even a self-signed certificate, using verify-ca often provides enough protection."

                        // Summary: If the CA private key is only used to sign server certs for servers you control/trust, `verify-ca` is ok as it skips the IP check (allows moving server around).
                        // - But, if the CA private key signs certificates for many different servers, some controlled by other people you do not trust, host/ip verification ensures that the connection is only trusted for your server and not others.


                        let mut x: SslConnector = builder.build();
                        let mut connector_openssl: MakeTlsConnector = MakeTlsConnectorOpenSSL::new(x);

                        if let Some(verify) = tls.verify_server_cert_hostname {
                            if !verify {
                                connector_openssl.set_callback(|config, domain| {
                                    // Issue: GCP requires `verify-ca` (no hostname verify), Supabase works with `verify-full` (verify hostname).
                                    config.set_verify_hostname(false);
                                    Ok(())
                                });
                            }
                        }

                        config.ssl_mode(SslMode::Require);
                        return config.connect(connector_openssl).unwrap();
                    }

                    config.connect(NoTls).unwrap()
                };


                // @see https://dba.stackexchange.com/questions/118178/does-updating-a-row-with-the-same-value-actually-update-the-row
                // - `changes` means "number of rows matching the WHERE clause, regardless of if the actual values changed"
                //      - Same as SQLite.
                //      - Differs to MySQL by default.

                UniCon::Postgres(ConMetaPostgres {
                    create: create.clone(),
                    c: get_con(),
                })
            }
        }
    }


    // @todo/next Report which tables were created/already existing to enable logging.
    /// Usage: `uc.ensure_schema_and_writable::<Db>()`
    /// @todo/low Some type of migration system to check if the tables already exist when moving the next version of an app.
    /// - Fix, temp: Each app should implement its own system (e.g. for user-read-only tables, store drop statements in the database at create time, the next version of the binary will read, apply drop statements and then re-create all tables without having to know which tables existed in the previous version).
    pub fn ensure_schema_and_tables_exist_and_writable<T>(&mut self) -> (bool, Schema) where T: DbStatic {
        let mut created_all = false;

        let schema_opt = self.get_user_defined_schema();

        // Get the target schema (schema and tables this program expects to write to).
        let target_schema = {
            let mut x = match self {
                UniCon::Rusqlite(_) => T::get_target_schema_sqlite(),
                UniCon::MySQL(_) => T::get_target_schema_mysql(),
                UniCon::Postgres(_) => T::get_target_schema_postgres(),
                UniCon::PlaceholderLibA(_) => unreachable!()
            };

            // Add user defined schema name from config.
            if let Some(x2) = &schema_opt {
                x.schema = Some(NameCreate {
                    name: x2.name.clone(),
                    create: x2.create.clone(),
                })
            }

            x
        };

        // Before: Set connection-level options before creating the schema.
        {
            match self {
                UniCon::Rusqlite(x) => {
                    // Enable WAL mode:
                    // - Increase write speed (batches up writes into a single fsync call).
                    // - Allow concurrent reading of DB file whilst its being written.
                    // - @todo/low Place behind config option in case file exists with user data in it.
                    x.c.pragma_update(None, "journal_mode", &"WAL").unwrap();

                    // Writes are safe from process crashes, but not OS crashes.
                    x.c.pragma_update(None, "synchronous", &"NORMAL").unwrap();

                    // This is per connection; does not persist with file.
                    x.c.pragma_update(None, "foreign_keys", &"ON").unwrap();
                }
                _ => {}
            }
        }


        // Create DB, tables and indexes if not exists.
        // @todo/med Detect later version, drop database and re-download all.
        {
            let mut utx = self.tx_open().unwrap();

            let existing_schema = utx.get_existing_schema(schema_opt.as_ref().and_then(|x| x.name.as_str().into()));
            let (all_exist, none_exist, diff) = Schema::diff(&target_schema, &existing_schema);

            // When: User is trying to write to an existing schema with table conflicts. Different db/cli versions.
            assert!(all_exist || none_exist, "Some tables do not exist. These tables are required: {:?}", &diff);

            if none_exist {
                let tbls_indexes = target_schema.get_create_tables_and_indexes();

                match utx {
                    UniTx::Rusqlite(ref tx) => {
                        assert_eq!(target_schema.schema, None);

                        for x in tbls_indexes {
                            let changes = tx.execute(&x, NO_PARAMS).unwrap();
                        }
                    }
                    UniTx::MySQL(_) | UniTx::Postgres(_) => {
                        let ts = target_schema.schema.as_ref().unwrap();

                        // When: (schema exists && tables do not) (E.g. empty schema pre-created, or schema with another non-td processes tables in it).
                        if existing_schema.schema.is_none() {
                            utx.exec_one(&ts.create);
                        }

                        // Note:
                        // - MySQL: This sets the active schema at the connection level (no tx isolation).
                        // - Postgres: If commit, persists on connection, if rollback does not persist.
                        utx.set_active_schema(&ts.name);

                        for x in tbls_indexes {
                            utx.exec_one(&x);
                        }
                    }
                    UniTx::PlaceholderLibA(_) => {}
                }


                created_all = true;
            }

            utx.tx_close();
        }

        // After: Set connection-level options after creating the schema (or determining that it exists from before).
        {

            // Set active schema on connection.
            let mut utx = self.tx_open().unwrap();
            match utx {
                UniTx::Rusqlite(_) => {}
                UniTx::MySQL(_) | UniTx::Postgres(_) => {
                    let ts = target_schema.schema.as_ref().unwrap();
                    utx.set_active_schema(&ts.name);
                }
                UniTx::PlaceholderLibA(_) => {}
            }
            utx.tx_close();

            // match self {
            //     UniCon::MySQL(x) => {
            //         // Sets default schema for the connection.
            //         // - This must be set to avoid `ERROR 1046 (3D000): No database selected`
            //         x.c.select_db(x.create.schema_name.as_ref().unwrap());
            //     }
            //     _ => {}
            // };
        }


        (created_all, target_schema)
    }

    pub fn get_last_id(&self) -> i64 {
        match self {
            UniCon::Rusqlite(x) => {
                x.c.last_insert_rowid()
            }
            UniCon::MySQL(x) => {
                i64::try_from(x.c.last_insert_id()).unwrap()
            }
            UniCon::Postgres(x) => {
                unreachable!("Cannot `uc.get_last_id()` for Postgres as there is no way to reliably read it in the presence of triggers. See: https://stackoverflow.com/a/2944481/4949386 Fix: Use the ID returned from `insert` which uses `INSERT ... RETURNING id`");
            }
            UniCon::PlaceholderLibA(_) => {
                unimplemented!()
            }
        }
    }

    /// @todo/med DB may be locked; impl retry.
    /// @todo/high Merge errors from each lib into single normalised error (same for tx close too).
    pub fn tx_open(&mut self) -> Result<UniTx> {
        match self {
            UniCon::Rusqlite(x) => {
                // Immediate = get db file write lock before starting any SQL statements; fail here early if DB lock cannot be attained.
                // @todo/low loop if locked.
                // Note: This may prevent "read transactions" which would allow reading from a snapshot point in time whilst writes are still proceeding.
                Ok(UniTx::Rusqlite(x.c.transaction_with_behavior(TransactionBehavior::Immediate).unwrap()))
            }
            UniCon::MySQL(x) => {
                let mut opts = TxOpts::default();

                // REPEATABLE READ
                // - Issues:
                //      - Writes to rows will see other transactions writes (no isolation).
                //          - Protect against running multiple versions of the TD CLI against the same target DB (each tx should get the last event_id and commit its work only if another has not committed in the mean time).
                // SERIALIZABLE
                // - Strongest isolation level, mimics SQLites whole file locking.
                //      - May be an issue for download + high traffic DB, but events should apply very quickly.
                opts.set_isolation_level(Some(IsolationLevel::Serializable));
                Ok(UniTx::MySQL(x.c.start_transaction(opts).unwrap()))
            }
            UniCon::Postgres(x) => {
                let tx = x.c.build_transaction()
                    .isolation_level(postgres::IsolationLevel::Serializable)
                    .start().unwrap();

                Ok(UniTx::Postgres(tx))
            }
            UniCon::PlaceholderLibA(_) => {
                Ok(UniTx::PlaceholderLibA("wow".into()))
            }
        }
    }


    // It is possible that the user will give a custom schema name in a config.
    pub fn get_user_defined_schema(&self) -> Option<CreateSQLObj> {
        match self {
            UniCon::Rusqlite(_) => None,
            UniCon::MySQL(x) => CreateSQLObj {
                obj_type: ObjType::Schema,
                // Can be user defined with config; dynamic.
                name: x.create.schema_name.as_ref().unwrap().clone(),
                create: format!("CREATE SCHEMA {} CHARACTER SET utf8 COLLATE utf8_general_ci;", &x.create.schema_name.as_ref().unwrap()),
            }.into(),
            UniCon::Postgres(x) => CreateSQLObj {
                obj_type: ObjType::Schema,
                // Can be user defined with config; dynamic.
                name: x.create.schema_name.as_ref().unwrap().clone(),
                create: format!("CREATE SCHEMA {};", &x.create.schema_name.as_ref().unwrap()),
            }.into(),
            UniCon::PlaceholderLibA(_) => None,
        }
    }


    // Gets a Vec of a given type.
    // - The SQL query must return all cols (so they can be set on the Rust struct).
    // - `std_sql` is a SQL string that should be valid in all SQL dialects (typically just simple select queries).
    // - @todo/maybe
    //      - Allow setting params for multiple where statements.
    //      - Allow overriding the std_sql with engine_x_sql for when there is not common syntax.
    pub fn get_vec_from_sql<T>(&mut self, std_sql: &str) -> Vec<T>
        where T: SQLiteFuncRusqliteStatic + MySQLFuncXStatic + PostgresFuncXStatic {
        let mut v = vec![];

        match self {
            UniCon::Rusqlite(x) => {
                let mut stmt = x.c.prepare_cached(std_sql).unwrap();
                let mut rows = stmt.query(NO_PARAMS).unwrap();
                while let Some(x2) = rows.next().unwrap() {
                    v.push(<T as SQLiteFuncRusqliteStatic>::row_to_ins(&x2));
                }
            }
            UniCon::MySQL(x) => {
                let res = x.c.exec(std_sql, Params::Empty).unwrap();
                for mut x2 in res {
                    v.push(<T as MySQLFuncXStatic>::row_to_ins(&mut x2))
                }
            }
            UniCon::Postgres(x) => {
                let res = x.c.query(std_sql, &[]).unwrap();
                for mut x2 in res {
                    v.push(<T as PostgresFuncXStatic>::row_to_ins(&mut x2))
                }
            }
            UniCon::PlaceholderLibA(x) => {
                unreachable!()
            }
        }

        v
    }
}


// `Schema` is a common interface/type to write: (target schema, existing schema) so the names can be compared to compute `all_or_none_exists(target, existing) -> bool` to check if the tables can be written to on CLI start up.
// - If it is possible to parse create SQL tables into `Table`, this may be used in the future for detailed comparison (including column names, types, index cols and types etc).
//      - For now a simple name diff is sufficient (assumes that the user will not modify those tables).
//
// - Separate `name` from the create SQL string to allow logging and approximate logic (E.g. name X exists).
#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Schema {
    pub schema: Option<NameCreate>,
    pub tables: Vec<TableCreate>,
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NameCreate {
    pub name: String,
    pub create: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TableCreate {
    pub name: String,
    pub create: String,
    pub indexes: Vec<NameCreate>,
}


impl Schema {
    // Basic detection for missing tables that will be attempted to be written to by the current version.
    // - Detect if:
    //      - User deletes some tables that are needed.
    //      - (old db, new version)
    //      - (new db, old version).
    //          - Both these cases will be detected with a schema version stored in the DB in the future and checked on start up.
    //      - Tables already exist in the schema that are not owned by TD (E.g. user is writing tables to an existing schema and there are naming conflicts).
    pub fn diff(target: &Schema, existing: &Schema) -> (bool, bool, Vec<String>) {
        assert!(target.tables.len() > 0);

        let to_set = |x: &Vec<TableCreate>| -> HashSet<String> {
            x.iter().map(|x| x.name.clone()).collect()
        };

        let a = to_set(&target.tables);
        let b = to_set(&existing.tables);


        let diff: Vec<String> = a.difference(&b).into_iter().map(|x| x.clone()).collect();

        let all_exist = diff.len() == 0;
        let none_exist = diff.len() == a.len();

        (all_exist, none_exist, diff)
    }


    pub fn get_create_tables_and_indexes(&self) -> Vec<String> {
        let mut all = vec![];

        for x in &self.tables {
            all.push(x.create.clone());

            for x in &x.indexes {
                all.push(x.create.clone());
            }
        }

        all
    }
}


// Note: Do not allow this method of UniCon instantiation as it depends on the client code using the exact same lib types as the implementation of the UniCon.
// - Connections and options represented in JSON-like plain data is much more portable/future proof.
//      - Allows extra meta data with the connection details; options that may be normalised over all engines.
//          - May want to hold on to the connection create data so that:
//              - The actual connection can be dropped and re-created.
//              - The data that created the connection can be read (instead of trying to get it back from the engine connection instance; some libs are happy to take data in a constructor, but may not offer 100% of that data back in a read interface).
//      - Implementation can change freely.
//      - No issue with different lib versions when UniCon moves a engine lib forward a few versions.
// impl From<Connection> for UniCon {
//     fn from(c: Connection) -> Self {
//         UniCon::Rusqlite(c)
//     }
// }

// @todo/maybe For cases when you might have a connection OR a tx (connection in an active tx state).
// - This would be useful for passing into a function that just converts tree-structs to row-structs and writes them to the DB.
//      - The parent function could optionally start/commit the tx.
// Alternative: Make `UniRW` a trait, impl for both `UniCon` and `UniTx`
// pub enum UniRW {
//     UniCon(UniCon),
//     UniTx(UniTx),
// }

// impl UniRW {
//     pub fn insert<T: Insert>(&self, rw: T) {
//         match &self {
//             UniRW::UniCon(c) => {
//                 // rw.insert(c);
//             }
//             UniRW::UniTx(_) => {
//                 // rw.insert_tx(c);
//             }
//         }
//     }
// }


// - This allows implementing a "this trait OR that trait" type.
// - Not used as a struct will be converted to a row when it is needed, and the "which trait" logic will be encoded in the parent UniCon or UniTx.
// pub enum UniRow<A: SQLiteFuncRusqlite, B: PlaceholderFuncStd> {
//     SQLiteFuncRusqlite(A),
//     PlaceholderFuncStd(B)
// }
