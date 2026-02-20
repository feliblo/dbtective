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
