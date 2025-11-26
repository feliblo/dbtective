use core::fmt;

use serde::Deserialize;

use crate::core::config::parse_config::SpecificRuleConfig;

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

pub fn default_applies_to_for_rule(rule_type: &SpecificRuleConfig) -> Vec<RuleTarget> {
    match rule_type {
        SpecificRuleConfig::HasDescription {} => [
            RuleTarget::Models,
            RuleTarget::Seeds,
            RuleTarget::Sources,
            RuleTarget::Macros,
        ]
        .to_vec(),
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
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
