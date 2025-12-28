// Trait implementations for UnitTest that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::UnitTest;

impl RuleTargetable for UnitTest {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::UnitTests
    }
}

impl IncludeExcludable for UnitTest {
    fn get_relative_path(&self) -> &String {
        self.get_relative_path()
    }
}

impl IncludeExcludable for &UnitTest {
    fn get_relative_path(&self) -> &String {
        (*self).get_relative_path()
    }
}

impl Descriptable for UnitTest {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    fn get_object_type(&self) -> &'static str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }
}

impl NameAble for UnitTest {
    fn name(&self) -> &str {
        self.get_name()
    }

    fn get_object_type(&self) -> &str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}
