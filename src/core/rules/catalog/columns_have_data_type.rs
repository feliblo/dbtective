use crate::{
    cli::table::RuleResult,
    core::{config::catalog_rule::CatalogRule, rules::common_traits::Columnable},
};

/// Check if columns have data types defined.
/// Uses the catalog object for the authoritative list of columns,
/// and the manifest object to check which have data types documented.
///
/// `min_coverage` is an optional percentage (0-100). If `None`, all columns must have data types.
/// If `Some(n)`, the percentage of columns with data types must meet the threshold.
pub fn columns_have_data_type<C: Columnable, M: Columnable>(
    catalog_object: &C,
    manifest_object: &M,
    rule: &CatalogRule,
    min_coverage: Option<u8>,
    _verbose: bool,
) -> Option<RuleResult> {
    let Some(catalog_columns) = catalog_object.get_column_names() else {
        return Some(RuleResult::new(
            &rule.severity,
            C::get_object_type(catalog_object),
            rule.get_name(),
            format!(
                "No columns available for '{}'",
                C::get_object_string(catalog_object)
            ),
            catalog_object.get_relative_path().map(str::to_owned),
        ));
    };

    if catalog_columns.is_empty() {
        return None;
    }

    let typed_columns = manifest_object.get_columns_with_types().unwrap_or_default();
    let typed_names: Vec<&str> = typed_columns
        .iter()
        .map(|(name, _)| name.as_str())
        .collect();

    let missing_columns: Vec<&str> = catalog_columns
        .iter()
        .filter(|col| !typed_names.iter().any(|t| t.eq_ignore_ascii_case(col)))
        .map(|col| col.as_str())
        .collect();

    if missing_columns.is_empty() {
        return None;
    }

    // When a percentage threshold is set, check if coverage meets it
    if let Some(threshold) = min_coverage {
        #[allow(clippy::cast_precision_loss)]
        let coverage = (typed_columns.len() as f64 / catalog_columns.len() as f64) * 100.0;

        if coverage >= f64::from(threshold) {
            return None;
        }

        return Some(RuleResult::new(
            &rule.severity,
            M::get_object_type(manifest_object),
            rule.get_name(),
            format!(
                "Only {:.0}% of columns in '{}' have data types defined (required: {}%). Missing: {:?}",
                coverage,
                M::get_object_string(manifest_object),
                threshold,
                missing_columns
            ),
            manifest_object.get_relative_path().map(str::to_owned),
        ));
    }

    // Default: all columns must have data types
    Some(RuleResult::new(
        &rule.severity,
        M::get_object_type(manifest_object),
        rule.get_name(),
        format!(
            "Some columns in '{}' do not have data types defined: {:?}",
            M::get_object_string(manifest_object),
            missing_columns
        ),
        manifest_object.get_relative_path().map(str::to_owned),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::catalog_rule::CatalogSpecificRuleConfig;
    use crate::core::config::severity::Severity;
    use crate::core::rules::common_traits::Identifiable;

    struct TestColumnable {
        object_type: String,
        object_string: String,
        relative_path: Option<String>,
        column_names: Option<Vec<String>>,
        /// Columns that have data types (name, type) pairs
        column_types: Option<Vec<(String, String)>>,
    }

    impl Identifiable for TestColumnable {
        fn get_object_type(&self) -> &str {
            &self.object_type
        }
        fn get_object_string(&self) -> &str {
            &self.object_string
        }
        fn get_relative_path(&self) -> Option<&str> {
            self.relative_path.as_deref()
        }
    }

    impl Columnable for TestColumnable {
        fn get_column_names(&self) -> Option<Vec<&String>> {
            self.column_names.as_ref().map(|cols| cols.iter().collect())
        }

        fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
            None
        }

        fn get_columns_with_types(&self) -> Option<Vec<(&String, &String)>> {
            self.column_types
                .as_ref()
                .map(|types| types.iter().map(|(name, dtype)| (name, dtype)).collect())
        }
    }

    fn create_test_rule(min_coverage: Option<u8>) -> CatalogRule {
        CatalogRule::from_specific_rule(
            CatalogSpecificRuleConfig::ColumnsHaveDataType { min_coverage },
            Severity::Warning,
        )
    }

    #[test]
    fn test_no_catalog_columns() {
        let catalog = TestColumnable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            relative_path: Some("models/test_model.sql".to_string()),
            column_names: None,
            column_types: None,
        };
        let manifest = TestColumnable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            relative_path: Some("models/test_model.sql".to_string()),
            column_names: None,
            column_types: None,
        };

        let rule = create_test_rule(None);
        let result = columns_have_data_type(&catalog, &manifest, &rule, None, false);

        assert!(result.is_some());
        assert!(result.unwrap().message.contains("No columns available for"));
    }

    #[test]
    fn test_empty_catalog_columns() {
        let catalog = TestColumnable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            relative_path: Some("models/test_model.sql".to_string()),
            column_names: Some(vec![]),
            column_types: None,
        };
        let manifest = TestColumnable {
            object_type: "model".to_string(),
            object_string: "test_model".to_string(),
            relative_path: Some("models/test_model.sql".to_string()),
            column_names: Some(vec![]),
            column_types: None,
        };

        let rule = create_test_rule(None);
        let result = columns_have_data_type(&catalog, &manifest, &rule, None, false);

        assert!(result.is_none());
    }

    #[test]
    fn test_all_columns_have_types() {
        let catalog = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
            ]),
            column_types: None,
        };
        let manifest = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
            ]),
            column_types: Some(vec![
                ("id".to_string(), "integer".to_string()),
                ("name".to_string(), "varchar".to_string()),
                ("email".to_string(), "varchar".to_string()),
            ]),
        };

        let rule = create_test_rule(None);
        let result = columns_have_data_type(&catalog, &manifest, &rule, None, false);

        assert!(result.is_none());
    }

    #[test]
    fn test_some_columns_missing_types_full_coverage() {
        let catalog = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
            ]),
            column_types: None,
        };
        let manifest = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
            ]),
            column_types: Some(vec![
                ("id".to_string(), "integer".to_string()),
                ("name".to_string(), "varchar".to_string()),
            ]),
        };

        let rule = create_test_rule(None);
        let result = columns_have_data_type(&catalog, &manifest, &rule, None, false);

        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert!(rule_result
            .message
            .contains("do not have data types defined"));
        assert!(rule_result.message.contains("email"));
    }

    #[test]
    fn test_some_columns_missing_types_percentage_pass() {
        let catalog = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
                "phone".to_string(),
            ]),
            column_types: None,
        };
        let manifest = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
                "phone".to_string(),
            ]),
            column_types: Some(vec![
                ("id".to_string(), "integer".to_string()),
                ("name".to_string(), "varchar".to_string()),
                ("email".to_string(), "varchar".to_string()),
            ]),
        };

        // 75% coverage, 50% required → pass
        let rule = create_test_rule(Some(50));
        let result = columns_have_data_type(&catalog, &manifest, &rule, Some(50), false);

        assert!(result.is_none());
    }

    #[test]
    fn test_some_columns_missing_types_percentage_fail() {
        let catalog = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
                "phone".to_string(),
            ]),
            column_types: None,
        };
        let manifest = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
                "phone".to_string(),
            ]),
            column_types: Some(vec![("id".to_string(), "integer".to_string())]),
        };

        // 25% coverage, 90% required → fail
        let rule = create_test_rule(Some(90));
        let result = columns_have_data_type(&catalog, &manifest, &rule, Some(90), false);

        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert!(rule_result.message.contains("25%"));
        assert!(rule_result.message.contains("90%"));
    }

    #[test]
    fn test_no_manifest_types_at_all() {
        let catalog = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec!["id".to_string(), "name".to_string()]),
            column_types: None,
        };
        let manifest = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec!["id".to_string(), "name".to_string()]),
            column_types: None,
        };

        let rule = create_test_rule(None);
        let result = columns_have_data_type(&catalog, &manifest, &rule, None, false);

        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert!(rule_result
            .message
            .contains("do not have data types defined"));
    }
}
