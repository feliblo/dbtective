# Adding or changing a rule

Authoritative checklist for implementing a dbtective rule. Read the root [AGENTS.md](../../../AGENTS.md) first.
`docs/content/docs/contributing/creating_a_new_rule.md` is an older, stale version of this checklist (it names `node_rules.rs`, which doesn't exist). Don't follow it and don't update it unless asked.
Once you add the enum variant, the compiler points you to most of the remaining steps; work through every error rather than silencing it.

## 0. Decide before writing code

- **Manifest or catalog rule?** Manifest rules need only `manifest.json`. Catalog rules compare against real warehouse columns in `catalog.json` (or the Parquet index). If the rule needs no warehouse data, make it a manifest rule.
- **Reuse or new trait?** Manifest traits live next to their rule in `rule_config/*.rs`; `Identifiable` and `Columnable` are in `common_traits.rs`. Reuse a trait that already exposes the data you need (`Descriptable`, `Tagable`, `HasCode`, `HasMetadata`, `CanReference`, `ChildMappable`, `ParentMappable`, `TestAble`, `PathCheckable`, `HasPatchPath`, `ContractAble`, `NameAble`, `Columnable`, …). Extend or reuse before creating a new one.
- **Which objects can it apply to, and what's the default?** Pick the full set of valid `RuleTarget`s and a sensible default subset.
- **Default category**: `Documentation`, `Naming`, `Testing`, `Governance`, `Structure` or `Performance`.
- **Options**: name them in `snake_case`, give each one a default where possible, and keep existing configs valid.
- **Does the data exist in both artifact formats?** If the field is missing from the Parquet index, see [dbt_artifact_parser/AGENTS.md](../../../dbt_artifact_parser/AGENTS.md).

## 1. Config enum — `src/core/config/manifest_rule.rs` (or `catalog_rule.rs`)

1. Add a `PascalCase` variant to `ManifestSpecificRuleConfig` (or `CatalogSpecificRuleConfig`) **in alphabetical position**, because `#[remain::sorted]` fails the build otherwise. Serde/strum turn it into the `snake_case` `type` users write (`HasOwner` → `has_owner`).
2. Options go inside the braces. Use `{}` for no options. Defaulted options use `#[serde(default)]` or `#[serde(default = "default_x")]`, with `default_x` (and any option enums) in `src/core/config/check_config_options.rs`, next to the existing ones and under a `// YourRule` comment.
3. `default_category()`: add the variant to the right arm.
4. `default_applies_to_for_manifest_rule` / `default_applies_to_for_catalog_rule`: default targets.
5. `applies_to_options_for_manifest_rule` / `applies_to_options_for_catalog_rule`: every valid target. The default must be a subset.
6. Catalog only: `supports_manifest_fallback()`. Return `true` if the rule gives meaningful results from manifest data alone, and `false` if it fundamentally needs warehouse columns (like `columns_all_documented`). Add the variant to `test_supports_manifest_fallback` in the same file.

## 2. Rule logic

**Manifest**: new file `src/core/rules/rule_config/<rule_name>.rs`. **Catalog**: `src/core/rules/catalog/<rule_name>.rs`.

```rust
use crate::cli::table::RuleResult;
use crate::core::config::manifest_rule::ManifestRule;
use crate::core::rules::common_traits::Identifiable;

pub trait HasOwner: Identifiable {
    fn owner(&self) -> Option<&str>;
}

pub fn has_owner<T: HasOwner>(obj: &T, rule: &ManifestRule) -> Option<RuleResult> {
    match obj.owner() {
        Some(owner) if !owner.trim().is_empty() => None,
        _ => Some(RuleResult::new(
            &rule.severity,
            obj.get_object_type(),
            rule.get_name(),
            format!("{} has no owner.", obj.get_object_string()),
            obj.get_problematic_path(false).map(str::to_owned),
        )),
    }
}
```

Conventions:
- Return `None` on pass and `Some(RuleResult)` on failure. Return `Vec<RuleResult>` only when one object can fail several times (see `has_required_tests`, `node_dependency`). Return `anyhow::Result<…>` only for invalid config such as a bad regex.
- Don't set `category` in the rule fn; the dispatcher does that.
- Messages start with the object name, say what is wrong, and where useful the expected value. Look at neighbouring rules for tone.
- `get_problematic_path(false)` points at the YAML property file; `true` points at the SQL file. Choose the file the user must edit.
- Rules that need the whole graph (parents/children/tests) take `manifest: &Manifest` as a parameter (see `is_not_orphaned` in `child_map.rs`, `max_upstream_dependencies` in `fan_in_out.rs`, `max_materialization_lineage` in `materialization_lineage.rs`).
- Catalog rule fns that compare warehouse columns with documented columns are generic over **two** `Columnable`s, `<C: Columnable, M: Columnable>(catalog_obj: &C, manifest_obj: &M, rule: &CatalogRule, …)`, so fallback mode can pass the manifest node as both. Rules that only look at one column list take a single `C` (see `columns_canonical_name`).
- Put unit tests at the bottom of the file (see [tests/AGENTS.md](../../../tests/AGENTS.md)).

## 3. Export — `src/core/rules/rule_config/mod.rs` (or `catalog/mod.rs`)

Add `pub mod <rule_name>;` and `pub use <rule_name>::<rule_fn>;`, keeping the lists alphabetical.

## 4. Trait impls — `src/core/manifest/*_impls.rs` (and `src/core/catalog/*_impls.rs`)

Implement your trait for every parser type the rule's **options** allow: `Node` (node_impls.rs), `Source`, `Macro`, `Exposure`, `SemanticModel`, `UnitTest`, `UDF` (function_impls.rs). Model/snapshot/analysis-specific data goes through the `ModelExt` / `SnapshotExt` / `AnalysisExt` helpers in `node_objects_impls.rs`. `Identifiable` is already implemented for all of them.

`Node` is an enum over every node type. If the trait only makes sense for some of them (e.g. models), existing impls end in `_ => unreachable!("… can only be called on models")`. That is safe **only** while `applies_to_options_for_manifest_rule` excludes every other node type. Widening the options later without updating the impl turns a user's config into a panic. Never `expect()`/`unwrap()` on values parsed from the artifact inside an impl; map unknown values to `None`.

## 5. Dispatch

Add a match arm in every dispatcher. The compiler errors on each missing one.

**Manifest rules**
- `src/core/rules/manifest/apply_manifest_node_rules.rs`: the nodes arm.
- `src/core/rules/manifest/apply_other_manifest_object_rules.rs`: one `match` per object type (`apply_source_rules`, `apply_macro_rules`, `apply_exposure_rules`, `apply_semantic_model_rules`, `apply_unit_test_rules`, `apply_function_rules`).
- If the rule doesn't apply to an object type, add the variant to that function's `=> return Ok(acc)` arm. **Don't use `_ =>`.**
- For `Vec`-returning rules, copy the existing pattern: loop, set `rule_row.category = rule.get_category().to_string()`, push `(rule_row, &rule.severity)` onto `acc`, then return `None` from the arm.

**Catalog rules**
- `src/core/rules/catalog/apply_catalog_node_rules.rs` and `apply_catalog_source_rules.rs`.
- `apply_catalog_fallback_node_rules.rs` and `apply_catalog_fallback_source_rules.rs`: call the rule with the manifest object as both arguments, or `return Ok(acc)` if `supports_manifest_fallback()` is `false`.

## 6. `dbtective init` — `src/core/init/`

- `questionnaire.rs`: add the `init_description()` arm, formatted `"<rule_name> - Short description"`. Update `test_manifest_rule_iter_count` / `test_catalog_rule_iter_count` and add the description to `test_*_rule_init_description`. Only add the rule to a `Strictness` preset (Basic/Standard/Strict) if the task asks for it.
- `config_builder.rs`:
  - `create_manifest_rule` / `create_catalog_rule`: sensible generated defaults, which may depend on `result.layering_strategy` / `result.methodology`.
  - `manifest_rule_to_yaml` / `manifest_rule_to_toml` (or the `catalog_rule_to_*` pair): the serialized form. YAML uses two-space list items; TOML uses `[[{section}]]` so the same code serves `dbtective.toml` and `pyproject.toml`.
  - Add a `test_create_manifest_rule_<rule>` unit test.
- `tests/test_init.rs` checks that generated configs parse. Run it.

## 7. Tests

Unit tests at the bottom of the rule file, and an integration test file in `tests/manifest_tests/` or `tests/catalog_tests/` registered in that folder's `mod.rs`. Catalog rules also get a case in `tests/catalog_tests/test_manifest_fallback.rs`. What each must cover: [tests/AGENTS.md](../../../tests/AGENTS.md).

## 8. Docs

Add a rule page (or a section in an existing family page), **and** a row in the rules table in `docs/content/docs/rules/_index.md`. The documented Default/Options must match the `applies_to` functions exactly. Details: [docs/AGENTS.md](../../../docs/AGENTS.md).

## 9. Dogfood

Don't edit `dbt_project/dbtective.yml` unless asked. To see the rule on the jaffle-shop project, put a temporary config inside `dbt_project/` (`--config-file` is resolved relative to `--entry-point`), run `cargo run -- run --entry-point ./dbt_project --config-file <tmp>.yml`, check the findings make sense, and delete the file. If the rule reads artifact fields, also run `just diff-artifacts`.

## Changing an existing rule

- Changing a default or option is user-visible. Keep old configs parsing, and update the rule page, the table row if the description or category changed, the init builder, and the tests in the same change.
- Bug fixes need a regression test that fails without the fix, at unit level and, where the bug showed up through config or dispatch, at integration level too.
- Renaming a rule `type` or option breaks users. Don't, unless explicitly asked.
