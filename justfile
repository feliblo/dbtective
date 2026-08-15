
[working-directory: './dbt_project']
init:
    cargo run init
run:
    cargo run run --verbose --entry-point ./dbt_project

run-verbose:
    cargo run run --verbose --entry-point ./dbt_project

run-release:
    cargo run --release run --verbose --entry-point ./dbt_project

run-release-manifest:
    cargo run --release run --verbose --entry-point ./dbt_project --only-manifest

# Run against the dbt v2 Parquet index
run-parquet:
    cargo run run --verbose --entry-point ./dbt_project --artifact-format parquet

run-release-parquet:
    cargo run --release run --entry-point ./dbt_project --artifact-format parquet --verbose

# Run against the JSON artifacts, ignoring the Parquet index
run-json:
    cargo run run --verbose --entry-point ./dbt_project --artifact-format json

run-release-json:
    cargo run --release run --verbose --entry-point ./dbt_project --artifact-format json

# Like-for-like: the v2 JSON manifest vs the Parquet index of the SAME dbt run.
# dbtective exits non-zero when it finds issues, hence the `|| true`.
diff-artifacts:
    -cargo run -q run --entry-point ./dbt_project --only-manifest --artifact-format json \
        --manifest-file target/manifest_v2.json --output-format ndjson --output-file /tmp/dbtective-json.ndjson
    -cargo run -q run --entry-point ./dbt_project --only-manifest --artifact-format parquet \
        --output-format ndjson --output-file /tmp/dbtective-parquet.ndjson
    sort /tmp/dbtective-json.ndjson -o /tmp/dbtective-json.ndjson
    sort /tmp/dbtective-parquet.ndjson -o /tmp/dbtective-parquet.ndjson
    -diff /tmp/dbtective-json.ndjson /tmp/dbtective-parquet.ndjson

install:
    cargo install --path .

fmt:
    cargo fmt

test:
    cargo test

test-cov:
    rustup run stable cargo llvm-cov test --workspace --all-features

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
