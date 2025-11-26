use crate::core::config::parse_config::SpecificRuleConfig;
use core::fmt;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

#[allow(dead_code)]
const ALL_RULE_TARGETS: &[RuleTarget] = &[
    RuleTarget::Models,
    RuleTarget::Seeds,
    RuleTarget::Sources,
    RuleTarget::Macros,
    RuleTarget::Metrics,
    RuleTarget::Exposures,
    RuleTarget::SemanticModels,
    RuleTarget::SavedQueries,
    RuleTarget::Tests,
    RuleTarget::Analyses,
    RuleTarget::Snapshots,
    RuleTarget::HookNodes,
];

pub const fn applies_to_options_for_rule(rule_type: &SpecificRuleConfig) -> &'static [RuleTarget] {
    match rule_type {
        SpecificRuleConfig::HasDescription {} => &[
            RuleTarget::Models,
            RuleTarget::Seeds,
            RuleTarget::Sources,
            RuleTarget::Macros,
        ],
    }
}

pub const fn default_applies_to_for_rule(rule_type: &SpecificRuleConfig) -> &'static [RuleTarget] {
    match rule_type {
        SpecificRuleConfig::HasDescription {} => &[
            RuleTarget::Models,
            RuleTarget::Seeds,
            RuleTarget::Sources,
            RuleTarget::Macros,
        ],
    }
}
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, EnumIter, AsRefStr, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum RuleTarget {
    Models,
    Seeds,
    Sources,
    Macros,
    Metrics,
    Exposures,
    SemanticModels,
    SavedQueries,
    Tests,
    Analyses,
    Snapshots,
    HookNodes,
    SqlOperations,
}
impl RuleTarget {
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
            Self::Tests => "tests",
            Self::Analyses => "analyses",
            Self::Snapshots => "snapshots",
            Self::HookNodes => "hook_nodes",
            Self::SqlOperations => "sql_operations",
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
            Self::Tests => "Test",
            Self::Analyses => "Analysis",
            Self::Snapshots => "Snapshot",
            Self::HookNodes => "HookNode",
            Self::SqlOperations => "SqlOperation",
        };
        write!(f, "{singular}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_default_applies_to_for_rule() {
        let applies_to = default_applies_to_for_rule(&SpecificRuleConfig::HasDescription {});
        assert_eq!(
            applies_to,
            vec![
                RuleTarget::Models,
                RuleTarget::Seeds,
                RuleTarget::Sources,
                RuleTarget::Macros
            ]
        );
    }
}
