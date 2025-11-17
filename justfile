lint:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

run *arguments:
    cargo run -- {{arguments}}

run-release *arguments:
    cargo run --release -- {{arguments}}

test:
    cargo test