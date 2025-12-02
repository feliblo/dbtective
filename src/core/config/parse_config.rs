use crate::core::config::manifest_rule::ManifestRule;
use crate::core::config::{
    catalog_rule::default_applies_to_for_catalog_rule, catalog_rule::CatalogRule,
    manifest_rule::default_applies_to_for_manifest_rule,
};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub manifest_tests: Option<Vec<ManifestRule>>,
    pub catalog_tests: Option<Vec<CatalogRule>>,
}

impl Config {
    /// Load and parse the configuration from a YAML file
    /// # Errors
    /// Returns an error if the file cannot be opened or if the YAML is invalid
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let file = File::open(path)
            .context(format!("Unable to open config file at {}", path.display()))?;

        let config: std::result::Result<Self, serde_yaml::Error> = serde_yaml::from_reader(file);

        match config {
            Ok(mut cfg) => {
                cfg.clean_config();
                cfg.validate()?;
                Ok(cfg)
            }
            Err(err) => Err(anyhow::anyhow!("Error parsing config file: {err}")),
        }
    }

    // 1. Apply default applies_to if not specified
    // 2. Normalize the includes/excludes paths
    pub fn clean_config(&mut self) {
        if let Some(rules) = &mut self.manifest_tests {
            for rule in rules {
                if rule.applies_to.is_none() {
                    rule.applies_to = Some(default_applies_to_for_manifest_rule(&rule.rule));
                }
                rule.normalize_includes_excludes();
            }
        }
        if let Some(rules) = &mut self.catalog_tests {
            for rule in rules {
                if rule.applies_to.is_none() {
                    rule.applies_to = Some(default_applies_to_for_catalog_rule(&rule.rule));
                }
                rule.normalize_includes_excludes();
            }
        }
    }
    // Validate each manifest rule's applies_to targets
    //  # Errors
    // Returns an error if any rule has invalid `applies_to` targets for that specific rule
    ///
    /// # Errors
    /// Returns an error if any rule has invalid `applies_to` targets for that specific rule
    pub fn validate(&self) -> Result<()> {
        if let Some(rules) = &self.manifest_tests {
            for rule in rules {
                rule.validate_applies_to()?;
            }
        }
        if let Some(rules) = &self.catalog_tests {
            for rule in rules {
                rule.validate_applies_to()?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
fn create_temp_file_from_str(content: &str) -> tempfile::NamedTempFile {
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    std::io::Write::write(&mut temp_file, content.as_bytes()).unwrap();
    temp_file
}

#[cfg(test)]
mod tests {

    use crate::core::config::severity::Severity;

    use super::*;

    #[test]
    fn test_parse_config() {
        let simple_rule = r#"
manifest_tests:
  - name: "model_seeds_have_description"
    type: "has_description"
    severity: "error"
    description: "All nodes must have a description."
"#;
        let temp_file = create_temp_file_from_str(simple_rule);
        let config = Config::from_file(temp_file.path()).expect("Failed to parse config");
        let manifest_tests = config
            .manifest_tests
            .expect("in this test manifest_tests should be Some");
        assert_eq!(manifest_tests.len(), 1);
        let rule = &manifest_tests[0];
        assert_eq!(rule.name.as_deref(), Some("model_seeds_have_description"));
        assert_eq!(rule.severity, Severity::Error);
        assert_eq!(rule.name, Some("model_seeds_have_description".to_string()));
    }

    #[test]
    fn test_validate_manifest_test_type() {
        let invalid_rule = r#"
manifest_tests:
   - type: "has_description"
    severity: "error"
"#;
        let temp_file = create_temp_file_from_str(invalid_rule);
        let result = Config::from_file(temp_file.path());
        result.expect_err("Should fail for unknown rule type");
    }

    #[test]
    fn test_valid_applies_to() {
        let valid_rule = r#"
manifest_tests:
  - type: "has_description"
    severity: "error"
    applies_to:
      - "models"
      - "seeds"
"#;
        let temp_file = create_temp_file_from_str(valid_rule);
        let result = Config::from_file(temp_file.path());
        assert!(result.is_ok(), "Should pass for valid applies_to targets");
    }

    #[test]
    fn test_not_available_applies_to() {
        let invalid_rule = r#"
manifest_tests:
    - type: has_description
      severity: "error"
      applies_to: ["hook_nodes"]

"#;
        let temp_file = create_temp_file_from_str(invalid_rule);
        let result = Config::from_file(temp_file.path());
        assert!(
            result.is_err(),
            "Should fail for invalid applies_to for specific rule"
        );
    }

    #[test]
    fn completely_invalid_applies_to() {
        let invalid_rule = r#"
manifest_tests:
    - type: has_description
      severity: "error"
      applies_to: ["invalid_target"]
"#;
        let temp_file = create_temp_file_from_str(invalid_rule);
        let result = Config::from_file(temp_file.path());
        assert!(
            result.is_err(),
            "Should fail for completely invalid applies_to targets"
        );
    }
}
