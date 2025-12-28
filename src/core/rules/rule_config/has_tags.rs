use crate::{
    cli::table::RuleResult,
    core::{
        config::{check_config_options::HasTagsCriteria, manifest_rule::ManifestRule},
        manifest::dbt_objects::Tags,
    },
};
pub trait Tagable {
    fn get_tags(&self) -> Option<&Tags>;
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}

pub fn check_tags<T: Tagable>(
    tagable: &T,
    rule: &ManifestRule,
    tags: &Tags,
    required_tags: &Vec<String>,
    criteria: &HasTagsCriteria,
) -> Option<RuleResult> {
    let matches = required_tags
        .iter()
        .filter(|tag| tags.contains(tag))
        .count();

    let condition_met = match criteria {
        HasTagsCriteria::All => matches == required_tags.len(),
        HasTagsCriteria::Any => matches > 0,
        HasTagsCriteria::OneOf => matches == 1,
    };

    if condition_met {
        None
    } else {
        let error_msg = match criteria {
            HasTagsCriteria::All => format!(
                "{} does not have all required tags: {:?}",
                tagable.get_object_string(),
                required_tags
            ),
            HasTagsCriteria::Any => format!(
                "{} does not have at least one of the required tags: {:?}",
                tagable.get_object_string(),
                required_tags
            ),
            HasTagsCriteria::OneOf => format!(
                "{} does not have exactly one of the required tags: {:?}",
                tagable.get_object_string(),
                required_tags
            ),
        };
        Some(RuleResult::new(
            &rule.severity,
            tagable.get_object_type(),
            rule.get_name(),
            error_msg,
            tagable.get_relative_path().cloned(),
        ))
    }
}

pub fn has_tags<T: Tagable>(
    tagable: &T,
    rule: &ManifestRule,
    required_tags: &Vec<String>,
    criteria: &HasTagsCriteria,
) -> Option<RuleResult> {
    tagable.get_tags().map_or_else(
        || {
            Some(RuleResult::new(
                &rule.severity,
                tagable.get_object_type(),
                "has_tags",
                format!(
                    "{} does not have any tags but required tags are: {:?} with criteria: {:?}.",
                    tagable.get_object_string(),
                    required_tags,
                    criteria
                ),
                tagable.get_relative_path().cloned(),
            ))
        },
        |tags| check_tags(tagable, rule, tags, required_tags, criteria),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

    struct TestTagable {
        tags: Option<Tags>,
        object_type: String,
        object_string: String,
        relative_path: Option<String>,
    }
    impl Tagable for TestTagable {
        fn get_tags(&self) -> Option<&Tags> {
            self.tags.as_ref()
        }
        fn get_object_type(&self) -> &str {
            &self.object_type
        }
        fn get_object_string(&self) -> &str {
            &self.object_string
        }
        fn get_relative_path(&self) -> Option<&String> {
            self.relative_path.as_ref()
        }
    }
    #[test]
    fn test_has_tags() {
        let tagable = TestTagable {
            tags: Some(Tags::from(vec!["tag1".to_string(), "tag2".to_string()])),
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let required_tags = vec!["tag1".to_string(), "tag3".to_string()];
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasTags {
                criteria: HasTagsCriteria::All,
                required_tags: required_tags.clone(),
            },
            Severity::Warning,
        );
        let criteria = HasTagsCriteria::Any;
        let result = has_tags(&tagable, &rule, &required_tags, &criteria);
        assert!(result.is_none());
        let criteria = HasTagsCriteria::All;
        let result = has_tags(&tagable, &rule, &required_tags, &criteria);
        assert!(result.is_some());
    }

    #[test]
    fn test_has_tags_no_tags() {
        let tagable = TestTagable {
            tags: None,
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let required_tags = vec!["tag1".to_string()];
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasTags {
                criteria: HasTagsCriteria::All,
                required_tags: required_tags.clone(),
            },
            Severity::Warning,
        );
        let criteria = HasTagsCriteria::All;
        let result = has_tags(&tagable, &rule, &required_tags, &criteria);
        assert!(result.is_some());
    }

    #[test]
    fn test_has_tags_one_of() {
        let tagable = TestTagable {
            tags: Some(Tags::from(vec!["tag1".to_string(), "tag2".to_string()])),
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
        };
        let required_tags = vec!["tag2".to_string(), "tag3".to_string()];
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasTags {
                criteria: HasTagsCriteria::OneOf,
                required_tags: required_tags.clone(),
            },
            Severity::Warning,
        );
        let criteria = HasTagsCriteria::OneOf;
        let result = has_tags(&tagable, &rule, &required_tags, &criteria);
        assert!(result.is_none());

        let required_tags = vec!["tag3".to_string(), "tag4".to_string()];
        let result = has_tags(&tagable, &rule, &required_tags, &criteria);
        assert!(result.is_some());

        let required_tags = vec!["tag1".to_string(), "tag2".to_string()];
        let result = has_tags(&tagable, &rule, &required_tags, &criteria);
        assert!(result.is_some());
    }
}
