// Trait implementations for Macro that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::common_traits::Identifiable;
use crate::core::rules::rule_config::allowed_subfolders::PathCheckable;
use crate::core::rules::rule_config::code_max_lines::HasCode;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::has_metadata_keys::HasMetadata;
use crate::core::rules::rule_config::has_tags::Tagable;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::dbt_objects::{Meta, Tags};
use dbt_artifact_parser::manifest::Macro;

impl RuleTargetable for Macro {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::Macros
    }
}

impl IncludeExcludable for Macro {
    fn get_original_file_path(&self) -> &String {
        self.get_original_file_path()
    }
}

impl Tagable for Macro {
    // Macro's do not contain a tags field.
    fn get_tags(&self) -> Option<&Tags> {
        None
    }
}

impl Identifiable for Macro {
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

impl Descriptable for Macro {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

impl NameAble for Macro {
    fn name(&self) -> &str {
        self.get_name()
    }
}

impl HasMetadata for Macro {
    fn get_metadata(&self) -> Option<&Meta> {
        self.meta.as_ref()
    }
}

impl HasCode for Macro {
    fn get_raw_code(&self) -> Option<&str> {
        Some(&self.macro_sql)
    }
}

impl PathCheckable for Macro {
    fn get_rule_target(&self) -> RuleTarget {
        self.ruletarget()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value};

    fn make_macro(
        patch_path: Option<&str>,
        description: Option<&str>,
        meta: Option<Meta>,
    ) -> Macro {
        Macro {
            name: "my_macro".to_string(),
            package_name: "my_project".to_string(),
            original_file_path: "macros/my_macro.sql".to_string(),
            patch_path: patch_path.map(String::from),
            macro_sql: "{% macro my_macro() %}\nSELECT 1\n{% endmacro %}".to_string(),
            description: description.map(String::from),
            meta,
        }
    }

    #[test]
    fn test_ruletarget() {
        let m = make_macro(None, None, None);
        assert_eq!(m.ruletarget(), RuleTarget::Macros);
    }

    #[test]
    fn test_include_excludable() {
        let m = make_macro(None, None, None);
        assert_eq!(
            IncludeExcludable::get_original_file_path(&m),
            "macros/my_macro.sql"
        );
    }

    #[test]
    fn test_tags_always_none() {
        let m = make_macro(None, None, None);
        assert!(m.get_tags().is_none());
    }

    #[test]
    fn test_identifiable_object_type() {
        let m = make_macro(None, None, None);
        assert_eq!(Identifiable::get_object_type(&m), "Macro");
    }

    #[test]
    fn test_identifiable_object_string() {
        let m = make_macro(None, None, None);
        assert_eq!(m.get_object_string(), "my_macro");
    }

    #[test]
    fn test_problematic_path_prefer_sql() {
        let m = make_macro(Some("proj://patches/macro.yml"), None, None);
        assert_eq!(m.get_problematic_path(true), Some("macros/my_macro.sql"));
    }

    #[test]
    fn test_problematic_path_with_patch() {
        let m = make_macro(Some("proj://patches/macro.yml"), None, None);
        assert_eq!(m.get_problematic_path(false), Some("patches/macro.yml"));
    }

    #[test]
    fn test_problematic_path_without_patch() {
        let m = make_macro(None, None, None);
        assert_eq!(m.get_problematic_path(false), Some("macros/my_macro.sql"));
    }

    #[test]
    fn test_description_some() {
        let m = make_macro(None, Some("A macro"), None);
        assert_eq!(m.description().unwrap(), "A macro");
    }

    #[test]
    fn test_description_none() {
        let m = make_macro(None, None, None);
        assert!(m.description().is_none());
    }

    #[test]
    fn test_name() {
        let m = make_macro(None, None, None);
        assert_eq!(NameAble::name(&m), "my_macro");
    }

    #[test]
    fn test_metadata_some() {
        let meta = Meta(Value::Object(Map::default()));
        let m = make_macro(None, None, Some(meta));
        assert!(m.get_metadata().is_some());
    }

    #[test]
    fn test_metadata_none() {
        let m = make_macro(None, None, None);
        assert!(m.get_metadata().is_none());
    }

    #[test]
    fn test_has_code() {
        let m = make_macro(None, None, None);
        let code = m.get_raw_code().unwrap();
        assert!(code.contains("SELECT 1"));
    }

    #[test]
    fn test_path_checkable() {
        let m = make_macro(None, None, None);
        assert_eq!(m.get_rule_target(), RuleTarget::Macros);
    }
}
