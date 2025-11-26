use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

use crate::core::config::error_handling::handle_config_error;
use crate::core::config::rule_targets::{default_applies_to_for_rule, RuleTarget};
use crate::core::config::severity::Severity;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum SpecificRuleConfig {
    #[serde(rename = "has_description")]
    HasDescription {},
}

impl SpecificRuleConfig {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::HasDescription {} => "has_description",
        }
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
    pub applies_to: Option<Vec<RuleTarget>>,
    #[serde(flatten)]
    pub rule: SpecificRuleConfig,
}

impl ManifestRule {
    pub fn get_name(&self) -> String {
        match &self.name {
            Some(name) => name.clone(),
            None => self.rule.as_str().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub manifest_tests: Vec<ManifestRule>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        let file = File::open(path)
            .context(format!("Unable to open config file at {}", path.display()))?;

        let config: std::result::Result<Self, serde_yaml::Error> = serde_yaml::from_reader(file);

        match config {
            Ok(mut cfg) => {
                cfg.apply_default_applies_to();
                Ok(cfg)
            }
            Err(err) => Err(handle_config_error(err)),
        }
    }

    pub fn apply_default_applies_to(&mut self) {
        for rule in &mut self.manifest_tests {
            if rule.applies_to.is_none() {
                rule.applies_to = Some(default_applies_to_for_rule(&rule.rule));
            }
        }
    }

    pub fn validate(&self) -> Result<()> {
        for rule in &self.manifest_tests {
            match &rule.rule {
                SpecificRuleConfig::HasDescription {} => { /* always valid */ }
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

        println!("{config:#?}");

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
   - type: "has_descriptio"
    severity: "error"
"#;
        let temp_file = create_temp_file_from_str(invalid_rule);
        let result = Config::from_file(temp_file.path());
        print!("{:?}", result);
        // result
        //     .validate()
        //     .expect_err("'invalid_rule_type' should fail validation");
    }
}
