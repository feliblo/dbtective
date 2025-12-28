// Trait implementations for Macro that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::has_metadata_keys::HasMetadata;
use crate::core::rules::rule_config::max_code_lines::HasCode;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::dbt_objects::Meta;
use dbt_artifact_parser::manifest::Macro;

impl RuleTargetable for Macro {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::Macros
    }
}

impl IncludeExcludable for Macro {
    fn get_relative_path(&self) -> &String {
        self.get_relative_path()
    }
}

impl IncludeExcludable for &Macro {
    fn get_relative_path(&self) -> &String {
        (*self).get_relative_path()
    }
}

impl Descriptable for Macro {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    fn get_object_type(&self) -> &'static str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
}

impl NameAble for Macro {
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

impl HasMetadata for Macro {
    fn get_metadata(&self) -> Option<&Meta> {
        self.meta.as_ref()
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

impl HasCode for Macro {
    fn get_code(&self) -> Option<&str> {
        Some(&self.macro_sql)
    }
    fn get_name(&self) -> &str {
        self.get_name()
    }
    fn get_relative_path(&self) -> Option<&String> {
        Some(self.get_relative_path())
    }
    fn get_object_type(&self) -> &str {
        Self::get_object_type()
    }
}
