#!/usr/bin/env bash
set -e;
real=$(realpath "$(dirname "$0")");
cd $real/../;

source $real/../../../sh/set-rust-env-flags.sh;
cargo build;
