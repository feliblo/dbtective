use anyhow::Result;
use regex::Regex;

use crate::{
    cli::table::RuleResult,
    core::{
        config::{check_config_options::ParentNodeType, manifest_rule::ManifestRule},
        manifest::Manifest,
        rules::{
            common_traits::Identifiable,
            rule_config::{fan_in_out::ParentMappable, name_convention::NameAble},
        },
    },
};

/// Check if a node has a forbidden dependency matching the given parent filters.
/// Returns one `RuleResult` per matching forbidden parent found.
///
/// # Errors
/// Returns an error if any of the regex patterns are invalid.
pub fn node_dependency<T: NameAble + ParentMappable>(
    obj: &T,
    rule: &ManifestRule,
    from_name_pattern: Option<&str>,
    parent_name_pattern: Option<&str>,
    parent_path_pattern: Option<&str>,
    parent_type: Option<&ParentNodeType>,
    manifest: &Manifest,
) -> Result<Vec<RuleResult>> {
    let from_name_re = from_name_pattern.map(Regex::new).transpose()?;
    let parent_name_re = parent_name_pattern.map(Regex::new).transpose()?;
    let parent_path_re = parent_path_pattern.map(Regex::new).transpose()?;

    if let Some(re) = &from_name_re {
        if !re.is_match(obj.name()) {
            return Ok(vec![]);
        }
    }

    let parents = obj.get_parents(manifest);
    let mut violations = vec![];

    for parent_id in parents {
        let parent_name = parent_id.split('.').next_back().unwrap_or("");
        let parent_type_str = parent_id.split('.').next().unwrap_or("");

        if let Some(pt) = parent_type {
            if !pt.matches(parent_type_str) {
                continue;
            }
        }

        if let Some(re) = &parent_name_re {
            if !re.is_match(parent_name) {
                continue;
            }
        }

        if let Some(re) = &parent_path_re {
            let parent_path = manifest
                .nodes
                .get(parent_id)
                .and_then(|n| n.get_problematic_path(true))
                .or_else(|| {
                    manifest
                        .sources
                        .get(parent_id)
                        .and_then(|s| s.get_problematic_path(false))
                });
            match parent_path {
                Some(path) if re.is_match(path) => {}
                _ => continue,
            }
        }

        violations.push(RuleResult::new(
            &rule.severity,
            obj.get_object_type(),
            rule.get_name(),
            format!(
                "{} has a forbidden dependency on {}",
                obj.get_object_string(),
                parent_name
            ),
            obj.get_problematic_path(true).map(str::to_owned),
        ));
    }

    Ok(violations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        config::{
            check_config_options::ParentNodeType,
            manifest_rule::{ManifestRule, ManifestSpecificRuleConfig},
            severity::Severity,
        },
        manifest::Manifest,
        rules::rule_config::fan_in_out::ParentMappable,
    };

    struct MockModel {
        name: &'static str,
        parents: Vec<&'static str>,
    }

    impl Identifiable for MockModel {
        fn get_object_type(&self) -> &'static str {
            "model"
        }
        fn get_object_string(&self) -> &str {
            self.name
        }
        fn get_problematic_path(&self, _prefer_sql: bool) -> Option<&str> {
            None
        }
    }

    impl NameAble for MockModel {
        fn name(&self) -> &str {
            self.name
        }
    }

    impl ParentMappable for MockModel {
        fn get_parents<'a>(&self, _manifest: &'a Manifest) -> Vec<&'a str> {
            self.parents.clone()
        }
    }

    fn make_rule() -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::NodeDependency {
                from_name_pattern: None,
                parent_name_pattern: None,
                parent_path_pattern: None,
                parent_type: None,
            },
            Severity::Error,
        )
    }

    // ====================================================================
    // from_name_pattern filtering
    // ====================================================================

    #[test]
    fn test_from_name_no_match_skips_node() {
        let node = MockModel {
            name: "fct_orders",
            parents: vec!["model.project.stg_customers"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let results = node_dependency(
            &node,
            &rule,
            Some("^stg_"),
            Some("^stg_"),
            None,
            None,
            &manifest,
        )
        .unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_from_name_match_checks_parents() {
        let node = MockModel {
            name: "stg_orders",
            parents: vec!["model.project.stg_customers"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let results = node_dependency(
            &node,
            &rule,
            Some("^stg_"),
            Some("^stg_"),
            None,
            None,
            &manifest,
        )
        .unwrap();
        assert_eq!(results.len(), 1);
    }

    // ====================================================================
    // parent_name_pattern matching
    // ====================================================================

    #[test]
    fn test_stg_to_stg_forbidden() {
        let node = MockModel {
            name: "stg_orders",
            parents: vec!["model.project.stg_customers", "source.project.raw.orders"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let results =
            node_dependency(&node, &rule, None, Some("^stg_"), None, None, &manifest).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].message.contains("stg_customers"));
    }

    #[test]
    fn test_no_violation_when_parent_does_not_match() {
        let node = MockModel {
            name: "stg_orders",
            parents: vec!["source.project.raw.orders"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let results =
            node_dependency(&node, &rule, None, Some("^stg_"), None, None, &manifest).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_multiple_forbidden_parents_produce_multiple_violations() {
        let node = MockModel {
            name: "fct_orders",
            parents: vec!["model.project.stg_orders", "model.project.stg_customers"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let results =
            node_dependency(&node, &rule, None, Some("^stg_"), None, None, &manifest).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_no_parents_produces_no_violations() {
        let node = MockModel {
            name: "stg_orders",
            parents: vec![],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let results =
            node_dependency(&node, &rule, None, Some("^stg_"), None, None, &manifest).unwrap();
        assert!(results.is_empty());
    }

    // ====================================================================
    // parent_type filtering
    // ====================================================================

    #[test]
    fn test_parent_type_source_catches_direct_source_dep() {
        let node = MockModel {
            name: "int_orders",
            parents: vec!["source.project.raw.orders", "model.project.stg_customers"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let results = node_dependency(
            &node,
            &rule,
            None,
            None,
            None,
            Some(&ParentNodeType::Source),
            &manifest,
        )
        .unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].message.contains("orders"));
    }

    #[test]
    fn test_parent_type_model_filters_only_model_parents() {
        let node = MockModel {
            name: "fct_revenue",
            parents: vec!["model.project.stg_orders", "source.project.raw.payments"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let results = node_dependency(
            &node,
            &rule,
            None,
            None,
            None,
            Some(&ParentNodeType::Model),
            &manifest,
        )
        .unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].message.contains("stg_orders"));
    }

    // ====================================================================
    // Combined filters (AND semantics)
    // ====================================================================

    #[test]
    fn test_from_and_parent_name_both_must_match() {
        let stg_node = MockModel {
            name: "stg_orders",
            parents: vec!["model.project.stg_customers"],
        };
        let int_node = MockModel {
            name: "int_orders",
            parents: vec!["model.project.stg_customers"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let stg_results = node_dependency(
            &stg_node,
            &rule,
            Some("^stg_"),
            Some("^stg_"),
            None,
            None,
            &manifest,
        )
        .unwrap();
        let int_results = node_dependency(
            &int_node,
            &rule,
            Some("^stg_"),
            Some("^stg_"),
            None,
            None,
            &manifest,
        )
        .unwrap();

        assert_eq!(stg_results.len(), 1);
        assert!(int_results.is_empty());
    }

    #[test]
    fn test_parent_name_and_type_both_must_match() {
        let node = MockModel {
            name: "fct_orders",
            // source unique_id last segment happens to start with stg_ but it's a source type
            parents: vec!["model.project.stg_orders", "source.project.raw.stg_fake"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        // parent_name_pattern matches both, but parent_type=model filters to only the model parent
        let results = node_dependency(
            &node,
            &rule,
            None,
            Some("^stg_"),
            None,
            Some(&ParentNodeType::Model),
            &manifest,
        )
        .unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].message.contains("stg_orders"));
    }

    // ====================================================================
    // Error cases
    // ====================================================================

    #[test]
    fn test_invalid_from_name_pattern_returns_error() {
        let node = MockModel {
            name: "stg_orders",
            parents: vec![],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let result = node_dependency(&node, &rule, Some("(*invalid"), None, None, None, &manifest);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_parent_name_pattern_returns_error() {
        let node = MockModel {
            name: "stg_orders",
            parents: vec!["model.project.stg_customers"],
        };
        let rule = make_rule();
        let manifest = Manifest::default();

        let result = node_dependency(&node, &rule, None, Some("(*invalid"), None, None, &manifest);
        assert!(result.is_err());
    }
}
