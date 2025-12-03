use crate::{
    cli::table::RuleResult,
    core::{
        checks::common_traits::Columnable, config::catalog_rule::CatalogRule, manifest::Manifest,
    },
};

#[allow(dead_code)]
pub fn check_columns_are_documented<C: Columnable, M: Columnable>(
    catalog_object: &C,
    manifest_object: &M,
    rule: &CatalogRule,
    manifest: &Manifest,
    _verbose: bool,
) -> Option<RuleResult> {
    let Some(catalog_columns) = catalog_object.get_column_names() else {
        let error_msg = format!(
            "No columns are documented for '{}'",
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

    let Some(manifest_columns) = manifest_object.get_column_names() else {
        return Some(RuleResult::new(
            &rule.severity,
            C::get_object_type(catalog_object),
            rule.get_name(),
            format!(
                "No columns in '{}' are documented: {:?}",
                C::get_object_string(catalog_object),
                catalog_columns
            ),
            // manifest object contains the path
            manifest_object.get_relative_path().cloned(),
        ));
    };

    // Snowflake requires case insensitive comparison.
    let case_insensitive = manifest
        .metadata
        .adapter_type
        .as_ref()
        .is_some_and(|adapter| adapter.eq_ignore_ascii_case("snowflake"));

    let undocumented_columns =
        compare_column_names(&catalog_columns, &manifest_columns, case_insensitive);

    if undocumented_columns.is_empty() {
        return None;
    }

    let message = if undocumented_columns.len() > 3 {
        format!(
            "Columns in '{}' not documented: {:?} & {} more",
            C::get_object_string(catalog_object),
            &undocumented_columns[..2],
            undocumented_columns.len() - 2
        )
    } else {
        format!(
            "Columns in '{}' not documented: {:?}",
            C::get_object_string(catalog_object),
            undocumented_columns
        )
    };

    Some(RuleResult::new(
        &rule.severity,
        C::get_object_type(catalog_object),
        rule.get_name(),
        message,
        // manifest object contains the path
        manifest_object.get_relative_path().cloned(),
    ))
}

fn compare_column_names(
    catalog_columns: &Vec<&String>,
    manifest_columns: &Vec<&String>,
    case_insensitive: bool,
) -> Vec<String> {
    let mut undocumented_columns = Vec::new();

    let manifest_column_set: std::collections::HashSet<String> = if case_insensitive {
        manifest_columns
            .iter()
            .map(|col| col.to_lowercase())
            .collect()
    } else {
        manifest_columns.iter().map(|col| (*col).clone()).collect()
    };

    for catalog_col in catalog_columns {
        let catalog_col_check = if case_insensitive {
            catalog_col.to_lowercase()
        } else {
            (*catalog_col).clone()
        };

        if !manifest_column_set.contains(&catalog_col_check) {
            undocumented_columns.push((*catalog_col).clone());
        }
    }

    undocumented_columns
}
#[cfg(test)]
mod tests {

    use crate::core::manifest::parse_manifest::ManifestMetadata;

    use super::*;

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
            name: Some("columns_are_documented".to_string()),
            severity: crate::core::config::severity::Severity::Warning,
            applies_to: None,
            description: None,
            includes: None,
            excludes: None,
            rule:
                crate::core::config::catalog_rule::CatalogSpecificRuleConfig::ColumnsAllDocumented {},
        }
    }

    fn create_test_manifest() -> Manifest {
        Manifest {
            metadata: ManifestMetadata {
                adapter_type: None,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_compare_column_names_case_sensitive() {
        let id = "id".to_string();
        let name = "Name".to_string();
        let age = "age".to_string();
        let catalog_columns = vec![&id, &name, &age];

        let id_manifest = "id".to_string();
        let name_manifest = "name".to_string(); // case difference
        let manifest_columns = vec![&id_manifest, &name_manifest];

        let result = compare_column_names(&catalog_columns, &manifest_columns, false);
        assert_eq!(result, vec!["Name".to_string(), "age".to_string()]);
    }

    #[test]
    fn test_compare_column_names_case_insensitive() {
        let id = "id".to_string();
        let name = "Name".to_string();
        let age = "age".to_string();
        let catalog_columns = vec![&id, &name, &age];

        let id_manifest = "ID".to_string();
        let name_manifest = "name".to_string();
        let manifest_columns = vec![&id_manifest, &name_manifest]; // case difference does not matter

        let result = compare_column_names(&catalog_columns, &manifest_columns, true);
        assert_eq!(result, vec!["age".to_string()]);
    }

    // test no catalog columns available
    #[test]
    fn test_check_columns_are_documented_no_catalog_columns() {
        let catalog_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: None,
        };

        // Doesnt matter what manifest columns are set to in this case, catalog gets priority
        let manifest_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: None,
        };

        let rule = create_test_catalog_rule();
        let manifest = create_test_manifest();

        let result = check_columns_are_documented(
            &catalog_object,
            &manifest_object,
            &rule,
            &manifest,
            false,
        );
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(
            rule_result.message,
            "No columns are documented for 'my_model'"
        );
    }

    #[test]
    fn test_check_columns_are_documented_no_manifest_columns() {
        let id = "id".to_string();
        let name = "name".to_string();
        let catalog_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![id, name]),
        };
        let manifest_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: None,
        };

        let rule = create_test_catalog_rule();
        let manifest = create_test_manifest();
        let result = check_columns_are_documented(
            &catalog_object,
            &manifest_object,
            &rule,
            &manifest,
            false,
        );
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(
            rule_result.message,
            "No columns in 'my_model' are documented: [\"id\", \"name\"]"
        );
    }

    #[test]
    fn some_columns_are_undocumented() {
        let id = "id".to_string();
        let name = "name".to_string();
        let age = "age".to_string();
        let catalog_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![id, name, age]),
        };
        let id_manifest = "id".to_string();
        let manifest_object = TestColumnable {
            object_type: "model".to_string(),
            object_string: "my_model".to_string(),
            relative_path: Some("models/my_model.sql".to_string()),
            column_names: Some(vec![id_manifest]),
        };

        let rule = create_test_catalog_rule();
        let manifest = create_test_manifest();
        let result = check_columns_are_documented(
            &catalog_object,
            &manifest_object,
            &rule,
            &manifest,
            false,
        );
        assert!(result.is_some());
        let rule_result = result.unwrap();
        assert_eq!(
            rule_result.message,
            "Columns in 'my_model' not documented: [\"name\", \"age\"]"
        );
    }
}
