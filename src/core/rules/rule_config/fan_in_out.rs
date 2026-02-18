use crate::{
    cli::table::RuleResult,
    core::{
        config::{check_config_options::DependencyType, manifest_rule::ManifestRule},
        manifest::Manifest,
        rules::{common_traits::Identifiable, rule_config::child_map::ChildMappable},
    },
};

/// Trait for objects that can look up their parents in the manifest's `parent_map`.
pub trait ParentMappable: Identifiable {
    fn get_parents<'a>(&self, manifest: &'a Manifest) -> Vec<&'a str>;
}

/// Check if a node has too many upstream dependencies (parents).
pub fn max_upstream_dependencies<T: ParentMappable>(
    obj: &T,
    rule: &ManifestRule,
    max_upstream: usize,
    exclude_types: &[DependencyType],
    manifest: &Manifest,
) -> Option<RuleResult> {
    let count = obj
        .get_parents(manifest)
        .into_iter()
        .filter(|p| {
            let obj_type = p.split('.').next().unwrap_or("unknown");
            !exclude_types.iter().any(|et| et.matches(obj_type))
        })
        .count();
    if count > max_upstream {
        let error_msg = format!(
            "{} has {} upstream dependencies (max allowed: {})",
            obj.get_object_string(),
            count,
            max_upstream
        );

        return Some(RuleResult::new(
            &rule.severity,
            obj.get_object_type(),
            rule.get_name(),
            error_msg,
            obj.get_problematic_path(false).map(str::to_owned),
        ));
    }

    None
}

/// Check if a node has too many downstream dependents (children).
pub fn max_downstream_dependencies<T: ChildMappable>(
    obj: &T,
    rule: &ManifestRule,
    max_downstream: usize,
    exclude_types: &[DependencyType],
    manifest: &Manifest,
) -> Option<RuleResult> {
    let count = obj
        .get_childs(manifest)
        .into_iter()
        .filter(|c| {
            let obj_type = c.split('.').next().unwrap_or("unknown");
            !exclude_types.iter().any(|et| et.matches(obj_type))
        })
        .count();
    if count > max_downstream {
        let error_msg = format!(
            "{} has {} downstream dependents (max allowed: {})",
            obj.get_object_string(),
            count,
            max_downstream
        );

        return Some(RuleResult::new(
            &rule.severity,
            obj.get_object_type(),
            rule.get_name(),
            error_msg,
            obj.get_problematic_path(false).map(str::to_owned),
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::check_config_options::DependencyType;
    use crate::core::config::manifest_rule::{ManifestRule, ManifestSpecificRuleConfig};
    use crate::core::config::severity::Severity;
    use crate::core::manifest::Manifest;
    use crate::core::rules::common_traits::Identifiable;
    use crate::core::rules::rule_config::child_map::ChildMappable;

    struct MockObject {
        object_type: String,
        object_string: String,
        parents: Vec<&'static str>,
        children: Vec<&'static str>,
    }

    impl Identifiable for MockObject {
        fn get_object_type(&self) -> &str {
            &self.object_type
        }
        fn get_object_string(&self) -> &str {
            &self.object_string
        }
        fn get_problematic_path(&self, _prefer_sql: bool) -> Option<&str> {
            None
        }
    }

    impl ParentMappable for MockObject {
        fn get_parents<'a>(&self, _manifest: &'a Manifest) -> Vec<&'a str> {
            self.parents.clone()
        }
    }

    impl ChildMappable for MockObject {
        fn get_childs<'a>(&self, _manifest: &'a Manifest) -> Vec<&'a str> {
            self.children.clone()
        }
    }

    fn create_upstream_rule(max: usize) -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::MaxUpstreamDependencies {
                max_upstream: max,
                exclude_types: vec![DependencyType::Tests],
            },
            Severity::Error,
        )
    }

    fn create_downstream_rule(max: usize) -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::MaxDownstreamDependencies {
                max_downstream: max,
                exclude_types: vec![DependencyType::Tests],
            },
            Severity::Error,
        )
    }

    // ========================================================================
    // max_upstream_dependencies tests
    // ========================================================================

    #[test]
    fn test_upstream_within_limit() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "orders".to_string(),
            parents: vec!["model.project.stg_orders", "model.project.stg_payments"],
            children: vec![],
        };

        let rule = create_upstream_rule(3);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests];

        let result = max_upstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_upstream_exceeds_limit() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "big_model".to_string(),
            parents: vec![
                "model.project.a",
                "model.project.b",
                "model.project.c",
                "model.project.d",
            ],
            children: vec![],
        };

        let rule = create_upstream_rule(3);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests];

        let result = max_upstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.message.contains("4 upstream dependencies"));
        assert!(result.message.contains("max allowed: 3"));
    }

    #[test]
    fn test_upstream_excludes_tests_by_default() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "tested_model".to_string(),
            parents: vec![
                "model.project.a",
                "model.project.b",
                "test.project.not_null_a_id",
                "test.project.unique_b_id",
            ],
            children: vec![],
        };

        let rule = create_upstream_rule(3);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests];

        // Only 2 non-test parents, within limit of 3
        let result = max_upstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_upstream_no_exclusions() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "tested_model".to_string(),
            parents: vec![
                "model.project.a",
                "model.project.b",
                "test.project.not_null_a_id",
                "test.project.unique_b_id",
            ],
            children: vec![],
        };

        let rule = create_upstream_rule(3);
        let manifest = Manifest::default();
        let exclude: Vec<DependencyType> = vec![];

        // All 4 parents counted, exceeds limit of 3
        let result = max_upstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_some());
    }

    #[test]
    fn test_upstream_exact_limit() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "exact_model".to_string(),
            parents: vec!["model.project.a", "model.project.b", "model.project.c"],
            children: vec![],
        };

        let rule = create_upstream_rule(3);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests];

        // Exactly 3 parents, at limit (not exceeding)
        let result = max_upstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_upstream_no_parents() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "leaf_model".to_string(),
            parents: vec![],
            children: vec![],
        };

        let rule = create_upstream_rule(3);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests];

        let result = max_upstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_upstream_mixed_types_with_exclusions() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "mixed_model".to_string(),
            parents: vec![
                "model.project.a",
                "seed.project.raw_data",
                "source.project.ext.table",
                "test.project.not_null",
                "macro.project.my_macro",
            ],
            children: vec![],
        };

        let rule = create_upstream_rule(2);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests, DependencyType::Macros];

        // 3 counted (model, seed, source), exceeds limit of 2
        let result = max_upstream_dependencies(&obj, &rule, 2, &exclude, &manifest);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("3 upstream dependencies"));
    }

    // ========================================================================
    // max_downstream_dependencies tests
    // ========================================================================

    #[test]
    fn test_downstream_within_limit() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "base_model".to_string(),
            parents: vec![],
            children: vec!["model.project.child_a", "model.project.child_b"],
        };

        let rule = create_downstream_rule(3);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests];

        let result = max_downstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_downstream_exceeds_limit() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "popular_model".to_string(),
            parents: vec![],
            children: vec![
                "model.project.a",
                "model.project.b",
                "model.project.c",
                "model.project.d",
            ],
        };

        let rule = create_downstream_rule(3);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests];

        let result = max_downstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.message.contains("4 downstream dependents"));
        assert!(result.message.contains("max allowed: 3"));
    }

    #[test]
    fn test_downstream_excludes_tests() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "tested_model".to_string(),
            parents: vec![],
            children: vec![
                "model.project.child_a",
                "model.project.child_b",
                "test.project.not_null_test",
                "test.project.unique_test",
            ],
        };

        let rule = create_downstream_rule(3);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests];

        // Only 2 non-test children, within limit of 3
        let result = max_downstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_downstream_no_children() {
        let obj = MockObject {
            object_type: "model".to_string(),
            object_string: "leaf_model".to_string(),
            parents: vec![],
            children: vec![],
        };

        let rule = create_downstream_rule(3);
        let manifest = Manifest::default();
        let exclude = vec![DependencyType::Tests];

        let result = max_downstream_dependencies(&obj, &rule, 3, &exclude, &manifest);
        assert!(result.is_none());
    }
}
