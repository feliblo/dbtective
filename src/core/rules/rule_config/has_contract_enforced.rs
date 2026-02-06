use crate::{
    cli::table::RuleResult,
    core::{
        config::{check_config_options::AccessLevel, manifest_rule::ManifestRule},
        rules::common_traits::Identifiable,
    },
};

pub trait ContractAble: Identifiable {
    fn get_contract_enforced(&self) -> Option<bool>;
    fn get_access(&self) -> Option<AccessLevel>;
}

pub fn has_contract_enforced<T: ContractAble>(
    model: &T,
    rule: &ManifestRule,
    access_level: Option<&AccessLevel>,
) -> Option<RuleResult> {
    // If access_level is specified, only check models with that access level
    if let Some(required_access) = access_level {
        if model.get_access().as_ref() != Some(required_access) {
            return None;
        }
    }

    if model.get_contract_enforced() == Some(true) {
        return None;
    }
    Some(RuleResult::new(
        &rule.severity,
        model.get_object_type(),
        rule.get_name(),
        format!(
            "{} does not have a contract enforced.",
            model.get_object_string()
        ),
        model.get_relative_path().map(str::to_owned),
    ))
}

#[cfg(test)]
mod tests {
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};
    use dbt_artifact_parser::manifest::nodes::Contract;

    use super::*;
    struct TestModel {
        name: String,
        contract: Option<Contract>,
        access: Option<AccessLevel>,
        relative_path: Option<String>,
    }
    impl Identifiable for TestModel {
        fn get_object_type(&self) -> &'static str {
            "Model"
        }
        fn get_object_string(&self) -> &str {
            &self.name
        }
        fn get_relative_path(&self) -> Option<&str> {
            self.relative_path.as_deref()
        }
    }
    impl ContractAble for TestModel {
        fn get_contract_enforced(&self) -> Option<bool> {
            self.contract.as_ref().map(|contract| contract.enforced)
        }
        fn get_access(&self) -> Option<AccessLevel> {
            self.access.clone()
        }
    }

    #[test]
    fn test_has_contract_enforced() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasContractEnforced { access_level: None },
            Severity::Warning,
        );
        let model_with_enforced_contract = TestModel {
            name: "model_with_contract".to_string(),
            contract: Some(Contract {
                enforced: true,
                alias_types: true,
            }),
            access: None,
            relative_path: Some("models/model_with_contract.sql".to_string()),
        };
        let model_without_enforced_contract = TestModel {
            name: "model_without_contract".to_string(),
            contract: Some(Contract {
                enforced: false,
                alias_types: true,
            }),
            access: None,
            relative_path: Some("models/model_without_contract.sql".to_string()),
        };
        let model_without_contract = TestModel {
            name: "model_without_contract".to_string(),
            contract: None,
            access: None,
            relative_path: Some("models/model_without_contract.sql".to_string()),
        };

        assert!(has_contract_enforced(&model_with_enforced_contract, &rule, None).is_none());
        assert!(has_contract_enforced(&model_without_enforced_contract, &rule, None).is_some());
        assert!(has_contract_enforced(&model_without_contract, &rule, None).is_some());
    }

    #[test]
    fn test_has_contract_enforced_with_access_level() {
        let rule = ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::HasContractEnforced {
                access_level: Some(AccessLevel::Public),
            },
            Severity::Warning,
        );

        // Public model without contract enforced -> should fail
        let public_model_no_contract = TestModel {
            name: "public_no_contract".to_string(),
            contract: Some(Contract {
                enforced: false,
                alias_types: true,
            }),
            access: Some(AccessLevel::Public),
            relative_path: Some("models/public_no_contract.sql".to_string()),
        };
        assert!(has_contract_enforced(
            &public_model_no_contract,
            &rule,
            Some(&AccessLevel::Public)
        )
        .is_some());

        // Public model with contract enforced -> should pass
        let public_model_with_contract = TestModel {
            name: "public_with_contract".to_string(),
            contract: Some(Contract {
                enforced: true,
                alias_types: true,
            }),
            access: Some(AccessLevel::Public),
            relative_path: Some("models/public_with_contract.sql".to_string()),
        };
        assert!(has_contract_enforced(
            &public_model_with_contract,
            &rule,
            Some(&AccessLevel::Public)
        )
        .is_none());

        // Protected model without contract -> should be skipped (not public)
        let protected_model_no_contract = TestModel {
            name: "protected_no_contract".to_string(),
            contract: Some(Contract {
                enforced: false,
                alias_types: true,
            }),
            access: Some(AccessLevel::Protected),
            relative_path: Some("models/protected_no_contract.sql".to_string()),
        };
        assert!(has_contract_enforced(
            &protected_model_no_contract,
            &rule,
            Some(&AccessLevel::Public)
        )
        .is_none());

        // Model with no access set without contract -> should be skipped (access is None, not public)
        let no_access_model = TestModel {
            name: "no_access_no_contract".to_string(),
            contract: Some(Contract {
                enforced: false,
                alias_types: true,
            }),
            access: None,
            relative_path: Some("models/no_access_no_contract.sql".to_string()),
        };
        assert!(
            has_contract_enforced(&no_access_model, &rule, Some(&AccessLevel::Public)).is_none()
        );
    }
}
