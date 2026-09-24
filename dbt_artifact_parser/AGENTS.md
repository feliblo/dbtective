# dbt_artifact_parser guide

This crate turns dbt artifacts into Rust types: `manifest.json` and `catalog.json` (dbt v1/v2), and the dbt v2 Parquet index (`target/index/*.parquet`).
It has no dbtective rule logic. Rule traits are implemented in the root crate (`src/core/manifest/*_impls.rs`, `src/core/catalog/*_impls.rs`).
Read the root [AGENTS.md](../AGENTS.md) first.

Features: `parquet` (enabled by dbtective) pulls in `parquet` with `default-features = false` (`snap`, `zstd` only, **no arrow**, on purpose to keep the binary small). `test-helpers` exposes `test_writer` to the root crate's tests.

## Adding a field a rule needs

1. **JSON types**: add the field to the struct under `src/manifest/` or `src/catalog/`. Make it `Option<T>` and/or `#[serde(default)]`, because dbt versions differ and older manifests must keep parsing. Never make a field required unless every supported dbt version writes it.
2. **Parquet mapping**: if the index has the column, read it in the row mapper (`parquet/nodes.rs`, `objects.rs`, `columns.rs`, `catalog.rs`) with the `IndexRow` helpers (`non_empty_str` for optional strings, `json` for nested blobs) and map it onto the same struct field.
   If the index **doesn't** have it (currently `raw_code`, `loader`, declared columns, the macro list and UDFs), extend `JsonSupplement` in `parquet/manifest.rs` so it's recovered from `manifest.json` when available. Deserialize only the keys you need. Then update the "Not in the index" table in `docs/content/docs/running/artifact-formats.md`.
3. **Expose it to rules** through a trait impl in the root crate (`src/core/manifest/*_impls.rs`).
4. **Test both paths**:
   - JSON: a parser unit test and/or a dbtective integration test with an inline manifest (see [tests/AGENTS.md](../tests/AGENTS.md)).
   - Parquet: a unit test in the relevant `parquet/*.rs` module using `test_writer`, and a case in `tests/test_parquet_index.rs` if the field reaches a rule.
   - End to end: `just diff-artifacts` must show **no diff** between findings from `manifest_v2.json` and from the Parquet index of the same dbt run in `dbt_project/target/`.

## Conventions

- Same lint bar as the root crate (pedantic/nursery/cargo deny). Public fns returning `Result` need a `# Errors` doc section.
- Errors are `anyhow`. Parse errors should name the file and, via `serde_path_to_error`, the JSON path of the bad field. Missing-index errors should say how to regenerate the index (see `IndexLayout::require`).
- Keep parsing tolerant: unknown fields are ignored, absent fields default. dbtective must not crash on a manifest from a newer dbt that adds keys.
- `Manifest::filter_to_project` drops package objects. Keep that behaviour when you add new object collections.
