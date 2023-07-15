# tdog

`tdog` is a CLI to download your Stripe account to a SQLite database.

- [tabledog.dev](https://tabledog.dev)
- [Sponsor development](https://github.com/sponsors/emadda)


# Install
```bash
# macOS:
curl -L --output /usr/local/bin/tdog https://github.com/tabledog/tdog-cli/releases/latest/download/tdog-x86_64-apple-darwin

# Linux:
#curl -L --output /usr/local/bin/tdog https://github.com/tabledog/tdog-cli/releases/latest/download/tdog-x86_64-unknown-linux-gnu

chmod +x /usr/local/bin/tdog;
```

- [Releases](https://github.com/tabledog/tdog-cli/releases)
	- [macOS](https://github.com/tabledog/tdog-cli/releases/latest/download/tdog-x86_64-apple-darwin)
	- [Linux](https://github.com/tabledog/tdog-cli/releases/latest/download/tdog-x86_64-unknown-linux-gnu)
	- [Windows](https://github.com/tabledog/tdog-cli/releases/latest/download/tdog-x86_64-pc-windows-gnu.exe)

# Usage
```bash
tdog --stripe-key abc --target db.sqlite --watch

# Or use a JSON config:
tdog --json-file /path/config.json;
tdog --json "{}";
```

See the [config JSON schema](notes/config.ts).


# Building
```bash
git clone https://github.com/tabledog/tdog-cli;
cd tdog-cli;
cargo build --package tdog_cli

# Run
./target/debug/tdog_cli
```





# Examples

## Running once vs polling

When `watch` is `true`, the `tdog` process will continue to run and poll the Stripe `/events` endpoint, applying new writes to the database as they occur.

When `watch` is `false`, the `tdog` process will:
- First run: Download all objects and then apply the events, exiting after done.
- Second or greater runs: Apply any events that have occurred since the last run (as long as less that 30 days have passed, which is the limit of Stripes event history). Exit after done.

```json5
// Config path `cmd.args.options`:
{
    "options": {
        "watch": true
    }
}
```

## SQLite


`config.json`
```json
{
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "sk_test_... download at https://dashboard.stripe.com/apikeys"
                }
            },
            "to": {
                "sqlite": {
                    "file": "/absolute/path/db.sqlite"
                }
            },
            "options": {
                "watch": true
            }
        }
    },
    "log": "info"
}
```
```bash
tdog --json-file config.json
 ```




## MySQL

Start MySQL:
```bash
# Note: Version <= 5.6 not supported.

# Version 5.7
docker run --platform linux/x86_64 -e MYSQL_ROOT_PASSWORD=my-secret-pw -p 3306:3306 -d mysql:5.7

# Version 8
docker run --platform linux/x86_64 -e MYSQL_ROOT_PASSWORD=my-secret-pw -p 3306:3306 -d mysql:8
```





`config.json`
```json
{
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "sk_test_... download at https://dashboard.stripe.com/apikeys"
                }
            },
            "to": {
                "mysql": {
                    "addr": {
                        "ip": "127.0.0.1",
                        "port": 3306
                    },
                    "user": "root",
                    "pass": "my-secret-pw"
                }
            },
            "options": {
                "watch": true
            }
        }
    },
    "log": "info"
}
```
```bash
tdog --json-file config.json
 ```



## Postgres


Start Postgres:
```bash
# Version 9.6.23
docker run --platform linux/x86_64 -e POSTGRES_PASSWORD=my-secret-pw -p 5432:5432 -d postgres:9.6.23


# Version 14
docker run --platform linux/x86_64 -e POSTGRES_PASSWORD=my-secret-pw -p 5432:5432 -d postgres:14
```

`config.json`
```json
{
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "sk_test_.. download at https://dashboard.stripe.com/apikeys"
                }
            },
            "to": {
                "postgres": {
                    "addr": {
                        "ip": "127.0.0.1",
                        "port": 5432
                    },
                    "user": "postgres",
                    "pass": "my-secret-pw"
                }
            },
            "options": {
                "watch": true
            }
        }
    },
    "log": "info"
}

```
```bash
tdog --json-file config.json
 ```



## Postgres with TLS (SSL)

`config.json`
```json5
{
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "sk_test_.. download at https://dashboard.stripe.com/apikeys"
                }
            },
            "to": {
                "postgres": {
                    "addr": {
                        "ip": "127.0.0.1",
                        "port": 5432
                    },
                    "user": "postgres",
                    "pass": "my-secret-pw",
                    "tls": {
                        // Certificate Authority certificate.
                        // Equal to `PGPASSWORD=x psql "port=5432 host=x user=postgres sslrootcert=/certs/ca.crt sslmode=verify-full"`
                        "ca_cert_file": "/certs/ca.crt"
                    }
                }
            },
            "options": {
                "watch": true
            }
        }
    },
    "log": "info"
}
```