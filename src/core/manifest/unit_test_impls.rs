// Trait implementations for UnitTest that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::common_traits::Identifiable;
use crate::core::rules::rule_config::allowed_subfolders::PathCheckable;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::has_tags::Tagable;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::dbt_objects::Tags;
use dbt_artifact_parser::manifest::UnitTest;

impl RuleTargetable for UnitTest {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::UnitTests
    }
}

impl IncludeExcludable for UnitTest {
    fn get_original_file_path(&self) -> &String {
        self.get_original_file_path()
    }
}

impl Tagable for UnitTest {
    // Unit tests do not contain a tags field~
    fn get_tags(&self) -> Option<&Tags> {
        None
    }
}

impl Identifiable for UnitTest {
    fn get_object_type(&self) -> &str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    fn get_problematic_path(&self, prefer_sql: bool) -> Option<&str> {
        if prefer_sql {
            return Some(self.get_original_file_path());
        }
        self.get_patch_path()
            .or_else(|| Some(self.get_original_file_path()))
    }
}

impl Descriptable for UnitTest {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

impl NameAble for UnitTest {
    fn name(&self) -> &str {
        self.get_name()
    }
}

impl PathCheckable for UnitTest {
    fn get_rule_target(&self) -> RuleTarget {
        self.ruletarget()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_unit_test(patch_path: Option<&str>, description: Option<&str>) -> UnitTest {
        UnitTest {
            name: "test_orders_valid".to_string(),
            model: "orders".to_string(),
            package_name: "my_project".to_string(),
            original_file_path: "models/staging/_unit_tests.yml".to_string(),
            patch_path: patch_path.map(String::from),
            description: description.map(String::from),
        }
    }

    #[test]
    fn test_ruletarget() {
        let ut = make_unit_test(None, None);
        assert_eq!(ut.ruletarget(), RuleTarget::UnitTests);
    }

    #[test]
    fn test_include_excludable() {
        let ut = make_unit_test(None, None);
        assert_eq!(
            IncludeExcludable::get_original_file_path(&ut),
            "models/staging/_unit_tests.yml"
        );
    }

    #[test]
    fn test_tags_always_none() {
        let ut = make_unit_test(None, None);
        assert!(ut.get_tags().is_none());
    }

    #[test]
    fn test_identifiable_object_type() {
        let ut = make_unit_test(None, None);
        assert_eq!(Identifiable::get_object_type(&ut), "UnitTest");
    }

    #[test]
    fn test_identifiable_object_string() {
        let ut = make_unit_test(None, None);
        assert_eq!(ut.get_object_string(), "test_orders_valid");
    }

    #[test]
    fn test_problematic_path_prefer_sql() {
        let ut = make_unit_test(Some("proj://patches/tests.yml"), None);
        assert_eq!(
            ut.get_problematic_path(true),
            Some("models/staging/_unit_tests.yml")
        );
    }

    #[test]
    fn test_problematic_path_with_patch() {
        let ut = make_unit_test(Some("proj://patches/tests.yml"), None);
        assert_eq!(ut.get_problematic_path(false), Some("patches/tests.yml"));
    }

    #[test]
    fn test_problematic_path_without_patch() {
        let ut = make_unit_test(None, None);
        assert_eq!(
            ut.get_problematic_path(false),
            Some("models/staging/_unit_tests.yml")
        );
    }

    #[test]
    fn test_description_some() {
        let ut = make_unit_test(None, Some("Test that orders are valid"));
        assert_eq!(ut.description().unwrap(), "Test that orders are valid");
    }

    #[test]
    fn test_description_none() {
        let ut = make_unit_test(None, None);
        assert!(ut.description().is_none());
    }

    #[test]
    fn test_name() {
        let ut = make_unit_test(None, None);
        assert_eq!(NameAble::name(&ut), "test_orders_valid");
    }

    #[test]
    fn test_path_checkable() {
        let ut = make_unit_test(None, None);
        assert_eq!(ut.get_rule_target(), RuleTarget::UnitTests);
    }
}
