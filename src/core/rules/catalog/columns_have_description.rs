use crate::{
    cli::table::RuleResult,
    core::{config::catalog_rule::CatalogRule, rules::common_traits::Columnable},
};

// Remember, the manifest object contains the descriptions, but the manifest isn't always exhaustive
// We need the names of the catalog columns (to check they exist in the manifest)
// Manifest only relies on actually documented columns.
pub fn columns_have_description<C: Columnable, M: Columnable>(
    catalog_object: &C,
    manifest_object: &M,
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

    // Return a result if there are no manifest columns
    let manifest_columns = match manifest_object.get_columns_with_descriptions() {
        Some(cols) if !cols.is_empty() => cols,
        _ => {
            return Some(RuleResult::new(
                &rule.severity,
                M::get_object_type(manifest_object),
                rule.get_name(),
                format!(
                    "No columns in '{}' have descriptions.",
                    M::get_object_string(manifest_object)
                ),
                manifest_object.get_relative_path().cloned(),
            ))
        }
    };

    // 1. Is there a column for each catalog column in the manifest columns?
    // 2. Does that column have a description?
    let missing_column_descriptions: Vec<&str> = manifest_columns
        .iter()
        .filter(|(name, description)| {
            description.trim().is_empty() || !catalog_columns.contains(name)
        })
        .map(|(name, _)| name.as_str())
        .collect();

    if missing_column_descriptions.is_empty() {
        return None;
    }

    Some(RuleResult::new(
        &rule.severity,
        M::get_object_type(manifest_object),
        rule.get_name(),
        format!(
            "Some columns in '{}' do not have descriptions: {:?}",
            M::get_object_string(manifest_object),
            missing_column_descriptions
        ),
        // manifest object contains the path
        manifest_object.get_relative_path().cloned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestColumnable {
        object_type: String,
        object_string: String,
        relative_path: Option<String>,
        column_names: Option<Vec<String>>,
        column_descriptions: Option<Vec<String>>,
    }

    impl Columnable for TestColumnable {
        fn get_column_names(&self) -> Option<Vec<&String>> {
            self.column_names.as_ref().map(|cols| cols.iter().collect())
        }

        fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
            let column_names: Option<Vec<(&String, &String)>> = self
                .column_names
                .as_ref()
                .map(|cols| cols.iter().map(|col| (col, col)).collect());
            let column_descriptions: Option<Vec<(&String, &String)>> = self
                .column_descriptions
                .as_ref()
                .map(|descs| descs.iter().map(|desc| (desc, desc)).collect());
            match (column_names, column_descriptions) {
                (Some(names), Some(descs)) => Some(
                    names
                        .iter()
                        .zip(descs.iter())
                        .map(|((name, _), (desc, _))| (*name, *desc))
                        .collect(),
                ),
                _ => None,
            }
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

    fn create_test_catalog_rule() -> CatalogRule {
        CatalogRule {
            name: Some("columns_have_description".to_string()),
            severity: crate::core::config::severity::Severity::Warning,
            applies_to: None,
            model_materializations: None,
            description: None,
            includes: None,
            excludes: None,
            rule:
                crate::core::config::catalog_rule::CatalogSpecificRuleConfig::ColumnsHaveDescription {  },
        }
    }

    #[test]
    fn test_check_columns_have_description_no_catalog_columns() {
        let catalog_object = TestColumnable {
            object_type: "table".to_string(),
            object_string: "test_table".to_string(),
            relative_path: Some("models/test_table.sql".to_string()),
            column_names: None,
            column_descriptions: None,
        };

        let manifest_object = TestColumnable {
            object_type: "table".to_string(),
            object_string: "test_table".to_string(),
            relative_path: Some("models/test_table.sql".to_string()),
            column_names: Some(vec!["col1".to_string(), "col2".to_string()]),
            column_descriptions: None,
        };

        let rule = create_test_catalog_rule();

        let result = columns_have_description(&catalog_object, &manifest_object, &rule, false);

        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(rule_result.message, "No columns available for 'test_table'");
    }

    #[test]
    fn test_no_column_descriptions_in_manifest() {
        let catalog_object = TestColumnable {
            object_type: "table".to_string(),
            object_string: "test_table".to_string(),
            relative_path: Some("models/test_table.sql".to_string()),
            column_names: Some(vec!["col1".to_string(), "col2".to_string()]),
            column_descriptions: None,
        };

        let manifest_object = TestColumnable {
            object_type: "table".to_string(),
            object_string: "test_table".to_string(),
            relative_path: Some("models/test_table.sql".to_string()),
            column_names: Some(vec!["col1".to_string(), "col2".to_string()]),
            column_descriptions: None,
        };
        let rule = create_test_catalog_rule();
        let result = columns_have_description(&catalog_object, &manifest_object, &rule, false);
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(
            rule_result.message,
            "No columns in 'test_table' have descriptions."
        );
    }

    #[test]
    fn test_some_column_descriptions_missing() {
        let catalog_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
            ]),
            column_descriptions: None,
        };
        let manifest_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
            ]),
            column_descriptions: Some(vec![
                "Identifier".to_string(),
                String::new(),
                "Email address".to_string(),
            ]),
        };

        let rule = create_test_catalog_rule();
        let result = columns_have_description(&catalog_object, &manifest_object, &rule, false);
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(
            rule_result.message,
            "Some columns in 'my_model' do not have descriptions: [\"name\"]"
        );
    }

    #[test]
    fn test_all_columns_have_descriptions() {
        let catalog_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
            ]),
            column_descriptions: None,
        };
        let manifest_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
            ]),
            column_descriptions: Some(vec![
                "Identifier".to_string(),
                "Name".to_string(),
                "Email address".to_string(),
            ]),
        };
        let rule = create_test_catalog_rule();
        let result = columns_have_description(&catalog_object, &manifest_object, &rule, false);
        assert!(result.is_none());
    }
}
