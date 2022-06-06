# Security
"cargo audit": Checks crates for known vulnerabilities
"cargo fuzz": Fuzzing with libFuzzer
"cargo geiger": Finds "unsafe" rust usage
"cargo tarpaulin": Checks the test coverage
"MIRAI": Static analyzer


# Test Coverage
cargo llvm-cov --html
cargo llvm-cov --open
https://crates.io/crates/cargo-llvm-cov#basic-usage


# run single case:
cargo test modify_admin_fee -- --exact
cargo test test_mod_name::test_fn_name -- --exact
modify_admin_fee

# Run example
cargo run --example basic
