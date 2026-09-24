# Testing guide

Every change needs both kinds of test. **Unit tests** check rule logic in isolation; **integration tests** check the full path from config and artifact through the dispatchers to the findings.
Read the root [AGENTS.md](../AGENTS.md) first.

## Running

```bash
just test                                   # cargo test (root crate)
cargo test --workspace --all-features       # + dbt_artifact_parser, incl. Parquet (what CI runs)
cargo test has_loader                       # filter by test name substring
cargo test --test test_catalog_manifest     # all manifest_tests/ + catalog_tests/
cargo test --lib <module_path>              # unit tests only
just test-cov                               # coverage (cargo llvm-cov)
```

CI runs `cargo llvm-cov nextest --workspace --all-features` on Ubuntu **and Windows**, so tests must not assume Unix paths, `/tmp`, or shell tools. (The `run_auto_parse` unit tests in `src/core/run.rs` call `echo`/`false`, which only works because Windows runners ship Git's Unix tools. Don't copy that.) Use `tempfile` for temp dirs, and `#[cfg(target_os = "windows")]` for Windows-only cases, as in `tests/test_includes_excludes.rs`.

Unit tests show up twice in the output (lib and bin targets). That's expected.

## Unit tests — in the source file

Put them at the bottom of the file under test, in `#[cfg(test)] mod tests { use super::*; … }`.

For rule logic, write a **minimal mock struct** that implements `Identifiable` plus the rule's trait, instead of building real parser objects. Copy the `tests` module of [src/core/rules/rule_config/has_loader.rs](../src/core/rules/rule_config/has_loader.rs): it's small and shows the mock, the rule construction and a full `RuleResult` assertion.

- Build rules with `ManifestRule::from_specific_rule(...)` / `CatalogRule::from_specific_rule(...)`. These are `#[cfg(test)]` only.
- Cover: the pass case, the fail case, empty and whitespace-only values, `None`, casing differences, every option and its default, and boundaries (`max`, `max + 1`, `0`).
- Assert the full `RuleResult` (`assert_eq!`) for at least one failure so message regressions get caught. Use `is_some()`/`is_none()` for the rest.
- Name tests `test_<what>_<expected>`, e.g. `test_source_with_empty_loader_fails`.
- Clippy lints test code too (`just lint` uses `--all-targets`), so tests need to be clean.

Config and helper code (`check_config_options.rs`, `applies_to.rs`, `init/config_builder.rs`, …) also has unit tests in its own file. Extend them when you change that code.

## Integration tests — `tests/`

For a new rule, add `tests/manifest_tests/test_<rule>.rs` and `mod test_<rule>;` to `tests/manifest_tests/mod.rs` (catalog: the same under `tests/catalog_tests/`). Keep the `mod` list alphabetical. Both folders compile into **one** test binary, `tests/test_catalog_manifest.rs`, so `cargo test --test test_catalog_manifest test_<rule>` runs just your file. Don't create a new top-level `tests/*.rs` for a rule; each one is a separate binary and slows the build.

### `TestEnvironment` (tests/common/mod.rs)

It writes the artifacts and config to a temp dir and runs the real parsers and dispatchers. Start from [tests/manifest_tests/test_has_loader.rs](manifest_tests/test_has_loader.rs) (manifest) or [tests/catalog_tests/test_columns_have_description.rs](catalog_tests/test_columns_have_description.rs) (catalog).

| Method                                              | Runs                                               |
| --------------------------------------------------- | -------------------------------------------------- |
| `TestEnvironment::new(manifest, config)`            | manifest-only environment                          |
| `TestEnvironment::new_with_catalog(m, catalog, c)`  | with `catalog.json`                                |
| `run_manifest_rules(verbose)`                       | node + object manifest dispatchers                 |
| `run_catalog_rules(verbose)`                        | catalog dispatchers (needs a catalog); returns `anyhow::Result` |
| `run_catalog_fallback_rules(verbose)`               | `--only-manifest` fallback dispatchers             |
| `run_structured_output(verbose)`                    | manifest rules → `StructuredOutput`                |
| `run_and_show_results(verbose)`                     | manifest rules → table + exit code                 |

Findings are `Vec<(RuleResult, Severity)>`. Assert on counts, `rule_name`, `object_type`, `message` contents, severity and `relative_path`.

### Manifest fixtures

- Inline the manifest as a raw JSON string. Copy a minimal manifest from a neighbouring test in the same folder: the `metadata` block (`dbt_schema_version` v12), all top-level keys (`nodes`, `sources`, `macros`, `exposures`, `metrics`, `groups`, `selectors`, `disabled`, `parent_map`, `child_map`, `group_map`, `saved_queries`, `semantic_models`, `unit_tests`), and only the objects the test needs.
- Keep fixtures minimal. Include only the fields the parser requires and the ones the rule reads. If parsing fails, the error names the missing field.
- Graph rules (orphaned, fan-in/out, lineage, node_dependency, unique tests) also need consistent `depends_on`, `parent_map` and `child_map`.
- Long test functions with big fixtures are normal here. Put `#[allow(clippy::too_many_lines)]` on the function when clippy asks.

### What integration tests should cover for a rule

1. It passes on a compliant object and fails on a non-compliant one.
2. The default `applies_to` hits the default targets and skips the rest.
3. An explicit `applies_to` narrows or widens as expected, and an invalid target is rejected at config load. Use `Config::from_file` with a temp file and assert `is_err()`, as in `tests/test_config.rs`.
4. `severity: warning` vs `error`, and a custom `name` shows up as `rule_name`.
5. `includes` / `excludes` (paths, `name:`, `tag:`) and `model_materializations`, if the rule targets models.
6. Each option, including its default when omitted, in YAML (and in TOML if the option has non-trivial syntax).
7. Catalog rules: normal mode with `new_with_catalog`, and fallback in `tests/catalog_tests/test_manifest_fallback.rs` (runs if `supports_manifest_fallback()`, otherwise produces no findings).

### Parquet tests

`tests/test_parquet_index.rs` builds Parquet tables in a temp dir with `dbt_artifact_parser::parquet::test_writer::{TableBuilder, Cell, ColumnKind, IndexBuilder}`, which is available to tests through the `test-helpers` dev-dependency feature. When a rule depends on a field that is mapped from the Parquet index, add a case here too. For end-to-end parity against real artifacts, run `just diff-artifacts`.

## Regression tests for bug fixes

Write the failing test first, at the level where the bug lives (a unit test for rule logic, an integration test for config or dispatch). Then fix the code and confirm the test fails without the fix.
