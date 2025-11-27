use regex::Regex;

pub trait IncludeExcludable {
    fn get_path(&self) -> &String;
}

pub fn should_run_test<T: IncludeExcludable>(
    object: &T,
    includes: Option<&Vec<String>>,
    excludes: Option<&Vec<String>>,
) -> bool {
    let path = object.get_path();
    // 1. Exact exclude -> always exclude
    if let Some(ex) = excludes {
        if ex.iter().any(|p| !p.contains('*') && p == path) {
            return false;
        }
    }

    // 2. Exact include -> always include (excepted by exact exclude above)
    if let Some(inc) = includes {
        if inc.iter().any(|p| !p.contains('*') && p == path) {
            return true; //
        }
    }

    // 3. Wildcard exclude -> exclude
    if let Some(ex) = excludes {
        if ex
            .iter()
            .any(|p| p.contains('*') && wildcard_match(p, path))
        {
            return false;
        }
    }

    // 4. Wildcard include -> include
    if let Some(inc) = includes {
        if inc.iter().any(|p| wildcard_match(p, path)) {
            return true;
        }
        return false;
    }

    // 5. Default allow
    true
}

fn wildcard_match(pattern: &str, path: &str) -> bool {
    let regex_pattern = regex::escape(pattern).replace(r"\*", ".*");
    let re = Regex::new(&format!("^{regex_pattern}$")).unwrap();
    re.is_match(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestObject {
        path: String,
    }
    impl IncludeExcludable for TestObject {
        fn get_path(&self) -> &String {
            &self.path
        }
    }

    // Include tests
    #[test]
    fn test_includes_specific_file() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = Some(vec!["dbt_project/models/my_model.sql".to_string()]);
        let excludes = None;
        assert!(
            should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should be included based on complete path match"
        );
    }
    #[test]
    fn test_includes_specific_file_partial_match() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = Some(vec!["my_model".to_string()]);
        let excludes = None;
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should not be included based on partial match"
        );
    }

    #[test]
    fn test_includes_with_wildcard_same_folder() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = Some(vec!["dbt_project/models/*.sql".to_string()]);
        let excludes = None;
        assert!(
            should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should be included based on wildcard match in the same folder"
        );
    }

    #[test]
    fn test_includes_with_wildcard_parent_recursive() {
        let obj = TestObject {
            path: "dbt_project/models/subfolder/my_model.sql".to_string(),
        };
        let includes = Some(vec!["dbt_project/models/**/*.sql".to_string()]);
        let excludes = None;
        assert!(
            should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should be included based on recursive wildcard match in parent folder"
        );
    }

    #[test]
    fn test_includes_wildcard_completely_different_folder() {
        let obj = TestObject {
            path: "dbt_project/other_folder/my_model.sql".to_string(),
        };
        let includes = Some(vec!["dbt_project/models/*.sql".to_string()]);
        let excludes = None;
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should not be included based on wildcard match in a different folder"
        );
    }

    // Exclude tests
    #[test]
    fn test_excludes_specific_file() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = None;
        let excludes = Some(vec!["dbt_project/models/my_model.sql".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should be excluded based on complete path match"
        );
    }

    #[test]
    fn test_excludes_specific_file_partial_match() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = None;
        let excludes = Some(vec!["my_model".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should not be excluded based on partial match"
        );
    }

    #[test]
    fn test_excludes_with_wildcard() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = None;
        let excludes = Some(vec!["dbt_project/models/*.sql".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should be excluded based on wildcard match"
        );
    }

    #[test]
    fn test_excludes_with_wildcard_recursive() {
        let obj = TestObject {
            path: "dbt_project/models/subfolder/my_model.sql".to_string(),
        };
        let includes = None;
        let excludes = Some(vec!["dbt_project/models/**/*.sql".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should be excluded based on recursive wildcard match"
        );
    }

    #[test]
    fn test_excludes_wildcard_no_match() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = None;
        let excludes = Some(vec!["dbt_project/other_folder/*.sql".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should not be excluded when wildcard does not match"
        );
    }

    #[test]
    fn exact_include_overrides_wildcard_exclude() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = Some(vec!["dbt_project/models/my_model.sql".to_string()]);
        let excludes = Some(vec!["dbt_project/models/*.sql".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Exact include should override wildcard exclude"
        );
    }

    #[test]
    fn exact_exclude_overrides_wildcard_include() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = Some(vec!["dbt_project/models/*.sql".to_string()]);
        let excludes = Some(vec!["dbt_project/models/my_model.sql".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Exact exclude should override wildcard include"
        );
    }

    #[test]
    fn wildcard_exclude_overrides_wildcard_include() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = Some(vec!["dbt_project/models/*.sql".to_string()]);
        let excludes = Some(vec!["dbt_project/models/*.sql".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Wildcard exclude should override wildcard include"
        );
    }

    #[test]
    fn test_no_includes_excludes() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = None;
        let excludes = None;
        assert!(
            should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should be included when no includes or excludes are specified"
        );
    }
}
