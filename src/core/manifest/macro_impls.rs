// Trait implementations for Macro that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::common_traits::Identifiable;
use crate::core::rules::rule_config::allowed_subfolders::PathCheckable;
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

impl Identifiable for Macro {
    fn get_object_type(&self) -> &str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    fn get_relative_path(&self) -> Option<&str> {
        self.get_patch_path()
            .or_else(|| Some(self.get_relative_path()))
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
    fn get_code(&self) -> Option<&str> {
        Some(&self.macro_sql)
    }
}

impl PathCheckable for Macro {
    fn get_rule_target(&self) -> RuleTarget {
        self.ruletarget()
    }
}
