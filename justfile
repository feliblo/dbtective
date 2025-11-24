lint:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

build-run:
    cargo build --release
    just run

run:
    ./target/release/dbtective run --entry-point ./dbt_project

run-verbose:
    ./target/release/dbtective run --entry-point ./dbt_project --verbose

build-run-verbose:
    cargo build --release
    just run-verbose

test:
    cargo test