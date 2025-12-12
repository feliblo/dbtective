use std::vec;

use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;

use crate::core::config::applies_to::AppliesTo;
use crate::core::config::applies_to::RuleTarget;
use crate::core::config::check_config_options::{
    default_allowed_references, default_allowed_test_names, HasTagsCriteria, OrphanedReferenceType,
};
use crate::core::config::severity::Severity;
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[derive(Debug, Deserialize, EnumIter, AsRefStr, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ManifestSpecificRuleConfig {
    HasDescription {},
    NameConvention {
        pattern: String,
    },
    HasTags {
        required_tags: Vec<String>,
        #[serde(default)]
        criteria: HasTagsCriteria,
    },
    IsNotOrphaned {
        #[serde(default = "default_allowed_references")]
        allowed_references: Vec<OrphanedReferenceType>,
    },
    HasUniqueTest {
        #[serde(default = "default_allowed_test_names")]
        allowed_test_names: Vec<String>,
    },
    HasContractEnforced {},
    HasMetadataKeys {
        required_keys: Vec<String>,
        #[serde(default)]
        custom_message: Option<String>,
    },
}

impl ManifestSpecificRuleConfig {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

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
    pub rule: ManifestSpecificRuleConfig,
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
        let options = applies_to_options_for_manifest_rule(&self.rule);
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

// default options if applies_to is not set
pub fn default_applies_to_for_manifest_rule(rule_type: &ManifestSpecificRuleConfig) -> AppliesTo {
    match rule_type {
        // has_description
        ManifestSpecificRuleConfig::HasDescription {} => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Seeds, RuleTarget::Snapshots],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![RuleTarget::UnitTests],
            macro_objects: vec![RuleTarget::Macros],
            exposure_objects: vec![RuleTarget::Exposures],
            semantic_model_objects: vec![RuleTarget::SemanticModels],
            custom_objects: vec![],
        },
        // name_convention
        ManifestSpecificRuleConfig::NameConvention { .. } => AppliesTo {
            node_objects: vec![
                RuleTarget::Models,
                RuleTarget::Seeds,
                RuleTarget::Snapshots,
                RuleTarget::Analyses,
            ],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![RuleTarget::UnitTests],
            macro_objects: vec![RuleTarget::Macros],
            exposure_objects: vec![RuleTarget::Exposures],
            semantic_model_objects: vec![RuleTarget::SemanticModels],
            custom_objects: vec![],
        },
        // has_tags
        ManifestSpecificRuleConfig::HasTags { .. } => AppliesTo {
            node_objects: vec![
                RuleTarget::Models,
                RuleTarget::Seeds,
                RuleTarget::Snapshots,
                RuleTarget::Analyses,
            ],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![RuleTarget::Exposures],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        // is_not_orphaned
        ManifestSpecificRuleConfig::IsNotOrphaned { .. } => AppliesTo {
            node_objects: vec![],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::HasUniqueTest { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::HasContractEnforced { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models],
            macro_objects: vec![],
            source_objects: vec![],
            unit_test_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::HasMetadataKeys { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![RuleTarget::Macros],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
    }
}

// All options a user can choose
fn applies_to_options_for_manifest_rule(rule_type: &ManifestSpecificRuleConfig) -> AppliesTo {
    match rule_type {
        // has_description
        ManifestSpecificRuleConfig::HasDescription {} => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Seeds, RuleTarget::Snapshots],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![RuleTarget::UnitTests],
            macro_objects: vec![RuleTarget::Macros],
            exposure_objects: vec![RuleTarget::Exposures],
            semantic_model_objects: vec![RuleTarget::SemanticModels],
            custom_objects: vec![],
        },
        // name_convention
        ManifestSpecificRuleConfig::NameConvention { .. } => AppliesTo {
            node_objects: vec![
                RuleTarget::Models,
                RuleTarget::Seeds,
                RuleTarget::Snapshots,
                RuleTarget::Analyses,
            ],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![RuleTarget::UnitTests],
            macro_objects: vec![RuleTarget::Macros],
            exposure_objects: vec![RuleTarget::Exposures],
            semantic_model_objects: vec![RuleTarget::SemanticModels],
            custom_objects: vec![],
        },
        // has_tags
        ManifestSpecificRuleConfig::HasTags { .. } => AppliesTo {
            node_objects: vec![
                RuleTarget::Models,
                RuleTarget::Seeds,
                RuleTarget::Snapshots,
                RuleTarget::Analyses,
            ],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![RuleTarget::UnitTests],
            macro_objects: vec![],
            exposure_objects: vec![RuleTarget::Exposures],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        // is_not_orphaned
        ManifestSpecificRuleConfig::IsNotOrphaned { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Seeds],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::HasUniqueTest { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Seeds, RuleTarget::Snapshots],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::HasContractEnforced { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models],
            macro_objects: vec![],
            source_objects: vec![],
            unit_test_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::HasMetadataKeys { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Seeds, RuleTarget::Snapshots],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![RuleTarget::Macros],
            exposure_objects: vec![RuleTarget::Exposures],
            semantic_model_objects: vec![RuleTarget::SemanticModels],
            custom_objects: vec![],
        },
    }
}
