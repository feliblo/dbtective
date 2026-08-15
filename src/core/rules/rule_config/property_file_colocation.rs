use std::path::Path;

use crate::cli::table::RuleResult;
use crate::core::config::check_config_options::ColocationMode;
use crate::core::config::manifest_rule::ManifestRule;
use crate::core::rules::common_traits::Identifiable;

/// Trait for objects that have both an original file path (SQL/CSV) and a patch path (YAML property file).
pub trait HasPatchPath: Identifiable {
    fn original_file_path(&self) -> &str;
    fn patch_path(&self) -> Option<&str>;
}

/// Normalise a path to always use forward slashes, so comparisons work on Windows too.
fn normalize_path(p: &str) -> String {
    p.replace('\\', "/")
}

/// Check that the property (YAML) file is colocated with the object's primary file.
///
/// In `SameDirectory` mode the two parent directories must be identical.
/// In `RelativeSubdirectory` mode the YAML file must live in one of the
/// `allowed_subdirectories` directly beneath the SQL file's directory.
pub fn property_file_colocation<T: HasPatchPath>(
    obj: &T,
    rule: &ManifestRule,
    mode: &ColocationMode,
    allowed_subdirectories: Option<&Vec<String>>,
) -> Option<RuleResult> {
    // No patch path means the object has no property file – nothing to check.
    let patch = normalize_path(obj.patch_path()?);

    let original = normalize_path(obj.original_file_path());

    let original_dir = Path::new(&original)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();
    let patch_dir = Path::new(&patch)
        .parent()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .unwrap_or_default();

    match mode {
        ColocationMode::SameDirectory => {
            if original_dir == patch_dir {
                None
            } else {
                Some(RuleResult::new(
                    &rule.severity,
                    obj.get_object_type(),
                    rule.get_name(),
                    format!(
                        "{} has its property file in a different directory than its definition file: `{}` vs `{}`.",
                        obj.get_object_string(),
                        patch_dir,
                        original_dir,
                    ),
                    obj.get_problematic_path(false).map(str::to_owned),
                ))
            }
        }
        ColocationMode::RelativeSubdirectory => {
            let allowed = match allowed_subdirectories {
                Some(dirs) if !dirs.is_empty() => dirs,
                _ => {
                    // No allowed subdirectories configured – behave like SameDirectory
                    if original_dir == patch_dir {
                        return None;
                    }
                    return Some(RuleResult::new(
                        &rule.severity,
                        obj.get_object_type(),
                        rule.get_name(),
                        format!(
                            "{} has its property file in a different directory than its definition file: `{}` vs `{}`.",
                            obj.get_object_string(),
                            patch_dir,
                            original_dir,
                        ),
                        obj.get_problematic_path(false).map(str::to_owned),
                    ));
                }
            };

            let is_valid = allowed.iter().any(|subdir| {
                let expected = if original_dir.is_empty() {
                    subdir.clone()
                } else {
                    format!("{original_dir}/{subdir}")
                };
                patch_dir == expected
            });

            if is_valid {
                None
            } else {
                let allowed_str = allowed
                    .iter()
                    .map(|s| format!("`{s}/`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                Some(RuleResult::new(
                    &rule.severity,
                    obj.get_object_type(),
                    rule.get_name(),
                    format!(
                        "{} has its property file in `{}` which is not in an allowed subdirectory ({}) relative to `{}`.",
                        obj.get_object_string(),
                        patch_dir,
                        allowed_str,
                        original_dir,
                    ),
                    obj.get_problematic_path(false).map(str::to_owned),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

    struct TestObject {
        original_file_path: String,
        patch_path: Option<String>,
    }

    #[allow(clippy::unnecessary_literal_bound)]
    impl Identifiable for TestObject {
        fn get_object_type(&self) -> &str {
            "model"
        }
        fn get_object_string(&self) -> &str {
            "test_model"
        }
        fn get_problematic_path(&self, _prefer_sql: bool) -> Option<&str> {
            self.patch_path.as_deref()
        }
    }

    impl HasPatchPath for TestObject {
        fn original_file_path(&self) -> &str {
            &self.original_file_path
        }
        fn patch_path(&self) -> Option<&str> {
            self.patch_path.as_deref()
        }
    }

    fn make_rule() -> ManifestRule {
        ManifestRule::from_specific_rule(
            ManifestSpecificRuleConfig::PropertyFileColocation {
                mode: ColocationMode::SameDirectory,
                allowed_subdirectories: None,
            },
            Severity::Error,
        )
    }

    // ========================================================================
    // No patch path — rule is skipped
    // ========================================================================

    #[test]
    fn test_no_patch_path_returns_none() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: None,
        };
        let rule = make_rule();
        let result = property_file_colocation(&obj, &rule, &ColocationMode::SameDirectory, None);
        assert!(result.is_none());
    }

    // ========================================================================
    // SameDirectory mode — Unix paths
    // ========================================================================

    #[test]
    fn test_same_directory_passes_unix() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: Some("models/staging/_stg_orders.yml".to_string()),
        };
        let rule = make_rule();
        let result = property_file_colocation(&obj, &rule, &ColocationMode::SameDirectory, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_same_directory_fails_unix() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: Some("models/schema.yml".to_string()),
        };
        let rule = make_rule();
        let result = property_file_colocation(&obj, &rule, &ColocationMode::SameDirectory, None);
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains("different directory"));
        assert!(msg.contains("models"));
        assert!(msg.contains("models/staging"));
    }

    // ========================================================================
    // SameDirectory mode — Windows paths
    // ========================================================================

    #[test]
    fn test_same_directory_passes_windows() {
        let obj = TestObject {
            original_file_path: "models\\staging\\stg_orders.sql".to_string(),
            patch_path: Some("models\\staging\\_stg_orders.yml".to_string()),
        };
        let rule = make_rule();
        let result = property_file_colocation(&obj, &rule, &ColocationMode::SameDirectory, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_same_directory_fails_windows() {
        let obj = TestObject {
            original_file_path: "models\\staging\\stg_orders.sql".to_string(),
            patch_path: Some("models\\schema.yml".to_string()),
        };
        let rule = make_rule();
        let result = property_file_colocation(&obj, &rule, &ColocationMode::SameDirectory, None);
        assert!(result.is_some());
    }

    #[test]
    fn test_same_directory_mixed_separators() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: Some("models\\staging\\_stg_orders.yml".to_string()),
        };
        let rule = make_rule();
        let result = property_file_colocation(&obj, &rule, &ColocationMode::SameDirectory, None);
        assert!(result.is_none());
    }

    // ========================================================================
    // RelativeSubdirectory mode — Unix paths
    // ========================================================================

    #[test]
    fn test_relative_subdirectory_passes_unix() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: Some("models/staging/properties/_stg_orders.yml".to_string()),
        };
        let rule = make_rule();
        let allowed = vec!["properties".to_string(), "tests".to_string()];
        let result = property_file_colocation(
            &obj,
            &rule,
            &ColocationMode::RelativeSubdirectory,
            Some(&allowed),
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_relative_subdirectory_second_option_passes() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: Some("models/staging/tests/_stg_orders.yml".to_string()),
        };
        let rule = make_rule();
        let allowed = vec!["properties".to_string(), "tests".to_string()];
        let result = property_file_colocation(
            &obj,
            &rule,
            &ColocationMode::RelativeSubdirectory,
            Some(&allowed),
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_relative_subdirectory_fails_wrong_subdir() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: Some("models/staging/docs/_stg_orders.yml".to_string()),
        };
        let rule = make_rule();
        let allowed = vec!["properties".to_string(), "tests".to_string()];
        let result = property_file_colocation(
            &obj,
            &rule,
            &ColocationMode::RelativeSubdirectory,
            Some(&allowed),
        );
        assert!(result.is_some());
        let msg = result.unwrap().message;
        assert!(msg.contains("not in an allowed subdirectory"));
    }

    #[test]
    fn test_relative_subdirectory_fails_same_dir() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: Some("models/staging/_stg_orders.yml".to_string()),
        };
        let rule = make_rule();
        let allowed = vec!["properties".to_string()];
        let result = property_file_colocation(
            &obj,
            &rule,
            &ColocationMode::RelativeSubdirectory,
            Some(&allowed),
        );
        assert!(result.is_some());
    }

    // ========================================================================
    // RelativeSubdirectory mode — Windows paths
    // ========================================================================

    #[test]
    fn test_relative_subdirectory_passes_windows() {
        let obj = TestObject {
            original_file_path: "models\\staging\\stg_orders.sql".to_string(),
            patch_path: Some("models\\staging\\properties\\_stg_orders.yml".to_string()),
        };
        let rule = make_rule();
        let allowed = vec!["properties".to_string()];
        let result = property_file_colocation(
            &obj,
            &rule,
            &ColocationMode::RelativeSubdirectory,
            Some(&allowed),
        );
        assert!(result.is_none());
    }

    // ========================================================================
    // RelativeSubdirectory — no allowed_subdirectories falls back to same_directory
    // ========================================================================

    #[test]
    fn test_relative_subdirectory_no_allowed_falls_back_to_same_dir() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: Some("models/staging/_stg_orders.yml".to_string()),
        };
        let rule = make_rule();
        let result =
            property_file_colocation(&obj, &rule, &ColocationMode::RelativeSubdirectory, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_relative_subdirectory_empty_allowed_falls_back_to_same_dir() {
        let obj = TestObject {
            original_file_path: "models/staging/stg_orders.sql".to_string(),
            patch_path: Some("models/other/schema.yml".to_string()),
        };
        let rule = make_rule();
        let allowed: Vec<String> = vec![];
        let result = property_file_colocation(
            &obj,
            &rule,
            &ColocationMode::RelativeSubdirectory,
            Some(&allowed),
        );
        assert!(result.is_some());
    }

    // ========================================================================
    // Root-level files (no parent directory)
    // ========================================================================

    #[test]
    fn test_root_level_same_directory() {
        let obj = TestObject {
            original_file_path: "model.sql".to_string(),
            patch_path: Some("schema.yml".to_string()),
        };
        let rule = make_rule();
        let result = property_file_colocation(&obj, &rule, &ColocationMode::SameDirectory, None);
        assert!(result.is_none());
    }

    #[test]
    fn test_root_level_relative_subdirectory() {
        let obj = TestObject {
            original_file_path: "model.sql".to_string(),
            patch_path: Some("properties/schema.yml".to_string()),
        };
        let rule = make_rule();
        let allowed = vec!["properties".to_string()];
        let result = property_file_colocation(
            &obj,
            &rule,
            &ColocationMode::RelativeSubdirectory,
            Some(&allowed),
        );
        assert!(result.is_none());
    }
}
