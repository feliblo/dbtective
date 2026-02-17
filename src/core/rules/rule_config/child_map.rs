use crate::{
    cli::table::RuleResult,
    core::{
        config::{check_config_options::OrphanedReferenceType, manifest_rule::ManifestRule},
        manifest::Manifest,
        rules::common_traits::Identifiable,
    },
};

// Models, seeds and sources could/should have child object that consume them.
pub trait ChildMappable: Identifiable {
    fn get_childs<'a>(&self, manifest: &'a Manifest) -> Vec<&'a str>;
}

pub fn is_not_orphaned<T: ChildMappable>(
    tagable: &T,
    rule: &ManifestRule,
    allowed_references: &[OrphanedReferenceType],
    manifest: &Manifest,
) -> Option<RuleResult> {
    let children = tagable.get_childs(manifest);

    // Filter out schema tests (test.*) since they are not meaningful downstream
    // references and are not a selectable OrphanedReferenceType
    let meaningful_children: Vec<&str> = children
        .iter()
        .filter(|c| c.split('.').next().unwrap_or("unknown_object") != "test")
        .copied()
        .collect();

    if meaningful_children.is_empty() {
        let error_msg = format!(
            "{} is orphaned (not referenced by any other object)",
            tagable.get_object_string()
        );

        return Some(RuleResult::new(
            &rule.severity,
            tagable.get_object_type(),
            rule.get_name(),
            error_msg,
            tagable.get_problematic_path(false).map(str::to_owned),
        ));
    }

    // Check if it has at least one allowed reference (given in config)
    let has_allowed_reference = meaningful_children.iter().any(|c| {
        let object_type = c.split('.').next().unwrap_or("unknown_object");
        allowed_references
            .iter()
            .any(|art| art.matches(object_type))
    });

    if has_allowed_reference {
        return None;
    }

    let non_allowed_types: Vec<&str> = meaningful_children
        .iter()
        .filter_map(|c| c.split('.').next())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let error_msg = format!(
        "{} is orphaned (only referenced by: {})",
        tagable.get_object_string(),
        non_allowed_types.join(", ")
    );

    Some(RuleResult::new(
        &rule.severity,
        tagable.get_object_type(),
        rule.get_name(),
        error_msg,
        tagable.get_problematic_path(false).map(str::to_owned),
    ))
}

#[cfg(test)]
mod tests {
    use crate::core::config::check_config_options::{
        default_allowed_references, OrphanedReferenceType,
    };
    use crate::core::config::manifest_rule::{ManifestRule, ManifestSpecificRuleConfig};
    use crate::core::config::severity::Severity;
    use crate::core::manifest::Manifest;
    use crate::core::rules::common_traits::Identifiable;
    use crate::core::rules::rule_config::child_map::{is_not_orphaned, ChildMappable};

    struct MockTaggable {
        object_type: String,
        object_string: String,
        childs: Vec<&'static str>,
    }

    impl Identifiable for MockTaggable {
        fn get_object_type(&self) -> &str {
            &self.object_type
        }
        fn get_object_string(&self) -> &str {
            &self.object_string
        }
        fn get_problematic_path(&self, __prefer_sql: bool) -> Option<&str> {
            None
        }
    }
    impl ChildMappable for MockTaggable {
        fn get_childs<'a>(&self, _manifest: &'a Manifest) -> Vec<&'a str> {
            self.childs.clone()
        }
    }

    #[test]
    fn test_orphaned_no_children() {
        let taggable = MockTaggable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            childs: vec![],
        };

        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: default_allowed_references(),
            },
            Severity::Error,
        );

        let manifest = Manifest::default();
        let allowed_references = vec![OrphanedReferenceType::Models];

        let result = is_not_orphaned(&taggable, &rule, &allowed_references, &manifest);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("is orphaned"));
    }

    #[test]
    fn test_is_not_orphaned_with_allowed_child() {
        let taggable = MockTaggable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            childs: vec!["model.child_model"],
        };

        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: default_allowed_references(),
            },
            Severity::Error,
        );

        let manifest = Manifest::default();
        let allowed_references = vec![OrphanedReferenceType::Models];

        let result = is_not_orphaned(&taggable, &rule, &allowed_references, &manifest);
        assert!(result.is_none());
    }

    #[test]
    fn test_orphaned_non_allowed_object() {
        let taggable = MockTaggable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            childs: vec!["seed.child_model"],
        };

        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: default_allowed_references(),
            },
            Severity::Error,
        );

        let manifest = Manifest::default();
        let allowed_references = vec![OrphanedReferenceType::Exposures];

        let result = is_not_orphaned(&taggable, &rule, &allowed_references, &manifest);
        assert!(result.is_some());
        assert!(result.unwrap().message.contains("only referenced by: seed"));
    }

    #[test]
    fn test_orphaned_only_test_children_treated_as_no_references() {
        let taggable = MockTaggable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            childs: vec!["test.project.not_null_test_model_id"],
        };

        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::IsNotOrphaned {
                allowed_references: default_allowed_references(),
            },
            Severity::Warning,
        );

        let manifest = Manifest::default();
        let allowed_references = vec![OrphanedReferenceType::Exposures];

        let result = is_not_orphaned(&taggable, &rule, &allowed_references, &manifest);
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result
            .message
            .contains("not referenced by any other object"));
        assert_eq!(result.severity, "WARN");
    }
}
