use crate::{
    cli::table::RuleResult,
    core::{config::manifest_rule::ManifestRule, manifest::dbt_objects::Meta},
};

pub trait HasMetadata {
    fn get_metadata(&self) -> Option<&Meta>;
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String>;
}

pub fn has_metadata_keys<T: HasMetadata>(
    has_metadata: &T,
    rule: &ManifestRule,
    required_keys: &[String],
    custom_message: Option<&String>,
) -> Option<RuleResult> {
    has_metadata.get_metadata().map_or_else(
        // No metadata present
        || {
            Some(RuleResult::new(
                &rule.severity,
                HasMetadata::get_object_type(has_metadata),
                rule.get_name(),
                format!(
                    "{} is missing metadata entirely.",
                    HasMetadata::get_object_string(has_metadata)
                ),
                has_metadata.get_relative_path().cloned(),
            ))
        },
        // Metadata present, check for missing keys
        |metadata| {
            let missing_keys = metadata.missing_keys(required_keys);
            if missing_keys.is_empty() {
                None
            } else {
                Some(RuleResult::new(
                    &rule.severity,
                    HasMetadata::get_object_type(has_metadata),
                    rule.get_name(),
                    // Generate message based on whether a custom message is provided
                    custom_message.as_ref().map_or_else(
                        || {
                            format!(
                                "{} is missing required metadata keys: {}.",
                                HasMetadata::get_object_string(has_metadata),
                                missing_keys
                                    .iter()
                                    .map(|s| s.as_str())
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            )
                        },
                        |msg| format!("{} {}", HasMetadata::get_object_string(has_metadata), msg),
                    ),
                    has_metadata.get_relative_path().cloned(),
                ))
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

    use super::*;

    struct TestObject {
        metadata: Option<Meta>,
        object_type: String,
        object_string: String,
        relative_path: Option<String>,
    }
    impl TestObject {
        fn add_test_keys(&mut self, keys: Vec<(&str, &str)>) {
            let mut map = serde_json::Map::new();
            for (k, v) in keys {
                map.insert(k.to_string(), serde_json::Value::String(v.to_string()));
            }
            self.metadata = Some(Meta(serde_json::Value::Object(map)));
        }
    }

    impl HasMetadata for TestObject {
        fn get_metadata(&self) -> Option<&Meta> {
            self.metadata.as_ref()
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
    fn test_no_metadata() {
        let test_object = TestObject {
            metadata: None,
            object_type: "test_type".to_string(),
            object_string: "Test Object".to_string(),
            relative_path: Some("path/to/object".to_string()),
        };
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasMetadataKeys {
                required_keys: vec!["key1".to_string(), "key2".to_string()],
                custom_message: None,
            },
            Severity::Warning,
        );
        let result = has_metadata_keys(
            &test_object,
            &rule,
            &["key1".to_string(), "key2".to_string()],
            None,
        );

        assert!(
            result.is_some(),
            "Has no metadata should return a RuleResult."
        );
        assert!(
            result
                .unwrap()
                .message
                .contains("is missing metadata entirely."),
            "Message should indicate missing metadata entirely."
        );
    }

    #[test]
    fn test_missing_metadata_keys() {
        let mut test_object = TestObject {
            metadata: None,
            object_type: "test_type".to_string(),
            object_string: "Test Object".to_string(),
            relative_path: Some("path/to/object".to_string()),
        };
        test_object.add_test_keys(vec![("key1", "value1")]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasMetadataKeys {
                required_keys: vec!["key1".to_string(), "key2".to_string()],
                custom_message: None,
            },
            Severity::Warning,
        );
        let result = has_metadata_keys(
            &test_object,
            &rule,
            &["key1".to_string(), "key2".to_string()],
            None,
        );
        assert!(result.is_some(), "Expected missing keys, but got none.");
        let rule_result = result.unwrap();
        assert_eq!(
            rule_result.message,
            "Test Object is missing required metadata keys: key2."
        );
    }

    #[test]
    fn test_all_metadata_keys_present() {
        let mut test_object = TestObject {
            metadata: None,
            object_type: "test_type".to_string(),
            object_string: "Test Object".to_string(),
            relative_path: Some("path/to/object".to_string()),
        };
        test_object.add_test_keys(vec![("key1", "value1"), ("key2", "value2")]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasMetadataKeys {
                required_keys: vec!["key1".to_string(), "key2".to_string()],
                custom_message: None,
            },
            Severity::Warning,
        );
        let result = has_metadata_keys(
            &test_object,
            &rule,
            &["key1".to_string(), "key2".to_string()],
            None,
        );
        assert!(result.is_none(), "Expected no missing keys, but got some.");
    }

    #[test]
    fn test_custom_message() {
        let mut test_object = TestObject {
            metadata: None,
            object_type: "test_type".to_string(),
            object_string: "Test Object".to_string(),
            relative_path: Some("path/to/object".to_string()),
        };
        test_object.add_test_keys(vec![("key1", "value1")]);
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasMetadataKeys {
                required_keys: vec!["key1".to_string(), "key2".to_string()],
                custom_message: Some("Custom missing keys message.".to_string()),
            },
            Severity::Warning,
        );
        let result = has_metadata_keys(
            &test_object,
            &rule,
            &["key1".to_string(), "key2".to_string()],
            Some(&"Custom missing keys message.".to_string()),
        );
        assert!(result.is_some(), "Expected missing keys, but got none.");
        let rule_result = result.unwrap();
        assert_eq!(
            rule_result.message,
            "Test Object Custom missing keys message."
        );
    }
}
