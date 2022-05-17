#!/usr/bin/env bash
real=$(realpath "$(dirname "$0")");
base=$real/../../..;

source $base/sh/set-rust-env-flags.sh;
cargo build --package tdog_cli;

# Note: assumes that the `main` function will exit after parsing the CLI args (just for testing; return early from main if testing).

# Errors
$base/target/debug/tdog_cli;
$base/target/debug/tdog_cli --json;
$base/target/debug/tdog_cli --json "abc";
$base/target/debug/tdog_cli --json-file;
$base/target/debug/tdog_cli --json-file "123";


# Ok
$base/target/debug/tdog_cli --json \
'{
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "123456789123456789123456789123456789"
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
    }
}';

temp_file=$(mktemp)
echo '{
    "cmd": {
        "fn": "download",
        "args": {
            "from": {
                "stripe": {
                    "secret_key": "123456789123456789123456789123456789"
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
    }
}' > "$temp_file";
$base/target/debug/tdog_cli --json-file "$temp_file";



