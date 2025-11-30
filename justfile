

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


bump:
    git checkout main
    git pull origin main
    cz bump --increment PATCH
    auto-changelog --hide-credit
    git add CHANGELOG.md
    git commit --amend --no-edit
    git push origin main --tags
    just changelog
