rm -rf incremental
mkdir -p incremental
time RUSTC_LOG=trace rustc +nightly slow.rs --crate-type lib -C incremental=incremental -C embed-bitcode=no -C debuginfo=2 --cfg fuzzing -Cpasses=sancov-module -Clink-dead-code -Zsanitizer=address -Cllvm-args=-sanitizer-coverage-trace-compares -C codegen-units=1 -C opt-level=3 -Z self-profile
