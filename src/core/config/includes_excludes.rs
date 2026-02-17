use regex::Regex;

pub trait IncludeExcludable {
    fn get_original_file_path(&self) -> &String;
}

pub fn should_run_test<T: IncludeExcludable>(
    object: &T,
    includes: Option<&Vec<String>>,
    excludes: Option<&Vec<String>>,
) -> bool {
    // Normalize path separators to forward slashes for cross-platform compatibility
    let raw_path = object.get_original_file_path();
    let path = &raw_path.replace('\\', "/");

    // 1. Exact exclude -> always exclude
    if let Some(ex) = excludes {
        if ex.iter().any(|p| p == path) {
            return false;
        }
    }

    // 2. Exact include -> always include (except if exact excluded above)
    if let Some(inc) = includes {
        if inc.iter().any(|p| p == path) {
            return true;
        }
    }

    // 3. Pattern exclude -> exclude
    if let Some(ex) = excludes {
        if ex.iter().any(|p| glob_match(p, path)) {
            return false;
        }
    }

    // 4. Pattern include -> include
    if let Some(inc) = includes {
        if inc.iter().any(|p| glob_match(p, path)) {
            return true;
        }
        return false;
    }

    // 5. Default allow
    true
}

/// Convert a glob pattern to a regex pattern
/// Supports:
/// - `*` matches any characters except `/`
/// - `**` matches any characters including `/`
/// - `^` at the start anchors to the beginning of the path
/// - `$` at the end anchors to the end of the path
/// - Without anchors, pattern matches if it appears anywhere in the path (contains)
fn glob_to_regex(pattern: &str) -> String {
    let has_start_anchor = pattern.starts_with('^');
    let has_end_anchor = pattern.ends_with('$');

    // Remove anchors for processing
    let pattern = pattern.strip_prefix('^').unwrap_or(pattern);
    let pattern = pattern.strip_suffix('$').unwrap_or(pattern);

    // Build regex by escaping special chars and converting globs
    let mut result = String::new();
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next(); // consume second *
                                  // ** matches anything including /
                    result.push_str(".*");
                } else {
                    // * matches anything except /
                    result.push_str("[^/]*");
                }
            }
            // Escape regex metacharacters
            '.' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '\\' => {
                result.push('\\');
                result.push(c);
            }
            _ => result.push(c),
        }
    }

    // Anchor logic:
    // - ^pattern = must start with pattern
    // - pattern$ = must end with pattern
    // - ^pattern$ = exact match
    // - pattern (no anchors) = contains (can appear anywhere)
    let start = if has_start_anchor { "^" } else { "" };
    let end = if has_end_anchor { "$" } else { "" };

    format!("{start}{result}{end}")
}

/// Match a glob pattern against a path
fn glob_match(pattern: &str, path: &str) -> bool {
    let regex_pattern = glob_to_regex(pattern);
    Regex::new(&regex_pattern).is_ok_and(|re| re.is_match(path))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestObject {
        path: String,
    }
    impl IncludeExcludable for TestObject {
        fn get_original_file_path(&self) -> &String {
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
    fn test_includes_contains_match() {
        // With "contains" support, a pattern without anchors matches anywhere in the path
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = Some(vec!["my_model".to_string()]);
        let excludes = None;
        assert!(
            should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should be included based on contains match"
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
    fn test_excludes_contains_match() {
        // With "contains" support, a pattern without anchors matches anywhere in the path
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = None;
        let excludes = Some(vec!["my_model".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Object should be excluded based on contains match"
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
    fn include_excludes_all_others() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = Some(vec![
            "dbt_project/some_other_dir/some_other_model.sql".to_string()
        ]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), None),
            "File should not be included"
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

    // Anchor tests - ^ (start) and $ (end)
    #[test]
    fn test_start_anchor_matches() {
        let obj = TestObject {
            path: "models/my_model.sql".to_string(),
        };
        let excludes = Some(vec!["^models/".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            "^models/ should exclude paths starting with models/"
        );
    }

    #[test]
    fn test_start_anchor_no_match() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let excludes = Some(vec!["^models/".to_string()]);
        assert!(
            should_run_test(&obj, None, excludes.as_ref()),
            "^models/ should not exclude paths that don't start with models/"
        );
    }

    #[test]
    fn test_end_anchor_matches() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let excludes = Some(vec![".sql$".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            ".sql$ should exclude paths ending with .sql"
        );
    }

    #[test]
    fn test_end_anchor_no_match() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql.bak".to_string(),
        };
        let excludes = Some(vec![".sql$".to_string()]);
        assert!(
            should_run_test(&obj, None, excludes.as_ref()),
            ".sql$ should not exclude paths that don't end with .sql"
        );
    }

    #[test]
    fn test_both_anchors_exact_match() {
        let obj = TestObject {
            path: "models/my_model.sql".to_string(),
        };
        let excludes = Some(vec!["^models/my_model.sql$".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            "^pattern$ should match exact path"
        );
    }

    #[test]
    fn test_both_anchors_no_match() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let excludes = Some(vec!["^models/my_model.sql$".to_string()]);
        assert!(
            should_run_test(&obj, None, excludes.as_ref()),
            "^pattern$ should not match when path has prefix"
        );
    }

    #[test]
    fn test_start_anchor_with_wildcard() {
        let obj = TestObject {
            path: "models/staging/my_model.sql".to_string(),
        };
        let excludes = Some(vec!["^models/*".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            "^models/* should exclude paths starting with models/ followed by anything without /"
        );
    }

    #[test]
    fn test_start_anchor_with_double_wildcard() {
        let obj = TestObject {
            path: "models/staging/subfolder/my_model.sql".to_string(),
        };
        let excludes = Some(vec!["^models/**".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            "^models/** should exclude any path starting with models/"
        );
    }

    #[test]
    fn test_end_anchor_with_wildcard() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let includes = Some(vec!["*.sql$".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "*.sql$ should include paths ending with .sql"
        );
    }

    #[test]
    fn test_contains_no_match_different_string() {
        let obj = TestObject {
            path: "dbt_project/models/my_model.sql".to_string(),
        };
        let excludes = Some(vec!["staging".to_string()]);
        assert!(
            should_run_test(&obj, None, excludes.as_ref()),
            "Pattern 'staging' should not match path without 'staging'"
        );
    }

    // Windows path tests
    #[test]
    fn test_windows_style_paths_with_forward_slashes() {
        // dbtective normalizes paths to use forward slashes
        let obj = TestObject {
            path: "models/staging/my_model.sql".to_string(),
        };
        let includes = Some(vec!["models/staging/*.sql".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Should match Windows-normalized paths with forward slashes"
        );
    }

    #[test]
    fn test_double_star_across_directories() {
        let obj = TestObject {
            path: "models/staging/sub1/sub2/my_model.sql".to_string(),
        };
        let includes = Some(vec!["^models/**/*.sql$".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "** should match across multiple directory levels"
        );
    }

    #[test]
    fn test_single_star_does_not_cross_directories() {
        let obj = TestObject {
            path: "models/staging/my_model.sql".to_string(),
        };
        let includes = Some(vec!["^models/*.sql$".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), None),
            "Single * should not match across directory separators"
        );
    }

    #[test]
    fn test_single_star_same_directory() {
        let obj = TestObject {
            path: "models/my_model.sql".to_string(),
        };
        let includes = Some(vec!["^models/*.sql$".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Single * should match within same directory"
        );
    }

    // Integration tests for common use cases
    #[test]
    fn test_exclude_specific_model_from_folder() {
        let model1 = TestObject {
            path: "models/staging/stg_orders.sql".to_string(),
        };
        let model2 = TestObject {
            path: "models/staging/stg_customers.sql".to_string(),
        };

        let includes = Some(vec!["^models/staging/".to_string()]);
        let excludes = Some(vec!["stg_orders".to_string()]);

        assert!(
            !should_run_test(&model1, includes.as_ref(), excludes.as_ref()),
            "stg_orders should be excluded"
        );
        assert!(
            should_run_test(&model2, includes.as_ref(), excludes.as_ref()),
            "stg_customers should be included"
        );
    }

    #[test]
    fn test_include_only_sql_files() {
        let sql_file = TestObject {
            path: "models/my_model.sql".to_string(),
        };
        let yaml_file = TestObject {
            path: "models/schema.yml".to_string(),
        };

        let includes = Some(vec![".sql$".to_string()]);

        assert!(
            should_run_test(&sql_file, includes.as_ref(), None),
            "SQL files should be included"
        );
        assert!(
            !should_run_test(&yaml_file, includes.as_ref(), None),
            "YAML files should not be included"
        );
    }

    #[test]
    fn test_exclude_test_files() {
        let model = TestObject {
            path: "models/staging/stg_orders.sql".to_string(),
        };
        let test = TestObject {
            path: "tests/test_stg_orders.sql".to_string(),
        };

        let excludes = Some(vec!["^tests/".to_string()]);

        assert!(
            should_run_test(&model, None, excludes.as_ref()),
            "Model files should not be excluded"
        );
        assert!(
            !should_run_test(&test, None, excludes.as_ref()),
            "Test files should be excluded"
        );
    }

    // glob_to_regex unit tests
    #[test]
    fn test_glob_to_regex_simple() {
        assert_eq!(glob_to_regex("models"), "models");
    }

    #[test]
    fn test_glob_to_regex_star() {
        assert_eq!(glob_to_regex("*.sql"), "[^/]*\\.sql");
    }

    #[test]
    fn test_glob_to_regex_double_star() {
        assert_eq!(glob_to_regex("models/**/*.sql"), "models/.*/[^/]*\\.sql");
    }

    #[test]
    fn test_glob_to_regex_start_anchor() {
        // The ^ is kept as regex anchor, not escaped
        assert_eq!(glob_to_regex("^models/"), "^models/");
    }

    #[test]
    fn test_glob_to_regex_end_anchor() {
        assert_eq!(glob_to_regex(".sql$"), "\\.sql$");
    }

    #[test]
    fn test_glob_to_regex_both_anchors() {
        assert_eq!(glob_to_regex("^models/*.sql$"), "^models/[^/]*\\.sql$");
    }

    #[test]
    fn test_glob_to_regex_escapes_dots() {
        assert_eq!(glob_to_regex("file.sql"), "file\\.sql");
    }

    // Windows path tests - dbt normalizes paths to forward slashes on all platforms
    // These tests verify the pattern matching works with forward-slash paths
    #[test]
    fn test_windows_normalized_paths_match_patterns() {
        // Windows paths are normalized to forward slashes in dbt manifests
        let obj = TestObject {
            path: "models/staging/stg_orders.sql".to_string(),
        };
        let includes = Some(vec!["^models/staging/".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Normalized Windows paths should match forward-slash patterns"
        );
    }

    #[test]
    fn test_windows_deep_path_with_double_star() {
        // Deep paths that might come from Windows should still match ** patterns
        let obj = TestObject {
            path: "models/staging/subfolder/deep/stg_orders.sql".to_string(),
        };
        let includes = Some(vec!["^models/**".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Deep normalized paths should match ** patterns"
        );
    }

    #[test]
    fn test_backslash_in_pattern_is_escape() {
        // Backslashes in patterns should be treated as escape characters (regex behavior)
        // This means users should always use forward slashes in patterns
        let regex = glob_to_regex(r"models\\staging");
        // The backslash escapes the 's', so it becomes a literal 's'
        assert!(regex.contains("models"), "Pattern should contain models");
    }

    #[test]
    fn test_pattern_with_spaces_in_path() {
        // Paths with spaces (common on Windows) should work
        let obj = TestObject {
            path: "models/my project/stg_orders.sql".to_string(),
        };
        let includes = Some(vec!["my project".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Paths with spaces should match contains patterns"
        );
    }

    #[test]
    fn test_case_sensitive_matching() {
        // Path matching should be case-sensitive (important for cross-platform)
        let obj = TestObject {
            path: "Models/Staging/stg_orders.sql".to_string(),
        };
        let includes = Some(vec!["^models/staging/".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), None),
            "Pattern matching should be case-sensitive"
        );
    }

    #[test]
    fn test_drive_letter_paths_normalized() {
        // If a Windows path somehow kept its drive letter (shouldn't happen in dbt),
        // patterns should still work on the rest of the path
        let obj = TestObject {
            path: "C:/Users/dev/project/models/stg_orders.sql".to_string(),
        };
        let includes = Some(vec!["models/stg_orders".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Contains pattern should match anywhere including after drive letter"
        );
    }

    #[test]
    fn test_backslash_paths_normalized_to_forward_slash() {
        // Paths with backslashes (Windows) should be normalized to forward slashes
        let obj = TestObject {
            path: r"models\staging\stg_orders.sql".to_string(),
        };
        let includes = Some(vec!["^models/staging/".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Backslash paths should be normalized and match forward-slash patterns"
        );
    }

    #[test]
    fn test_mixed_slash_paths_normalized() {
        // Paths with mixed slashes should be normalized
        let obj = TestObject {
            path: r"models/staging\subfolder/stg_orders.sql".to_string(),
        };
        let includes = Some(vec!["^models/staging/subfolder/".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Mixed slash paths should be normalized"
        );
    }

    #[test]
    fn test_backslash_exact_exclude() {
        // Exact exclude patterns use forward slashes, but paths may have backslashes
        let obj = TestObject {
            path: r"models\staging\stg_orders.sql".to_string(),
        };
        // Note: exact match patterns should also use forward slashes
        let excludes = Some(vec!["models/staging/stg_orders.sql".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            "Exact exclude should work with normalized backslash paths"
        );
    }

    // =========================================================================
    // Windows manifest paths with dbt/ prefix
    // These simulate real Windows dbt manifests where original_file_path has
    // a prefix like "dbt/" and backslash separators
    // =========================================================================

    #[test]
    fn test_windows_dbt_prefix_include_directory() {
        // Real Windows manifest: "dbt/models\\marts\\mart_orders.sql"
        let obj = TestObject {
            path: r"dbt/models\marts\mart_orders.sql".to_string(),
        };
        let includes = Some(vec!["models/marts".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Include 'models/marts' should match path with dbt/ prefix via contains"
        );
    }

    #[test]
    fn test_windows_dbt_prefix_exclude_specific_file() {
        let obj = TestObject {
            path: r"dbt/models\marts\mart_orders.sql".to_string(),
        };
        let includes = Some(vec!["models/marts".to_string()]);
        let excludes = Some(vec!["models/marts/mart_orders.sql".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Exclude specific file should work with dbt/ prefix path"
        );
    }

    #[test]
    fn test_windows_dbt_prefix_exclude_by_filename_only() {
        let obj = TestObject {
            path: r"dbt/models\marts\mart_orders.sql".to_string(),
        };
        let includes = Some(vec!["models/marts".to_string()]);
        let excludes = Some(vec!["mart_orders".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Exclude by filename-only should work via contains matching"
        );
    }

    #[test]
    fn test_windows_dbt_prefix_exclude_by_filename_with_extension() {
        let obj = TestObject {
            path: r"dbt/models\marts\mart_orders.sql".to_string(),
        };
        let includes = Some(vec!["models/marts".to_string()]);
        let excludes = Some(vec!["mart_orders.sql".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Exclude by filename.sql should work via contains matching"
        );
    }

    #[test]
    fn test_windows_dbt_prefix_exclude_directory() {
        // Exclude entire directory
        let obj = TestObject {
            path: r"dbt/models\raw\subsystem\subsystem_case_info.sql".to_string(),
        };
        let excludes = Some(vec!["models/raw".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            "Exclude 'models/raw' should exclude all files under that directory"
        );
    }

    #[test]
    fn test_windows_dbt_prefix_exclude_directory_with_glob() {
        let obj = TestObject {
            path: r"dbt/models\raw\subsystem\subsystem_case_info.sql".to_string(),
        };
        let excludes = Some(vec!["models/raw/**/*".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            "Exclude 'models/raw/**/*' should exclude nested files"
        );
    }

    #[test]
    fn test_windows_dbt_prefix_include_and_exclude_combined() {
        let included_object = TestObject {
            path: r"dbt/models\marts\mart_customers.sql".to_string(),
        };
        let excluded_object = TestObject {
            path: r"dbt/models\marts\mart_orders.sql".to_string(),
        };
        let not_in_scope = TestObject {
            path: r"dbt/models\staging\stg_customers.sql".to_string(),
        };

        let includes = Some(vec!["models/marts".to_string()]);
        let excludes = Some(vec!["models/marts/mart_orders.sql".to_string()]);

        assert!(
            should_run_test(&included_object, includes.as_ref(), excludes.as_ref()),
            "mart_customers should be included"
        );
        assert!(
            !should_run_test(&excluded_object, includes.as_ref(), excludes.as_ref()),
            "mart_orders should be excluded"
        );
        assert!(
            !should_run_test(&not_in_scope, includes.as_ref(), excludes.as_ref()),
            "staging model should not be in scope (not in includes)"
        );
    }

    #[test]
    fn test_windows_dbt_prefix_deep_backslash_path() {
        let obj = TestObject {
            path: r"dbt/models\raw\subsystem\subsystem_case_info.sql".to_string(),
        };
        let includes = Some(vec!["models/raw".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Include 'models/raw' should match deeply nested Windows path"
        );
    }

    #[test]
    fn test_windows_dbt_prefix_mixed_slashes() {
        // Some Windows manifests have mixed forward/backslashes
        let obj = TestObject {
            path: r"dbt/models\marts\mart_orders.sql".to_string(),
        };
        let includes = Some(vec!["models/marts".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Mixed slashes in path should be normalized and match"
        );
    }

    #[test]
    fn test_windows_dbt_prefix_exclude_does_not_affect_others() {
        // Excluding one file should not affect other files in the same directory
        let mart_a = TestObject {
            path: r"dbt/models\marts\mart_a.sql".to_string(),
        };
        let mart_b = TestObject {
            path: r"dbt/models\marts\mart_b.sql".to_string(),
        };
        let mart_c = TestObject {
            path: r"dbt/models\marts\mart_c.sql".to_string(),
        };

        let includes = Some(vec!["models/marts".to_string()]);
        let excludes = Some(vec!["mart_b".to_string()]);

        assert!(
            should_run_test(&mart_a, includes.as_ref(), excludes.as_ref()),
            "mart_a should be included"
        );
        assert!(
            !should_run_test(&mart_b, includes.as_ref(), excludes.as_ref()),
            "mart_b should be excluded"
        );
        assert!(
            should_run_test(&mart_c, includes.as_ref(), excludes.as_ref()),
            "mart_c should be included"
        );
    }

    // =========================================================================
    // Unix manifest paths (no prefix or with prefix)
    // =========================================================================

    #[test]
    fn test_unix_path_include_directory_no_prefix() {
        let obj = TestObject {
            path: "models/marts/mart_model.sql".to_string(),
        };
        let includes = Some(vec!["models/marts".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Include should match Unix path without prefix"
        );
    }

    #[test]
    fn test_unix_path_exclude_specific_file_no_prefix() {
        let obj = TestObject {
            path: "models/marts/mart_orders.sql".to_string(),
        };
        let includes = Some(vec!["models/marts".to_string()]);
        let excludes = Some(vec!["models/marts/mart_orders.sql".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Exclude should work with exact path on Unix"
        );
    }

    #[test]
    fn test_unix_path_with_prefix_include() {
        let obj = TestObject {
            path: "dbt/models/marts/mart_model.sql".to_string(),
        };
        let includes = Some(vec!["models/marts".to_string()]);
        assert!(
            should_run_test(&obj, includes.as_ref(), None),
            "Include should match Unix path with dbt/ prefix via contains"
        );
    }

    #[test]
    fn test_unix_path_with_prefix_exclude() {
        let obj = TestObject {
            path: "dbt/models/marts/mart_orders.sql".to_string(),
        };
        let includes = Some(vec!["models/marts".to_string()]);
        let excludes = Some(vec!["models/marts/mart_orders.sql".to_string()]);
        assert!(
            !should_run_test(&obj, includes.as_ref(), excludes.as_ref()),
            "Exclude should work with Unix path with dbt/ prefix"
        );
    }

    #[test]
    fn test_unix_exclude_directory_with_prefix() {
        let obj = TestObject {
            path: "dbt/models/raw/subsystem/file.sql".to_string(),
        };
        let excludes = Some(vec!["models/raw".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            "Directory exclude should work with prefix path"
        );
    }

    #[test]
    fn test_unix_exclude_glob_with_prefix() {
        let obj = TestObject {
            path: "dbt/models/raw/subsystem/file.sql".to_string(),
        };
        let excludes = Some(vec!["models/raw/**/*".to_string()]);
        assert!(
            !should_run_test(&obj, None, excludes.as_ref()),
            "Glob exclude should work with prefix path"
        );
    }

    // =========================================================================
    // Edge cases: various exclude pattern formats
    // =========================================================================

    #[test]
    fn test_exclude_all_pattern_formats_work() {
        // All these patterns should exclude mart_orders.sql
        let obj = TestObject {
            path: r"dbt/models\marts\mart_orders.sql".to_string(),
        };

        // Pattern 1: full relative path
        let ex1 = Some(vec!["models/marts/mart_orders.sql".to_string()]);
        assert!(
            !should_run_test(&obj, None, ex1.as_ref()),
            "Full relative path exclude should work"
        );

        // Pattern 2: filename only (no extension)
        let ex2 = Some(vec!["mart_orders".to_string()]);
        assert!(
            !should_run_test(&obj, None, ex2.as_ref()),
            "Filename-only exclude should work"
        );

        // Pattern 3: filename with extension
        let ex3 = Some(vec!["mart_orders.sql".to_string()]);
        assert!(
            !should_run_test(&obj, None, ex3.as_ref()),
            "Filename with extension exclude should work"
        );

        // Pattern 4: directory path
        let ex4 = Some(vec!["models/marts".to_string()]);
        assert!(
            !should_run_test(&obj, None, ex4.as_ref()),
            "Directory path exclude should work"
        );

        // Pattern 5: glob pattern
        let ex5 = Some(vec!["models/marts/*.sql".to_string()]);
        assert!(
            !should_run_test(&obj, None, ex5.as_ref()),
            "Glob pattern exclude should work"
        );

        // Pattern 6: full path with dbt/ prefix
        let ex6 = Some(vec!["dbt/models/marts/mart_orders.sql".to_string()]);
        assert!(
            !should_run_test(&obj, None, ex6.as_ref()),
            "Full path with dbt/ prefix exclude should work (exact match after normalization)"
        );
    }

    #[test]
    fn test_include_all_pattern_formats_work() {
        let obj = TestObject {
            path: r"dbt/models\marts\mart_orders.sql".to_string(),
        };

        // Pattern 1: directory contains
        let inc1 = Some(vec!["models/marts".to_string()]);
        assert!(
            should_run_test(&obj, inc1.as_ref(), None),
            "Directory contains include should work"
        );

        // Pattern 2: filename contains
        let inc2 = Some(vec!["mart_orders".to_string()]);
        assert!(
            should_run_test(&obj, inc2.as_ref(), None),
            "Filename contains include should work"
        );

        // Pattern 3: glob
        let inc3 = Some(vec!["models/marts/*.sql".to_string()]);
        assert!(
            should_run_test(&obj, inc3.as_ref(), None),
            "Glob include should work"
        );

        // Pattern 4: full path with dbt/ prefix (exact match after normalization)
        let inc4 = Some(vec!["dbt/models/marts/mart_orders.sql".to_string()]);
        assert!(
            should_run_test(&obj, inc4.as_ref(), None),
            "Full path with prefix include should work"
        );
    }

    // =========================================================================
    // Path normalization consistency tests
    // =========================================================================

    #[test]
    fn test_all_path_formats_resolve_same_way() {
        // These should all be treated as the same model for matching purposes
        let paths = vec![
            r"dbt/models\marts\mart_orders.sql", // Windows with dbt prefix
            r"dbt\models\marts\mart_orders.sql", // Pure Windows with dbt prefix
            "dbt/models/marts/mart_orders.sql",  // Unix with dbt prefix
            r"models\marts\mart_orders.sql",     // Windows without prefix
            "models/marts/mart_orders.sql",      // Unix without prefix
        ];

        let includes = Some(vec!["models/marts".to_string()]);
        let excludes_filename = Some(vec!["mart_orders".to_string()]);

        for path_str in &paths {
            let obj = TestObject {
                path: (*path_str).to_string(),
            };
            assert!(
                should_run_test(&obj, includes.as_ref(), None),
                "Include 'models/marts' should match path: {path_str}"
            );
            assert!(
                !should_run_test(&obj, None, excludes_filename.as_ref()),
                "Exclude 'mart_orders' should match path: {path_str}"
            );
        }
    }
}
