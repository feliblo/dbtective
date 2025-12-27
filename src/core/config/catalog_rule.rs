use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;

use crate::core::config::applies_to::RuleTarget;
use crate::core::config::materialization::Materialization;
use crate::core::config::naming_convention::NamingConvention;
use crate::core::config::{applies_to::AppliesTo, severity::Severity};
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[derive(Debug, Deserialize, EnumIter, AsRefStr, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum CatalogSpecificRuleConfig {
    ColumnsAllDocumented {},
    ColumnsHaveDescription {},
    ColumnsNameConvention {
        #[serde(rename = "pattern")]
        convention: NamingConvention,
        data_types: Option<Vec<DataTypes>>,
    },
}

#[derive(Debug, Deserialize, EnumIter, AsRefStr, EnumString, PartialEq, Eq, Hash, Clone)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DataTypes {
    // Numeric types
    Integer,
    BigInt,
    SmallInt,
    TinyInt,
    Decimal,
    Numeric,
    Float,
    Double,
    Real,

    // String types
    String,
    Text,
    Varchar,
    Char,

    // Date/Time types
    Date,
    DateTime,
    Time,
    Timestamp,
    Timestamptz,

    // Boolean
    Boolean,

    // Semi-structured types
    Json,
    Jsonb,
    Array,
    Object,
    Variant,

    // Binary types
    Binary,
    Varbinary,

    // Other types
    Uuid,
    Interval,
}

impl CatalogSpecificRuleConfig {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

const fn catalog_default_severity() -> Severity {
    Severity::Error
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogRule {
    pub name: Option<String>,
    #[serde(default = "catalog_default_severity")]
    pub severity: Severity,
    pub description: Option<String>,
    pub includes: Option<Vec<String>>,
    pub excludes: Option<Vec<String>>,
    pub applies_to: Option<AppliesTo>,
    pub model_materializations: Option<Vec<Materialization>>,
    #[serde(flatten)]
    pub rule: CatalogSpecificRuleConfig,
}

impl CatalogRule {
    #[allow(dead_code)]
    pub fn get_name(&self) -> String {
        self.name
            .as_ref()
            .map_or_else(|| self.rule.as_str().to_string(), Clone::clone)
    }

    #[allow(dead_code)]
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
        let options = applies_to_options_for_catalog_rule(&self.rule);
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

pub fn default_applies_to_for_catalog_rule(rule_type: &CatalogSpecificRuleConfig) -> AppliesTo {
    match rule_type {
        CatalogSpecificRuleConfig::ColumnsAllDocumented { .. }
        | CatalogSpecificRuleConfig::ColumnsHaveDescription { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Seeds, RuleTarget::Snapshots],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
        CatalogSpecificRuleConfig::ColumnsNameConvention { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Seeds, RuleTarget::Snapshots],
            source_objects: vec![],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
    }
}

fn applies_to_options_for_catalog_rule(rule_type: &CatalogSpecificRuleConfig) -> AppliesTo {
    match rule_type {
        CatalogSpecificRuleConfig::ColumnsAllDocumented { .. }
        | CatalogSpecificRuleConfig::ColumnsNameConvention { .. }
        | CatalogSpecificRuleConfig::ColumnsHaveDescription { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Seeds, RuleTarget::Snapshots],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
    }
}
