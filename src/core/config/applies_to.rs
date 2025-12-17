use log::debug;
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

// Decides the entrypoint in Manifest.
// e.g nodes are under the "nodes" key
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleTargetType {
    Node,
    Test,
    Macro,
    Source,
    Exposure,
    SemanticModel,
    Custom,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, EnumIter, AsRefStr, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum RuleTarget {
    Models,
    Seeds,
    Metrics,
    Analyses,
    Snapshots,
    HookNodes,
    SqlOperations,
    UnitTests,
    SavedQueries,
    SemanticModels,
    Macros,
    Custom,
    Sources,
    Exposures,
}
impl RuleTarget {
    // Objects can be nodes or their own type
    pub const fn target_type(&self) -> RuleTargetType {
        match self {
            Self::Models
            | Self::Seeds
            | Self::Metrics
            | Self::Analyses
            | Self::Snapshots
            | Self::HookNodes
            | Self::SqlOperations
            | Self::SavedQueries => RuleTargetType::Node,
            Self::Sources => RuleTargetType::Source,
            Self::UnitTests => RuleTargetType::Test,
            Self::Macros => RuleTargetType::Macro,
            Self::Exposures => RuleTargetType::Exposure,
            Self::SemanticModels => RuleTargetType::SemanticModel,
            Self::Custom => RuleTargetType::Custom,
        }
    }

    pub const fn as_snake_case(&self) -> &'static str {
        match *self {
            Self::Models => "models",
            Self::Seeds => "seeds",
            Self::Sources => "sources",
            Self::Macros => "macros",
            Self::Metrics => "metrics",
            Self::Exposures => "exposures",
            Self::SemanticModels => "semantic_models",
            Self::SavedQueries => "saved_queries",
            Self::UnitTests => "unit_tests",
            Self::Analyses => "analyses",
            Self::Snapshots => "snapshots",
            Self::HookNodes => "hook_nodes",
            Self::SqlOperations => "sql_operations",
            Self::Custom => "custom",
        }
    }

    pub fn get_all_as_str() -> Vec<String> {
        Self::iter()
            .map(|target| target.as_snake_case().to_string())
            .collect()
    }
}

impl fmt::Display for RuleTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let singular = match self {
            Self::Models => "Model",
            Self::Seeds => "Seed",
            Self::Sources => "Source",
            Self::Macros => "Macro",
            Self::Metrics => "Metric",
            Self::Exposures => "Exposure",
            Self::SemanticModels => "SemanticModel",
            Self::SavedQueries => "SavedQuery",
            Self::UnitTests => "UnitTest",
            Self::Analyses => "Analysis",
            Self::Snapshots => "Snapshot",
            Self::HookNodes => "HookNode",
            Self::SqlOperations => "SqlOperation",
            Self::Custom => "Custom",
        };
        write!(f, "{singular}")
    }
}

#[derive(Debug, Default)]
pub struct AppliesTo {
    pub node_objects: Vec<RuleTarget>,
    pub macro_objects: Vec<RuleTarget>,
    pub source_objects: Vec<RuleTarget>,
    pub unit_test_objects: Vec<RuleTarget>,
    pub exposure_objects: Vec<RuleTarget>,
    pub semantic_model_objects: Vec<RuleTarget>,
    pub custom_objects: Vec<RuleTarget>,
}

impl<'de> Deserialize<'de> for AppliesTo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let items: Option<Vec<String>> = Option::deserialize(deserializer)?;
        let mut node_objects = Vec::new();
        let mut source_objects = Vec::new();
        let mut unit_test_objects = Vec::new();
        let mut macro_objects = Vec::new();
        let mut exposure_objects = Vec::new();
        let mut semantic_model_objects = Vec::new();
        let mut custom_objects = Vec::new();
        let mut unknown_targets = Vec::new();

        if let Some(items) = items {
            for item in items {
                match RuleTarget::from_str(&item) {
                    Ok(target) => match target.target_type() {
                        RuleTargetType::Node => node_objects.push(target),
                        RuleTargetType::Source => source_objects.push(target),
                        RuleTargetType::Test => unit_test_objects.push(target),
                        RuleTargetType::Macro => macro_objects.push(target),
                        RuleTargetType::Exposure => exposure_objects.push(target),
                        RuleTargetType::Custom => custom_objects.push(target),
                        RuleTargetType::SemanticModel => semantic_model_objects.push(target),
                    },
                    Err(_) => unknown_targets.push(item),
                }
            }
        }

        // First validate if there are any unknown targets - fail immediately if so
        if !unknown_targets.is_empty() {
            debug!("{unknown_targets:?}");
            let msg = format!(
                "Unknown applies_to targets: {:?}. Valid options are: {}",
                unknown_targets,
                RuleTarget::get_all_as_str().join(", ")
            );
            return Err(de::Error::custom(msg));
        }

        // Then validate if all target lists are empty
        if node_objects.is_empty()
            && source_objects.is_empty()
            && unit_test_objects.is_empty()
            && macro_objects.is_empty()
            && exposure_objects.is_empty()
            && custom_objects.is_empty()
            && semantic_model_objects.is_empty()
        {
            let msg = format!(
                "applies_to must specify at least one valid target (e.g. models, sources, tests, snapshots). Valid options are: {}",
                RuleTarget::get_all_as_str().join(", ")
            );
            return Err(de::Error::custom(msg));
        }

        Ok(Self {
            node_objects,
            macro_objects,
            source_objects,
            unit_test_objects,
            exposure_objects,
            semantic_model_objects,
            custom_objects,
        })
    }
}

#[cfg(test)]
impl AppliesTo {
    pub fn empty() -> Self {
        Self {
            node_objects: vec![],
            source_objects: vec![],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        }
    }
}
