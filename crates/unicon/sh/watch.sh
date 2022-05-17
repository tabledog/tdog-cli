#!/usr/bin/env bash
set -e;
real=$(realpath "$(dirname "$0")");
workspace=$real/../../..;

# Run from workspace so that Cargo outputs compiler error file paths relative to it (enabling click to open in iterm).
cd $workspace;
source ./sh/set-rust-env-flags.sh;

# Show first few compiler errors (avoid terminal scrolling).
cargo watch --ignore ".idea/*" -s "clear && printf '\e[3J'; cargo test --package unicon --color always 2>&1  | head -5000"