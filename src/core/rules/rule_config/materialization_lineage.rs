use std::collections::HashSet;

use crate::{
    cli::table::RuleResult,
    core::{
        config::manifest_rule::ManifestRule, manifest::Manifest, rules::common_traits::Identifiable,
    },
};
use dbt_artifact_parser::manifest::{Materialization, Node};

use super::fan_in_out::ParentMappable;
use super::has_refs::CanReference;

/// Check if a model is at the end of a chain of consecutive non-persisted materializations
/// that exceeds the allowed maximum.
///
/// Walks up the DAG via `parent_map`, counting consecutive ancestors whose materialization
/// is in `included_materializations`. The walk stops at non-model parents (sources, seeds, etc.)
/// and at models with a materialization not in the included set.
///
/// When a node has multiple parents, the longest chain is used.
pub fn max_materialization_lineage(
    node: &Node,
    rule: &ManifestRule,
    max: usize,
    included_materializations: &[Materialization],
    manifest: &Manifest,
) -> Option<RuleResult> {
    // Only check models whose materialization is in the included set
    let node_mat = node.get_materialization()?;
    if !included_materializations.contains(node_mat) {
        return None;
    }

    let mut visited = HashSet::new();
    visited.insert(node.get_unique_id().clone());

    // Count = 1 for the current node itself
    let chain_length =
        1 + longest_ancestor_chain(node, manifest, included_materializations, &mut visited);

    if chain_length > max {
        let mat_names: Vec<&str> = included_materializations
            .iter()
            .map(Materialization::as_str)
            .collect();
        let error_msg = format!(
            "{} has a materialization lineage of {} consecutive {} (max: {})",
            node.get_object_string(),
            chain_length,
            mat_names.join("/"),
            max,
        );

        return Some(RuleResult::new(
            &rule.severity,
            node.get_object_type(),
            rule.get_name(),
            error_msg,
            node.get_problematic_path(false).map(str::to_owned),
        ));
    }

    None
}

/// Recursively find the longest chain of consecutive ancestors with matching materializations.
fn longest_ancestor_chain(
    node: &Node,
    manifest: &Manifest,
    included_materializations: &[Materialization],
    visited: &mut HashSet<String>,
) -> usize {
    let mut max_depth = 0;

    for parent_id in node.get_parents(manifest) {
        // Only follow model parents
        if !parent_id.starts_with("model.") {
            continue;
        }

        // Defensive cycle check
        if visited.contains(parent_id) {
            continue;
        }

        let Some(parent_node) = manifest.get_node(parent_id) else {
            continue;
        };

        // Check if parent's materialization is in the included set
        let Some(parent_mat) = parent_node.get_materialization() else {
            continue;
        };

        if !included_materializations.contains(parent_mat) {
            // Chain breaks here — this parent is persisted
            continue;
        }

        // This parent continues the chain; recurse
        visited.insert(parent_id.to_string());
        let depth =
            1 + longest_ancestor_chain(parent_node, manifest, included_materializations, visited);
        if depth > max_depth {
            max_depth = depth;
        }
    }

    max_depth
}

/// Check that every model parent of an exposure uses a persisted materialization.
///
/// Returns one `RuleResult` per violating parent (similar to `has_required_tests`).
pub fn exposure_parents_materialized(
    exposure: &dbt_artifact_parser::manifest::Exposure,
    rule: &ManifestRule,
    allowed_materializations: &[Materialization],
    manifest: &Manifest,
) -> Vec<RuleResult> {
    let mut results = Vec::new();

    for parent_id in exposure.get_depends_on_nodes() {
        // Only check model parents — sources/seeds don't have materializations
        if !parent_id.starts_with("model.") {
            continue;
        }

        let Some(parent_node) = manifest.get_node(parent_id) else {
            continue;
        };

        let Some(parent_mat) = parent_node.get_materialization() else {
            continue;
        };

        if !allowed_materializations.contains(parent_mat) {
            let allowed_names: Vec<&str> = allowed_materializations
                .iter()
                .map(Materialization::as_str)
                .collect();

            let error_msg = format!(
                "exposure '{}' depends on '{}' which is materialized as '{}' (allowed: {})",
                exposure.get_object_string(),
                parent_node.get_object_string(),
                parent_mat.as_str(),
                allowed_names.join(", "),
            );

            results.push(RuleResult::new(
                &rule.severity,
                exposure.get_object_type(),
                rule.get_name(),
                error_msg,
                exposure.get_problematic_path(false).map(str::to_owned),
            ));
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::manifest_rule::{ManifestRule, ManifestSpecificRuleConfig};
    use crate::core::config::severity::Severity;
    use dbt_artifact_parser::manifest::exposure::{Exposure, ExposureDependsOn};
    use std::collections::HashMap;

    fn model_json(name: &str, unique_id: &str, materialization: &str) -> serde_json::Value {
        serde_json::json!({
            "resource_type": "model",
            "name": name,
            "unique_id": unique_id,
            "package_name": "project",
            "path": format!("{name}.sql"),
            "original_file_path": format!("models/{name}.sql"),
            "schema": "public",
            "alias": name,
            "fqn": [name],
            "checksum": {"name": "sha256", "checksum": "abc"},
            "config": {"materialized": materialization},
            "depends_on": {"nodes": [], "macros": []}
        })
    }

    fn make_model_node(name: &str, unique_id: &str, materialization: &str) -> Node {
        serde_json::from_value(model_json(name, unique_id, materialization)).unwrap()
    }

    fn make_manifest(
        nodes: &[(&str, &str, &str)],
        parent_map: HashMap<String, Vec<String>>,
    ) -> Manifest {
        let mut node_map = HashMap::new();
        for (name, unique_id, mat) in nodes {
            let node = make_model_node(name, unique_id, mat);
            node_map.insert((*unique_id).to_string(), node);
        }
        Manifest {
            nodes: node_map,
            parent_map,
            ..Default::default()
        }
    }

    fn lineage_rule(max: usize) -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::MaxMaterializationLineage {
                max,
                included_materializations: vec![Materialization::View, Materialization::Ephemeral],
            },
            Severity::Warning,
        )
    }

    fn exposure_rule() -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::ExposureParentsMaterialized {
                allowed_materializations: vec![
                    Materialization::Table,
                    Materialization::Incremental,
                    Materialization::MaterializedView,
                ],
            },
            Severity::Error,
        )
    }

    fn make_exposure(name: &str, depends_on: Vec<String>) -> Exposure {
        Exposure {
            name: name.to_string(),
            package_name: "project".to_string(),
            original_file_path: format!("models/{name}.yml"),
            patch_path: None,
            description: None,
            meta: None,
            tags: None,
            depends_on: ExposureDependsOn {
                macros: None,
                nodes: Some(depends_on),
            },
        }
    }

    fn included() -> Vec<Materialization> {
        vec![Materialization::View, Materialization::Ephemeral]
    }

    fn allowed() -> Vec<Materialization> {
        vec![
            Materialization::Table,
            Materialization::Incremental,
            Materialization::MaterializedView,
        ]
    }

    // ========================================================================
    // max_materialization_lineage tests
    // ========================================================================

    #[test]
    fn test_table_model_is_skipped() {
        let manifest = make_manifest(
            &[("fct_orders", "model.project.fct_orders", "table")],
            HashMap::new(),
        );
        let node = manifest.get_node("model.project.fct_orders").unwrap();
        let rule = lineage_rule(4);

        let result = max_materialization_lineage(node, &rule, 4, &included(), &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_single_view_within_limit() {
        let manifest = make_manifest(
            &[("stg_orders", "model.project.stg_orders", "view")],
            HashMap::new(),
        );
        let node = manifest.get_node("model.project.stg_orders").unwrap();
        let rule = lineage_rule(4);

        let result = max_materialization_lineage(node, &rule, 4, &included(), &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_chain_exceeds_limit() {
        // source → stg (view) → int (view) → int2 (view) → int3 (view) → fct (view)
        // chain length at fct = 5
        let mut parent_map = HashMap::new();
        parent_map.insert(
            "model.project.stg".into(),
            vec!["source.project.raw".into()],
        );
        parent_map.insert("model.project.int".into(), vec!["model.project.stg".into()]);
        parent_map.insert(
            "model.project.int2".into(),
            vec!["model.project.int".into()],
        );
        parent_map.insert(
            "model.project.int3".into(),
            vec!["model.project.int2".into()],
        );
        parent_map.insert(
            "model.project.fct".into(),
            vec!["model.project.int3".into()],
        );

        let manifest = make_manifest(
            &[
                ("stg", "model.project.stg", "view"),
                ("int", "model.project.int", "view"),
                ("int2", "model.project.int2", "view"),
                ("int3", "model.project.int3", "view"),
                ("fct", "model.project.fct", "view"),
            ],
            parent_map,
        );
        let fct = manifest.get_node("model.project.fct").unwrap();
        let rule = lineage_rule(4);

        let result = max_materialization_lineage(fct, &rule, 4, &included(), &manifest);
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains('5'), "Expected chain length 5, got: {msg}");
        assert!(msg.contains("max: 4"));
    }

    #[test]
    fn test_chain_broken_by_table() {
        // stg (view) → int (table) → int2 (view) → fct (view)
        // chain at fct = 2 (int2 + fct)
        let mut parent_map = HashMap::new();
        parent_map.insert("model.project.int".into(), vec!["model.project.stg".into()]);
        parent_map.insert(
            "model.project.int2".into(),
            vec!["model.project.int".into()],
        );
        parent_map.insert(
            "model.project.fct".into(),
            vec!["model.project.int2".into()],
        );

        let manifest = make_manifest(
            &[
                ("stg", "model.project.stg", "view"),
                ("int", "model.project.int", "table"),
                ("int2", "model.project.int2", "view"),
                ("fct", "model.project.fct", "view"),
            ],
            parent_map,
        );
        let fct = manifest.get_node("model.project.fct").unwrap();
        let rule = lineage_rule(4);

        let result = max_materialization_lineage(fct, &rule, 4, &included(), &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_ephemeral_counts_in_chain() {
        // stg (view) → int (ephemeral) → fct (view)
        // chain at fct = 3
        let mut parent_map = HashMap::new();
        parent_map.insert("model.project.int".into(), vec!["model.project.stg".into()]);
        parent_map.insert("model.project.fct".into(), vec!["model.project.int".into()]);

        let manifest = make_manifest(
            &[
                ("stg", "model.project.stg", "view"),
                ("int", "model.project.int", "ephemeral"),
                ("fct", "model.project.fct", "view"),
            ],
            parent_map,
        );
        let fct = manifest.get_node("model.project.fct").unwrap();
        let rule = lineage_rule(2);

        let result = max_materialization_lineage(fct, &rule, 2, &included(), &manifest);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains('3'));
    }

    #[test]
    fn test_source_parent_does_not_count() {
        // source → stg (view)
        // chain at stg = 1 (just itself)
        let mut parent_map = HashMap::new();
        parent_map.insert(
            "model.project.stg".into(),
            vec!["source.project.raw".into()],
        );

        let manifest = make_manifest(&[("stg", "model.project.stg", "view")], parent_map);
        let stg = manifest.get_node("model.project.stg").unwrap();
        let rule = lineage_rule(4);

        let result = max_materialization_lineage(stg, &rule, 4, &included(), &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_diamond_takes_longest_path() {
        // stg_a (view) → int (view)
        // stg_b (table) → int (view)
        // chain at int = 2 (stg_a path: view→view)
        let mut parent_map = HashMap::new();
        parent_map.insert(
            "model.project.int".into(),
            vec!["model.project.stg_a".into(), "model.project.stg_b".into()],
        );

        let manifest = make_manifest(
            &[
                ("stg_a", "model.project.stg_a", "view"),
                ("stg_b", "model.project.stg_b", "table"),
                ("int", "model.project.int", "view"),
            ],
            parent_map,
        );
        let int_node = manifest.get_node("model.project.int").unwrap();
        let rule = lineage_rule(1);

        let result = max_materialization_lineage(int_node, &rule, 1, &included(), &manifest);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains('2'));
    }

    // ========================================================================
    // exposure_parents_materialized tests
    // ========================================================================

    #[test]
    fn test_exposure_all_table_parents_pass() {
        let exposure = make_exposure("dashboard", vec!["model.project.fct_orders".into()]);
        let manifest = make_manifest(
            &[("fct_orders", "model.project.fct_orders", "table")],
            HashMap::new(),
        );
        let rule = exposure_rule();

        let results = exposure_parents_materialized(&exposure, &rule, &allowed(), &manifest);
        assert!(results.is_empty());
    }

    #[test]
    fn test_exposure_view_parent_fails() {
        let exposure = make_exposure("dashboard", vec!["model.project.fct_orders".into()]);
        let manifest = make_manifest(
            &[("fct_orders", "model.project.fct_orders", "view")],
            HashMap::new(),
        );
        let rule = exposure_rule();

        let results = exposure_parents_materialized(&exposure, &rule, &allowed(), &manifest);
        assert_eq!(results.len(), 1);
        assert!(results[0].message.contains("view"));
        assert!(results[0].message.contains("fct_orders"));
    }

    #[test]
    fn test_exposure_incremental_parent_passes() {
        let exposure = make_exposure("dashboard", vec!["model.project.fct_orders".into()]);
        let manifest = make_manifest(
            &[("fct_orders", "model.project.fct_orders", "incremental")],
            HashMap::new(),
        );
        let rule = exposure_rule();

        let results = exposure_parents_materialized(&exposure, &rule, &allowed(), &manifest);
        assert!(results.is_empty());
    }

    #[test]
    fn test_exposure_source_parent_ignored() {
        let exposure = make_exposure("dashboard", vec!["source.project.raw_orders".into()]);
        let manifest = make_manifest(&[], HashMap::new());
        let rule = exposure_rule();

        let results = exposure_parents_materialized(&exposure, &rule, &allowed(), &manifest);
        assert!(results.is_empty());
    }

    #[test]
    fn test_exposure_multiple_violations() {
        let exposure = make_exposure(
            "dashboard",
            vec![
                "model.project.dim_users".into(),
                "model.project.fct_orders".into(),
            ],
        );
        let manifest = make_manifest(
            &[
                ("dim_users", "model.project.dim_users", "view"),
                ("fct_orders", "model.project.fct_orders", "ephemeral"),
            ],
            HashMap::new(),
        );
        let rule = exposure_rule();

        let results = exposure_parents_materialized(&exposure, &rule, &allowed(), &manifest);
        assert_eq!(results.len(), 2);
    }
}
