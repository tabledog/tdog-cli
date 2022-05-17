# For use when debugging macros.
# Note: nightly must be used.
export RUSTFLAGS="-Z macro-backtrace";
cargo +nightly test --package sql_macro_test  -- --nocapture;
export RUSTFLAGS="";

#cargo expand >| del/del-expand.rs