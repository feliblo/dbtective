# AGENTS.md

Guidance for AI coding agents (and humans) working on **dbtective**, a Rust CLI that lints dbt metadata.
It reads dbt artifacts (`manifest.json` / `catalog.json`, or the dbt v2 Parquet index in `target/index/*.parquet`),
applies user-configured rules from `dbtective.yml` / `dbtective.toml` / `pyproject.toml [tool.dbtective]`,
prints a table or JSON/CSV/NDJSON, and exits `1` when any `error`-severity finding exists.

It is **open source** (MIT, [github.com/feliblo/dbtective](https://github.com/feliblo/dbtective)). Everything you write (code, commit messages, PR descriptions, comments) is public and read by maintainers and users who weren't part of your session. Write it so they can follow it.

## Task routing — read the scoped guide before you start

| You are…                                                     | Read first                                                         |
| ------------------------------------------------------------ | ------------------------------------------------------------------ |
| Adding or changing a rule (manifest or catalog)              | [src/core/rules/AGENTS.md](src/core/rules/AGENTS.md)               |
| Writing unit or integration tests                            | [tests/AGENTS.md](tests/AGENTS.md)                                 |
| Updating documentation (rule page, rules table, other pages) | [docs/AGENTS.md](docs/AGENTS.md)                                   |
| Parsing a new dbt field, JSON/Parquet artifacts              | [dbt_artifact_parser/AGENTS.md](dbt_artifact_parser/AGENTS.md)     |

A new rule touches all four. Do not skip the docs or the tests — a rule is not done until both exist.

## How a run works

1. `resolve_config_path` → `Config::from_file` → `clean_config` (fill default `applies_to`, normalise includes/excludes) → `validate` (every `applies_to` target must be in the rule's allowed options).
2. `artifacts::resolve` picks JSON or Parquet; `load_manifest`.
3. Manifest rules: `apply_manifest_node_rules` (everything in `manifest.nodes`) and `apply_manifest_object_rules` (sources, macros, exposures, semantic models, unit tests, functions).
4. Catalog rules: with a catalog → `apply_catalog_node_rules` / `apply_catalog_source_rules`; with `--only-manifest` → the `apply_catalog_fallback_*` dispatchers, only for rules where `supports_manifest_fallback()` is `true`.
5. Output (table or structured) and exit code.

Each dispatcher, per (object, rule): `should_run_test` (includes/excludes) → `applies_to` → `model_materializations` → exhaustive `match` on the rule variant → rule fn returns `Option<RuleResult>` (or `Vec<RuleResult>` for multi-finding rules) → dispatcher sets `category` and pairs it with the severity.

## Commands

Run everything from the repo root through the [justfile](justfile).

| Command               | What it does                                                                                   |
| --------------------- | ---------------------------------------------------------------------------------------------- |
| `just fmt`            | `cargo fmt`                                                                                    |
| `just lint`           | `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` (same as CI)   |
| `just test`           | `cargo test` (root crate; see [tests/AGENTS.md](tests/AGENTS.md) for filters and the parser crate) |
| `just run`            | Run dbtective against `dbt_project/`. A non-zero exit means findings, which is expected.        |
| `just run-parquet`    | Same, forcing the Parquet index (`just run-json` forces JSON)                                   |
| `just diff-artifacts` | Findings from JSON vs Parquet for the same dbt run. **Always exits 0**, so read the output: any diff lines are a failure. |

### Quality gate — run before you hand off

```bash
just fmt && just lint && just test && prek run --all-files
```

All four must pass with zero warnings. `prek` runs `.pre-commit-config.yaml` (YAML/whitespace checks, fmt, check, clippy). Its clippy hook skips `--all-targets`, so it doesn't replace `just lint`.
CI also runs `cargo shear` (unused dependencies) and runs the tests on **Ubuntu and Windows**. If you touched `Cargo.toml`, run `cargo shear` if it's installed.
If a tool isn't installed (`cargo-shear`, Hugo), don't install it. Say in your handoff what you couldn't run.

## Rust practices in this repo

`clippy::{all, pedantic, nursery, cargo}` and rust `warnings` are **deny**. The workspace allows only `multiple_crate_versions`, `struct_field_names`, `module_name_repetitions` and `must_use_candidate`, so don't sprinkle `#[must_use]`.

- **Fix lints; don't silence them.** Never loosen `[lints]` in `Cargo.toml` and never add crate-level `#![allow]`. Use an item-level `#[allow(...)]` only when the lint is genuinely wrong for that item. The usual ones are `clippy::too_many_lines` on dispatchers, big builders and big test fixtures; `clippy::unnecessary_literal_bound` on test trait impls; and `dead_code` on serde-only fields.
- Lints that come up often: a `# Errors` doc section on pub fns returning `Result`; `const fn` where possible; inline format args (`format!("{name}")`); `&str`/`&[T]`/`Option<&T>` params instead of `&String`/`&Vec<T>`/`&Option<T>`; `map_or_else`/`is_some_and` over `map().unwrap_or()`; backticks around identifiers in doc comments.
- **Exhaustive matches are the checklist.** Never use `_ =>` on `ManifestSpecificRuleConfig` / `CatalogSpecificRuleConfig`. If a rule doesn't apply to an object type, list its variant in the `=> return Ok(acc)` arm. Then adding a variant causes a compile error at every place that needs a decision.
- **`#[remain::sorted]`**: both rule enums must keep their variants in alphabetical order, or the build fails.
- **Rule logic is generic over traits.** Rule fns take `<T: SomeTrait>` where `SomeTrait: Identifiable`, never concrete parser types. That lets unit tests use small mock structs and lets catalog rules reuse manifest objects in fallback mode.
- **Borrow, don't clone.** Rule fns take `&T`, `&[String]`, `Option<&str>`; dispatchers pass `.as_deref()` / `.as_ref()`; trait getters return `Option<&str>` / `Option<&String>`.
- **Errors**: use `anyhow::Result` with `.context(...)`, and write messages that tell the user how to fix the problem. Rule fns report *findings*, not errors; return `Result` only for invalid config (e.g. a bad regex). Only the top of the run path (`run.rs`, and `resolve_config_path` which it calls) exits via `unwrap_or_exit`; `init` prints the error and returns `1` itself. Everything else, including rules and dispatchers, propagates with `?`.
- **No panics on user input.** Outside tests, never `unwrap()`/`expect()`/`panic!` on anything derived from config or artifacts. A newer dbt can write values we don't know, and that must not crash the run. `expect("why this cannot fail")` is fine only for true invariants, like compiling a hard-coded regex. The `unreachable!` arms in `src/core/manifest/node_impls.rs` are safe only because `applies_to_options_for_*_rule` never lets those node types reach the rule, so keep the two in sync.
- **Config must stay backwards compatible.** New rule options get `#[serde(default)]` or `#[serde(default = "default_fn")]`, with the default fns in `check_config_options.rs`. Options use `snake_case`.
- **Mind case and platform.** Warehouses change identifier casing (Snowflake uppercases), so compare dbt names and columns case-insensitively where it matters. Tests run on Windows: build paths with `Path`/`PathBuf`, and remember manifest paths use `/`.
- **Output**: coloured output (`owo-colors`) belongs in the CLI/run layer. Inside rules, use `log::debug!` for diagnostics, never `println!`.
- **Style**: match the surrounding code. Comments explain *why*. Keep `pub` visibility on modules that integration tests need.
- `PartialEq`/`Hash` on the rule enums compare **only the variant**, not field values (the init flow relies on this). Never use `==` to compare rule options.

## Stop and ask before you

- add or upgrade a dependency (the binary size is watched in CI; `parquet` is built without arrow on purpose). If `Cargo.toml` changes, commit `Cargo.lock` too, because clippy runs with `--locked`.
- rename a rule `type` or option, or change a default in a way that alters results for existing configs.
- regenerate `dbt_project/target/*` (committed fixtures that tests read; needs a local dbt install) or edit `dbt_project/dbtective.yml`.
- change `[lints]`, CI workflows, or the `mod cli; mod core;` layout in `main.rs`. It makes unit tests run twice (lib and bin), which is known and accepted.

Never bump versions (`just bump` / `cz bump`), edit `CHANGELOG.md`, or change version strings in README/docs. Maintainers do that at release time.

## Branches, commits and pull requests

Every change reaches `main` through a pull request. Never commit or push to `main` directly.

- **Branch** off `main`: `feature/…`, `fix/…`, `docs/…`.
- **Commits** follow Conventional Commits, enforced by the `commit-msg` hook: `type(scope): subject`, e.g. `feat(rules): add node_dependency rule`, `fix(check): make columns_have_descriptions case-insensitive`. Common scopes: `rules`, `check`, `init`, `config`, `cli`, `dbtv2`, `udf`, `tests`, `docs`, `table`, `ci`. Keep commits focused, one logical change each.
- **One PR per concern.** Don't bundle a refactor or an unrelated fix with a feature.
- **The PR description is the design record.** Reviewers and future contributors should understand the change without reading your session. Fill in [.github/PULL_REQUEST_TEMPLATE.md](.github/PULL_REQUEST_TEMPLATE.md) and cover:
  - **What and why**: `Closes: #…`, the problem, and what the PR changes.
  - **Architectural decisions**: where the logic lives and why (new trait vs. reused one, manifest vs. catalog rule, new option vs. new rule, parser change vs. rule change), the alternatives you considered and why you rejected them, and any trade-offs (performance, binary size, JSON/Parquet parity).
  - **User-visible impact**: new rules or options with their defaults, changed findings for existing configs, anything breaking. If nothing changes for users, say so.
  - **Examples**: a sample config plus the resulting output (from `just run` or a test).
  - **Testing**: which unit and integration tests you added, what you ran, and anything you couldn't run (e.g. Hugo, `cargo-shear`).
  - **Open questions / follow-ups** for the maintainer, and unrelated issues you noticed but didn't fix.
- Tick the template's checklist only for items you actually did.

## Definition of done

- [ ] Quality gate passes.
- [ ] New behaviour has unit tests (in the source file) **and** integration tests (in `tests/`). A bug fix has a regression test that fails without the fix.
- [ ] Docs updated per [docs/AGENTS.md](docs/AGENTS.md).
- [ ] `just run` output makes sense. If you touched artifact fields, `just diff-artifacts` prints no diff.
- [ ] No unrelated changes. If you spot an unrelated bug, report it in your handoff rather than fixing it.
- [ ] Work is on a branch with a PR (or a ready-to-paste PR description) that covers the points above. The PR description doubles as your handoff.
