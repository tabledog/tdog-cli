#!/usr/bin/env bash
# Exit on fail.
set -e;

real=$(realpath "$(dirname "$0")");
base=$real/../../..;
source $base/sh/set-rust-env-flags.sh;

# For long compile reports, lock to top of terminal to fix top compile errors first (iTerm2 -> Right Click -> Terminal State -> Focus Reporting).
cargo watch  --ignore "stripe/event_seq/data/*" --ignore ".idea/*" -s "clear && printf '\e[3J'; cargo test --color always --package tdog_cli -- --nocapture --test-threads=1"