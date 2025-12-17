use crate::{cli::table::RuleResult, core::config::manifest_rule::ManifestRule};

pub trait ContractAble {
    fn get_contract_enforced(&self) -> Option<bool>;
    fn get_name(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String>;
    fn get_object_type(&self) -> &str;
}

pub fn has_contract_enforced<T: ContractAble>(
    model: &T,
    rule: &ManifestRule,
) -> Option<RuleResult> {
    if model.get_contract_enforced() == Some(true) {
        return None;
    }
    Some(RuleResult::new(
        &rule.severity,
        model.get_object_type(),
        rule.get_name(),
        format!("{} does not have a contract enforced.", model.get_name()),
        model.get_relative_path().cloned(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::core::{
        config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity},
        manifest::dbt_objects::nodes::node::Contract,
    };

    use super::*;
    struct TestModel {
        name: String,
        contract: Option<Contract>,
        relative_path: Option<String>,
    }
    impl ContractAble for TestModel {
        fn get_contract_enforced(&self) -> Option<bool> {
            self.contract.as_ref().map(|contract| contract.enforced)
        }
        fn get_name(&self) -> &str {
            &self.name
        }
        fn get_relative_path(&self) -> Option<&String> {
            self.relative_path.as_ref()
        }
        fn get_object_type(&self) -> &'static str {
            "Model"
        }
    }
    #[test]
    fn test_has_contract_enforced() {
        let rule = ManifestRule {
            name: Some("Has Contract Enforced".to_string()),
            severity: Severity::Warning,
            applies_to: None,
            includes: None,
            excludes: None,
            description: None,
            rule: ManifestSpecificRuleConfig::HasContractEnforced {},
        };
        let model_with_enforced_contract = TestModel {
            name: "model_with_contract".to_string(),
            contract: Some(Contract {
                enforced: true,
                alias_types: true,
            }),
            relative_path: Some("models/model_with_contract.sql".to_string()),
        };
        let model_without_enforced_contract = TestModel {
            name: "model_without_contract".to_string(),
            contract: Some(Contract {
                enforced: false,
                alias_types: true,
            }),
            relative_path: Some("models/model_without_contract.sql".to_string()),
        };
        let model_without_contract = TestModel {
            name: "model_without_contract".to_string(),
            contract: None,
            relative_path: Some("models/model_without_contract.sql".to_string()),
        };

        assert!(has_contract_enforced(&model_with_enforced_contract, &rule).is_none());
        assert!(has_contract_enforced(&model_without_enforced_contract, &rule).is_some());
        assert!(has_contract_enforced(&model_without_contract, &rule).is_some());
    }
}
