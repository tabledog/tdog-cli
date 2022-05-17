// This TypeScript file documents the JSON configuration for the `tdog` CLI.
// - Edit the examples at the bottom. Use the TypeScript compiler messages to ensure the config is correct as you edit.
//      - Copy/paste this file into https://www.typescriptlang.org/play.


interface Config {
    cmd: Cmd,
    log?: LogLevel | LogConfig
}

type Cmd =
    { fn: "download", args: DownloadArgs };

interface DownloadArgs {
    from: From,
    to: To,
    options?: Options
}

type From =
    { stripe: Stripe };

type To =
    { sqlite: SQLite } |
    { mysql: MySQL } |
    { postgres: MySQL };

interface MySQL {
    addr: Addr,
    user?: string,
    pass?: string,

    // MySQL ignores this.
    db_name?: string,

    // Tree structure of database objects:
    // - database/schema/table (Postgres, Oracle etc).
    // - schema/table (MySQL)
    // - table (SQLite)
    schema_name?: string

    // If set to an object, TLS is always used (will not fall back to a non-TLS connection).
    tls?: Tls
}

type Addr = IPPort | Socket;

interface IPPort {
    ip: string,
    port: number
}

interface Socket {
    socket: string
}

interface Tls {
    // Absolute path to the CA file.
    // - During the TLS handshake, the client uses this CA certificate to determine if the certificate returned by the server should be trusted (was previously signed by the CA private key).
    // @see https://stackoverflow.com/a/590169/4949386
    //
    // - Fixing issues:
    //      - Web proxies: When running with web debugging proxies enabled (E.g macOS host with Charles, Docker guest), the proxy may try to intercept the TLS connection which will fail due to the Charles CA not being installed on the Docker guest OS.
    //      - macOS.
    //          - macOS has very strict certificate requirements.
    //              - Fix: Add both the `ca.cert` and the `server.crt` to your OS keychain and select "Always Trust".
    //                  - `openssl s_client` can be used to download and save the `server.crt`
    //
    // Similar DB CLI client args:
    // - MySQL CLI: `--ssl-ca`
    //      - @see https://dev.mysql.com/doc/mysql-shell/8.0/en/mysql-shell-encrypted-connections.html
    // - Postgres CLI: `--sslrootcert`
    //      - @see https://www.postgresql.org/docs/9.1/libpq-connect.html
    ca_cert_file?: string,

    // Whether to verify the hostname on the certificate returned by the server.
    // - `false` = Roughly equal to `verify-ca`
    //      - `false` can be safe to use if the CA cert only signs certificates for a single customer/owner.
    //          - E.g. with GCP managed SQL servers, each CA cert is generated per database instance, so if the server cert has been signed by the CA, hostname verification does not matter as it is the only possible instance that has a server certificate signed by that CA cert.
    //
    // Defaults to true.
    verify_server_cert_hostname?: boolean
}

interface Stripe {
    // E.g: `sk_...`
    // Create a read-only key from https://dashboard.stripe.com/apikeys
    secret_key: string,
    http?: HttpOpts,

    // During the first download, if true, on the first 429 the TD CLI will exit, rolling back the database transaction and aborting the download.
    // - This is to prevent any impact on other processes that may be reading/writing to the same Stripe account.
    //
    // Defaults to false.
    // - If false, all outstanding requests retried, new requests are not started until they have all completed.
    // - If you see 429 responses in the logs regularly, you will need to set a lower `max_requests_per_second`.
    exit_on_429?: boolean,

    // On the first download HTTP requests run concurrently. Each Stripe account has an upper concurrency/rate limit per second.
    // - A higher `max_requests_per_second` means a faster download.
    // - You can contact Stripe to increase this limit on your live account.
    // - See `/notes/large-accounts.md`.
    //
    // Defaults:
    // - Test: 10 (25 is the maximum).
    // - Live: 50 (100 is the maximum).
    max_requests_per_second?: number
}

interface HttpOpts {
    // Roughly equal to Linux env var `http_proxy`.
    // - Log all HTTP requests sent for debugging.
    proxy: Proxy
}


interface Proxy {
    url?: string
}


interface SQLite {
    // Absolute path to SQLite file.
    file: string
}

interface Options {
    // If true, the process continually polls `/events` and applies any new writes to the database. Defaults to false.
    watch?: boolean,
    // Time in ms between each poll, defaults to 400ms.
    poll_freq_ms?: number
}

// Logs output when they are equal or higher to LogLevel.
type LogLevel = "error" | "warn" | "info" | "debug" | "trace";

interface LogConfig {
    level?: LogLevel,
    groups: {
        [k in Group]: LogLevel | null
    },
    friendly_mod_names: boolean
}


enum Group {
    tdog = 'tdog',
    tdog_lib = 'tdog_lib',
    dep = 'dep',
}

// Edit any of these examples, save the JSON into a file.
// Run `tdog --json-file /abs/path/config.json`
const example_sqlite: Config = {
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "sk_test_abc123"
                }
            },
            "to": {
                "sqlite": {
                    "file": "/abs/path/db.sqlite"
                }
            },
            "options": {
                "watch": true,
            }
        }
    },
    "log": "info"
}

const example_mysql: Config = {
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "sk_test_abc123"
                }
            },
            "to": {
                "mysql": {
                    "addr": {
                        "ip": "127.0.0.1",
                        "port": 3306
                    },
                    "user": "root",
                    "pass": "my-secret-pw",
                    "schema_name": "stripe_acc_x"
                }
            },
            "options": {
                "watch": true
            }
        }
    },
    "log": "info"
}


const example_postgres: Config = {
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "sk_test_abc123"
                }
            },
            "to": {
                "postgres": {
                    "addr": {
                        "socket": "/var/run/postgresql/.s.PGSQL.5432."
                    },
                    "user": "root",
                    "pass": "my-secret-pw",
                    "schema_name": "stripe_acc_x"
                }
            },
            "options": {
                "watch": true
            }
        }
    },
    "log": "info"
}


// Outputs every possible log line.
const example_full_logs: Config = {
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "sk_test_abc123"
                }
            },
            "to": {
                "sqlite": {
                    "file": "/abs/path/db.sqlite"
                }
            },
            "options": {
                "watch": true
            }
        }
    },
    "log": {
        "level": "trace",
        "groups": {
            "tdog": "trace",
            "tdog_lib": "trace",
            "dep": "trace"
        },
        "friendly_mod_names": false
    }
}