#!/usr/bin/env bash
# Usage in terminal: `source ./sh/set-rust-env-flags.sh`

if [ -z "$SET_RUST_ENV_RAN" ]; then

    # Avoid setting env vars twice which invalidates the Cargo build cache.
   export SET_RUST_ENV_RAN="1";


   # https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html List of `-A` flags to disable warnings.

   # Issue: When running `cargo test`, if the env vars are not exactly the same as before, cargo clears the compile cache and rebuilds taking minutes.
   # Fix: Store the env in this re-usable file for each sh file containing a `cargo test`.
   # Usage: `source x.sh; cargo test ...`, `set;` to print current env.
   export RUST_MIN_STACK=16777216;
   export RUSTFLAGS="$RUSTFLAGS -A dead_code -A warnings -A unused_imports";
   export RUST_LOG=info;
   export RUST_BACKTRACE=1;

fi

