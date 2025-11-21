lint:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

build-run:
    cargo build --release
    ./target/release/dbtective run --entry-point ./dbt_project

test:
    cargo test