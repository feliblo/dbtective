use crate::{
    cli::table::RuleResult,
    core::{
        config::{catalog_rule::CatalogRule, check_config_options::InvalidColumnName},
        rules::common_traits::Columnable,
    },
};

pub fn columns_canonical_name<C: Columnable>(
    catalog_object: &C,
    canonical: &str,
    invalid_names: &[InvalidColumnName],
    rule: &CatalogRule,
    _verbose: bool,
) -> Option<RuleResult> {
    // Return a result if there are no catalog columns
    let Some(catalog_columns) = catalog_object.get_column_names() else {
        let error_msg = format!(
            "No columns available for '{}'",
            C::get_object_string(catalog_object)
        );
        return Some(RuleResult::new(
            &rule.severity,
            C::get_object_type(catalog_object),
            rule.get_name(),
            error_msg,
            catalog_object.get_relative_path().cloned(),
        ));
    };
    let invalid_columns: Vec<&str> = catalog_columns
        .iter()
        .map(|s| s.as_str())
        .filter(|column_name| {
            column_name != &canonical
                && invalid_names
                    .iter()
                    .any(|invalid_name| invalid_name.matches(column_name))
        })
        .collect();

    if invalid_columns.is_empty() {
        return None;
    }

    Some(RuleResult::new(
        &rule.severity,
        C::get_object_type(catalog_object),
        rule.get_name(),
        format!("The following columns should be named '{canonical}': {invalid_columns:?}"),
        catalog_object.get_relative_path().cloned(),
    ))
}

#[cfg(test)]
#[allow(clippy::trivial_regex)]
mod tests {
    use super::*;
    use crate::core::config::catalog_rule::CatalogSpecificRuleConfig::ColumnsCanonicalName;
    use crate::core::config::severity::Severity;

    struct TestColumnable {
        object_type: String,
        object_string: String,
        relative_path: Option<String>,
        column_names: Option<Vec<String>>,
    }

    impl Columnable for TestColumnable {
        fn get_column_names(&self) -> Option<Vec<&String>> {
            self.column_names.as_ref().map(|cols| cols.iter().collect())
        }

        fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
            None
        }

        fn get_columns_with_types(&self) -> Option<Vec<(&String, &String)>> {
            None
        }

        fn get_object_type(&self) -> &str {
            &self.object_type
        }
        fn get_object_string(&self) -> &str {
            &self.object_string
        }
        fn get_relative_path(&self) -> Option<&String> {
            self.relative_path.as_ref()
        }
    }

    #[test]
    fn test_columns_canonical_name() {
        let test_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            relative_path: Some("models/test_model.sql".to_string()),
            column_names: Some(vec![
                "zipcode".to_string(),     // invalid
                "postal_code".to_string(), // invalid
                "zip_code".to_string(),    // valid
                "bad-name".to_string(),    // should be ignored
            ]),
        };
        let invalid_names = vec![
            InvalidColumnName::Regex(regex::Regex::new("^zip").unwrap()),
            InvalidColumnName::Literal("postal_code".to_string()),
        ];
        let rule = CatalogRule::from_specific_rule(
            ColumnsCanonicalName {
                canonical: "zip_code".to_string(),
                invalid_names: invalid_names.clone(),
            },
            Severity::Error,
        );
        let result = columns_canonical_name(&test_object, "zip_code", &invalid_names, &rule, false);

        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.object_type, "model");
        assert_eq!(rule_result.rule_name, rule.get_name());
        assert_eq!(
            rule_result.message,
            "The following columns should be named 'zip_code': [\"zipcode\", \"postal_code\"]"
        );
        assert_eq!(
            rule_result.relative_path,
            Some("models/test_model.sql".to_string())
        );
    }

    #[test]
    fn test_columns_canonical_name_only_literal() {
        let test_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            relative_path: Some("models/test_model.sql".to_string()),
            column_names: Some(vec![
                "customer_id".to_string(), // valid
                "cust_id".to_string(),     // invalid
            ]),
        };
        let invalid_names = vec![InvalidColumnName::Literal("cust_id".to_string())];
        let rule = CatalogRule::from_specific_rule(
            ColumnsCanonicalName {
                canonical: "customer_id".to_string(),
                invalid_names: invalid_names.clone(),
            },
            Severity::Warning,
        );
        let result =
            columns_canonical_name(&test_object, "customer_id", &invalid_names, &rule, false);

        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.object_type, "model");
        assert_eq!(rule_result.rule_name, rule.get_name());
        assert_eq!(
            rule_result.message,
            "The following columns should be named 'customer_id': [\"cust_id\"]"
        );
        assert_eq!(
            rule_result.relative_path,
            Some("models/test_model.sql".to_string())
        );
    }

    #[test]
    fn test_columns_canonical_name_only_regex() {
        let test_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            relative_path: Some("models/test_model.sql".to_string()),
            column_names: Some(vec![
                "user_id".to_string(),         // valid
                "usr_id".to_string(),          // invalid
                "user_identifier".to_string(), // invalid
            ]),
        };
        let invalid_names = vec![InvalidColumnName::Regex(regex::Regex::new("^usr").unwrap())];
        let rule = CatalogRule::from_specific_rule(
            ColumnsCanonicalName {
                canonical: "user_id".to_string(),
                invalid_names: invalid_names.clone(),
            },
            Severity::Error,
        );
        let result = columns_canonical_name(&test_object, "user_id", &invalid_names, &rule, false);

        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.object_type, "model");
        assert_eq!(rule_result.rule_name, rule.get_name());
        assert_eq!(
            rule_result.message,
            "The following columns should be named 'user_id': [\"usr_id\"]"
        );
        assert_eq!(
            rule_result.relative_path,
            Some("models/test_model.sql".to_string())
        );
    }

    #[test]
    fn test_columns_canonical_name_no_violations() {
        let test_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            relative_path: Some("models/test_model.sql".to_string()),
            column_names: Some(vec![
                "zip_code".to_string(),       // valid
                "ignored_column".to_string(), // valid
            ]),
        };
        let invalid_names = vec![
            InvalidColumnName::Regex(regex::Regex::new("^zip").unwrap()),
            InvalidColumnName::Literal("postal_code".to_string()),
        ];
        let rule = CatalogRule::from_specific_rule(
            ColumnsCanonicalName {
                canonical: "zip_code".to_string(),
                invalid_names: invalid_names.clone(),
            },
            Severity::Error,
        );
        let result = columns_canonical_name(&test_object, "zip_code", &invalid_names, &rule, false);
        assert!(result.is_none());
    }

    #[test]
    fn test_columns_canonical_name_no_catalog_columns() {
        let test_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            relative_path: Some("models/test_model.sql".to_string()),
            column_names: None,
        };
        let invalid_names = vec![
            InvalidColumnName::Regex(regex::Regex::new("^zip").unwrap()),
            InvalidColumnName::Literal("postal_code".to_string()),
        ];
        let rule = CatalogRule::from_specific_rule(
            ColumnsCanonicalName {
                canonical: "zip_code".to_string(),
                invalid_names: invalid_names.clone(),
            },
            Severity::Error,
        );
        let result = columns_canonical_name(&test_object, "zip_code", &invalid_names, &rule, false);
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.object_type, "model");
        assert_eq!(rule_result.rule_name, rule.get_name());
        assert_eq!(rule_result.message, "No columns available for 'test_model'");
        assert_eq!(
            rule_result.relative_path,
            Some("models/test_model.sql".to_string())
        );
    }
}
