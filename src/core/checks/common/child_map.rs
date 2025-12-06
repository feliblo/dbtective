use crate::{
    cli::table::RuleResult,
    core::{
        config::{check_config_options::OrphanedReferenceType, manifest_rule::ManifestRule},
        manifest::Manifest,
    },
};

// Models, seeds and sources could/should have child object that consume them.
pub trait ChildMappable {
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
    fn get_childs<'a>(&self, manifest: &'a Manifest) -> Vec<&'a str>;
}

pub fn is_not_orphaned<T: ChildMappable>(
    tagable: &T,
    rule: &ManifestRule,
    allowed_references: &[OrphanedReferenceType],
    manifest: &Manifest,
) -> Option<RuleResult> {
    let children = tagable.get_childs(manifest);

    if children.is_empty() {
        let error_msg = format!(
            "{} is orphaned (not referenced by any other object)",
            tagable.get_object_string()
        );

        return Some(RuleResult::new(
            &rule.severity,
            tagable.get_object_type(),
            rule.get_name(),
            error_msg,
            tagable.get_relative_path().cloned(),
        ));
    }

    // Check if it has at least one allowed reference (given in config)
    let has_allowed_reference = children.iter().any(|c| {
        let object_type = c.split('.').next().unwrap_or("unknown_object");
        allowed_references.iter().any(|art| {
            println!(
                "Checking if child object_type '{}' matches allowed reference type '{}'",
                object_type,
                art.as_ref()
            );

            art.matches(object_type)
        })
    });

    if has_allowed_reference {
        return None;
    }

    let error_msg = format!(
        "{} is orphaned (only referenced by non-allowed objects)",
        tagable.get_object_string()
    );

    Some(RuleResult::new(
        &rule.severity,
        tagable.get_object_type(),
        rule.get_name(),
        error_msg,
        tagable.get_relative_path().cloned(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::core::checks::common::child_map::{is_not_orphaned, ChildMappable};
    use crate::core::config::check_config_options::{
        default_allowed_references, OrphanedReferenceType,
    };
    use crate::core::config::manifest_rule::ManifestRule;
    use crate::core::config::manifest_rule::ManifestSpecificRuleConfig::IsNotOrphaned;
    use crate::core::config::severity::Severity;
    use crate::core::manifest::Manifest;

    struct MockTaggable {
        object_type: String,
        object_string: String,
        childs: Vec<&'static str>,
    }

    impl ChildMappable for MockTaggable {
        fn get_object_type(&self) -> &str {
            &self.object_type
        }
        fn get_object_string(&self) -> &str {
            &self.object_string
        }

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

        let rule = ManifestRule {
            name: Some("is_not_orphaned".to_string()),
            severity: Severity::Error,
            description: None,
            applies_to: None,
            includes: None,
            excludes: None,
            rule: IsNotOrphaned {
                allowed_references: default_allowed_references(),
            },
        };

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

        let rule = ManifestRule {
            name: Some("is_not_orphaned".to_string()),
            severity: Severity::Error,
            description: None,
            applies_to: None,
            includes: None,
            excludes: None,
            rule: IsNotOrphaned {
                allowed_references: default_allowed_references(),
            },
        };

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

        let rule = ManifestRule {
            name: Some("is_not_orphaned".to_string()),
            severity: Severity::Error,
            description: None,
            applies_to: None,
            includes: None,
            excludes: None,
            rule: IsNotOrphaned {
                allowed_references: default_allowed_references(),
            },
        };

        let manifest = Manifest::default();
        let allowed_references = vec![OrphanedReferenceType::Exposures];

        let result = is_not_orphaned(&taggable, &rule, &allowed_references, &manifest);
        assert!(result.is_some());
        assert!(result
            .unwrap()
            .message
            .contains("only referenced by non-allowed objects"));
    }
}
