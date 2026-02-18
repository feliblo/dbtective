use std::vec;

use anyhow::Context;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::core::config::applies_to::AppliesTo;
use crate::core::config::applies_to::RuleTarget;
use crate::core::config::check_config_options::{
    default_allowed_references, default_allowed_test_names, default_max_code_lines,
    default_max_joins, AccessLevel, HasTagsCriteria, OrphanedReferenceType,
};
use crate::core::config::naming_convention::NamingConvention;
use crate::core::config::severity::Severity;
use crate::core::config::Materialization;
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[derive(Debug, Clone, Deserialize, Serialize, EnumIter, AsRefStr, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ManifestSpecificRuleConfig {
    HasDescription {
        #[serde(default)]
        min_length: Option<usize>,
        #[serde(default)]
        forbidden_substrings: Option<Vec<String>>,
    },
    NameConvention {
        #[serde(rename = "pattern")]
        convention: NamingConvention,
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
    HasContractEnforced {
        #[serde(default)]
        access_level: Option<AccessLevel>,
    },
    HasMetadataKeys {
        required_keys: Vec<String>,
        #[serde(default)]
        custom_message: Option<String>,
    },
    HasRefs {},
    MaxCodeLines {
        #[serde(default = "default_max_code_lines")]
        max_lines: usize,
    },
    AllowedSubfolders {
        allowed_subfolders: Vec<String>,
        #[serde(default)]
        path_prefix: Option<String>,
        #[serde(default)]
        path_postfix: Option<String>,
    },
    HasForbiddenCode {
        forbidden_patterns: Vec<String>,
        #[serde(default)]
        case_sensitive: bool,
    },
    CodeContainsRefs {},
    MaxJoins {
        #[serde(default = "default_max_joins")]
        max_joins: usize,
    },
    SourcesHaveLoader {},
    SourcesHaveFreshness {},
}

// Compares only on variant discriminant, ignoring field values.
// In the init flow, rule selection uses these as set membership keys;
// the actual field values are populated later by the config builder.
impl PartialEq for ManifestSpecificRuleConfig {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for ManifestSpecificRuleConfig {}

impl std::hash::Hash for ManifestSpecificRuleConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl ManifestSpecificRuleConfig {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

const fn manifest_default_severity() -> Severity {
    Severity::Error
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestRule {
    pub name: Option<String>,
    #[serde(default = "manifest_default_severity")]
    pub severity: Severity,
    #[allow(dead_code)]
    pub description: Option<String>, // Human-readable description of the rule, not used in logic
    pub applies_to: Option<AppliesTo>,
    pub includes: Option<Vec<String>>,
    pub excludes: Option<Vec<String>>,
    pub model_materializations: Option<Vec<Materialization>>,
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
#[allow(clippy::too_many_lines)]
pub fn default_applies_to_for_manifest_rule(rule_type: &ManifestSpecificRuleConfig) -> AppliesTo {
    match rule_type {
        // has_description
        ManifestSpecificRuleConfig::HasDescription { .. } => AppliesTo {
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
        // is_not_orphaned & sources_have_loader & sources_have_freshness
        ManifestSpecificRuleConfig::IsNotOrphaned { .. }
        | ManifestSpecificRuleConfig::SourcesHaveLoader {}
        | ManifestSpecificRuleConfig::SourcesHaveFreshness {} => AppliesTo {
            node_objects: vec![],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        // has_contract_enforced
        ManifestSpecificRuleConfig::HasContractEnforced { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models],
            macro_objects: vec![],
            source_objects: vec![],
            unit_test_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::MaxCodeLines { .. }
        | ManifestSpecificRuleConfig::HasForbiddenCode { .. }
        | ManifestSpecificRuleConfig::MaxJoins { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Snapshots],
            source_objects: vec![],
            unit_test_objects: vec![],
            macro_objects: vec![RuleTarget::Macros],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        // has_unique_test & has_metadata_keys
        ManifestSpecificRuleConfig::HasUniqueTest { .. }
        | ManifestSpecificRuleConfig::HasMetadataKeys { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::HasRefs {} => AppliesTo {
            node_objects: vec![
                RuleTarget::Models,
                RuleTarget::Snapshots,
                RuleTarget::Analyses,
            ],
            source_objects: vec![],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::CodeContainsRefs {} => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Snapshots],
            source_objects: vec![],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        // allowed_subfolders
        ManifestSpecificRuleConfig::AllowedSubfolders { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models],
            source_objects: vec![],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
    }
}

// All options a user can choose
#[allow(clippy::too_many_lines)]
fn applies_to_options_for_manifest_rule(rule_type: &ManifestSpecificRuleConfig) -> AppliesTo {
    match rule_type {
        // has_description
        ManifestSpecificRuleConfig::HasDescription { .. } => AppliesTo {
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
        ManifestSpecificRuleConfig::MaxCodeLines { .. }
        | ManifestSpecificRuleConfig::HasForbiddenCode { .. }
        | ManifestSpecificRuleConfig::CodeContainsRefs {}
        | ManifestSpecificRuleConfig::MaxJoins { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Snapshots],
            source_objects: vec![],
            unit_test_objects: vec![],
            macro_objects: vec![RuleTarget::Macros],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::HasRefs {} => AppliesTo {
            node_objects: vec![
                RuleTarget::Models,
                RuleTarget::Snapshots,
                RuleTarget::Analyses,
            ],
            source_objects: vec![],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![RuleTarget::SemanticModels],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::AllowedSubfolders { .. } => AppliesTo {
            node_objects: vec![
                RuleTarget::Models,
                RuleTarget::Seeds,
                RuleTarget::Snapshots,
                RuleTarget::Analyses,
                RuleTarget::Metrics,
                RuleTarget::HookNodes,
                RuleTarget::SqlOperations,
                RuleTarget::SavedQueries,
            ],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![RuleTarget::UnitTests],
            macro_objects: vec![RuleTarget::Macros],
            exposure_objects: vec![RuleTarget::Exposures],
            semantic_model_objects: vec![RuleTarget::SemanticModels],
            custom_objects: vec![],
        },
        ManifestSpecificRuleConfig::SourcesHaveLoader {}
        | ManifestSpecificRuleConfig::SourcesHaveFreshness {} => AppliesTo {
            node_objects: vec![],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
    }
}

#[cfg(test)]
impl ManifestRule {
    /// Create a `ManifestRule` with sensible defaults for testing.
    /// Name defaults to the rule type name.
    pub fn from_specific_rule(rule: ManifestSpecificRuleConfig, severity: Severity) -> Self {
        Self {
            name: Some(rule.as_str().to_string()),
            severity,
            applies_to: None,
            includes: None,
            excludes: None,
            description: None,
            model_materializations: None,
            rule,
        }
    }
}
