

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
setup-docs:
    cd docs
    hugo mod tidy
    cd ..

docs:
    hugo server --logLevel debug --disableFastRender -p 1313 -s docs

changelog:
    auto-changelog --hide-credit

release VERSION:
    git tag "$VERSION"
    git push origin "$VERSION"
