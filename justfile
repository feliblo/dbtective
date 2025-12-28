

run:
    cargo run run --verbose --entry-point ./dbt_project

run-verbose:
    cargo run run --verbose --entry-point ./dbt_project

run-release:
    cargo run --release run --verbose --entry-point ./dbt_project

install:
    cargo install --path .

fmt:
    cargo fmt

test:
    cargo test

lint:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

# Needs Hugo and Go installed
[working-directory: 'docs']
setup-docs:
    hugo mod tidy

docs:
    hugo server --logLevel debug --disableFastRender --baseURL http://localhost:1313/ -p 1313 -s docs


bump:
    cz bump --increment PATCH
