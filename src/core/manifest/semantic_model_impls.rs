// Trait implementations for SemanticModel that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::has_metadata_keys::HasMetadata;
use crate::core::rules::rule_config::has_refs::CanReference;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::dbt_objects::Meta;
use dbt_artifact_parser::manifest::SemanticModel;

impl RuleTargetable for SemanticModel {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::SemanticModels
    }
}

impl IncludeExcludable for SemanticModel {
    fn get_relative_path(&self) -> &String {
        self.get_relative_path()
    }
}

impl IncludeExcludable for &SemanticModel {
    fn get_relative_path(&self) -> &String {
        (*self).get_relative_path()
    }
}

impl Descriptable for SemanticModel {
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

impl NameAble for SemanticModel {
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

impl HasMetadata for SemanticModel {
    fn get_metadata(&self) -> Option<&Meta> {
        self.metadata.as_ref()
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

impl CanReference for SemanticModel {
    fn get_depends_on_nodes(&self) -> &[String] {
        match &self.depends_on.nodes {
            Some(nodes) => nodes,
            None => &[],
        }
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
