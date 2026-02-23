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
