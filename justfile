lint:
    cargo clippy --fix --all-targets

run *arguments:
    cargo run -- {{arguments}}

run-release *arguments:
    cargo run --release -- {{arguments}}

test:
    cargo test