use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;

use crate::core::config::applies_to::applies_to_options_for_rule;
use crate::core::config::applies_to::AppliesTo;
use crate::core::config::parse_config::SpecificRuleConfig;
use crate::core::config::severity::Severity;

const fn manifest_default_severity() -> Severity {
    Severity::Error
}

#[derive(Debug, Deserialize)]
pub struct ManifestRule {
    pub name: Option<String>,
    #[serde(default = "manifest_default_severity")]
    pub severity: Severity,
    #[allow(dead_code)]
    pub description: Option<String>, // Human-readable description of the rule, not used in logic
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

        // Check each target in applies_to against the valid options for this rule
        // All applies to that are nodes get the Node target here
        // All other applies to get their own target type
        let pairs = [
            (&applies_to.node_objects, &options.node_objects),
            (&applies_to.source_objects, &options.source_objects),
            (&applies_to.unit_test_objects, &options.unit_test_objects),
            (&applies_to.macro_objects, &options.macro_objects),
            (&applies_to.exposure_objects, &options.exposure_objects),
            (
                &applies_to.semantic_model_objects,
                &options.semantic_model_objects,
            ),
            (&applies_to.custom_objects, &options.custom_objects),
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
