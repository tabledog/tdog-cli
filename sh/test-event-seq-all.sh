#!/usr/bin/env bash
source ./sh/set-rust-env-flags.sh;
cargo test event_seq_ --package lib_app -- --nocapture --test-threads=1;
#cargo watch --ignore ".idea/*" -s "clear && printf '\e[3J'; cargo test event_seq_ --package lib-app --color always -- --nocapture --test-threads=1 2>&1  | head -50000"
