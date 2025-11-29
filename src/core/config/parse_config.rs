use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

use crate::core::config::applies_to::{
    applies_to_options_for_rule, default_applies_to_for_rule, AppliesTo,
};
use crate::core::config::severity::Severity;

#[allow(dead_code)]
#[derive(Debug, Deserialize, EnumIter, AsRefStr, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type")]
pub enum SpecificRuleConfig {
    #[serde(rename = "has_description")]
    HasDescription {},
    #[serde(rename = "name_convention")]
    NameConvention { pattern: String },
}

impl SpecificRuleConfig {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
    pub fn get_all_variants() -> Vec<Self> {
        Self::iter().collect()
    }
    pub fn get_all_as_str() -> Vec<String> {
        Self::iter().map(|rule| rule.as_ref().to_string()).collect()
    }
}

const fn default_severity() -> Severity {
    Severity::Error
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ManifestRule {
    pub name: Option<String>,
    #[serde(default = "default_severity")]
    pub severity: Severity,
    pub description: Option<String>,
    pub applies_to: Option<AppliesTo>,
    pub includes: Option<Vec<String>>,
    pub excludes: Option<Vec<String>>,
    #[serde(flatten)]
    pub rule: SpecificRuleConfig,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub enum NodeRuleTarget {
    Models,
    Seeds,
    Sources,
    Macros,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub enum RootManifestRuleTarget {
    Tests,
    Snapshots,
}

impl ManifestRule {
    pub fn get_name(&self) -> String {
        self.name
            .as_ref()
            .map_or_else(|| self.rule.as_str().to_string(), Clone::clone)
    }

    pub fn normalize_includes_excludes(&mut self) {
        self.includes = self.includes.take().map(|v| {
            v.into_iter()
                .map(|s| {
                    s.trim_start_matches("./")
                        .trim_start_matches('/')
                        .to_string()
                })
                .collect()
        });
        self.excludes = self.excludes.take().map(|v| {
            v.into_iter()
                .map(|s| {
                    s.trim_start_matches("./")
                        .trim_start_matches('/')
                        .to_string()
                })
                .collect()
        });
    }

    /// Validate that the `applies_to` targets are valid for the specific rule
    /// # Errors
    /// Returns an error if any target in `applies_to` is not valid for the rule type
    pub fn validate_applies_to(&self) -> Result<()> {
        let options = applies_to_options_for_rule(&self.rule);
        let mut invalid_targets = Vec::new();
        let applies_to = self
            .applies_to
            .as_ref()
            .context("applies_to must be set before validation, so this should never happen")?;

        let pairs = [
            (&applies_to.node_objects, &options.node_objects),
            (&applies_to.source_objects, &options.source_objects),
            (&applies_to.test_objects, &options.test_objects),
            (&applies_to.macro_objects, &options.macro_objects),
            (&applies_to.exposure_objects, &options.exposure_objects),
        ];

        for (targets, valid) in pairs {
            for target in targets {
                if !valid.contains(target) {
                    invalid_targets.push(target.as_snake_case());
                }
            }
        }

        if !invalid_targets.is_empty() {
            let valid_options: Vec<String> = pairs
                .iter()
                .flat_map(|(_, valid)| valid.iter().map(|t| t.as_snake_case().to_string()))
                .collect();

            return Err(anyhow::anyhow!(
                "Invalid applies_to targets: {:?} for rule type '{}'. Valid options are: {:?}",
                invalid_targets,
                self.rule.as_str(),
                valid_options
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub manifest_tests: Vec<ManifestRule>,
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
        for rule in &mut self.manifest_tests {
            if rule.applies_to.is_none() {
                rule.applies_to = Some(default_applies_to_for_rule(&rule.rule));
            }
            rule.normalize_includes_excludes();
        }
    }
    // Validate each manifest rule's applies_to targets
    //  # Errors
    // Returns an error if any rule has invalid `applies_to` targets for that specific rule
    ///
    /// # Errors
    /// Returns an error if any rule has invalid `applies_to` targets for that specific rule
    pub fn validate(&self) -> Result<()> {
        for rule in &self.manifest_tests {
            rule.validate_applies_to()?;
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
        assert_eq!(config.manifest_tests.len(), 1);
        assert_eq!(
            config.manifest_tests[0].name.as_deref(),
            Some("model_seeds_have_description")
        );
        assert_eq!(config.manifest_tests[0].severity, Severity::Error);
        assert_eq!(
            config.manifest_tests[0].name,
            Some("model_seeds_have_description".to_string())
        );
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
