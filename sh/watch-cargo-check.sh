#!/usr/bin/env bash
source ./sh/set-rust-env-flags.sh;
cargo watch --ignore ".idea/*" -s "clear && printf '\e[3J'; cargo check --all --color always 2>&1  | head -50"
