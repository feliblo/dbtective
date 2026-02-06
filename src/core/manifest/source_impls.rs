// Trait implementations for Source that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::common_traits::{Columnable, Identifiable};
use crate::core::rules::rule_config::allowed_subfolders::PathCheckable;
use crate::core::rules::rule_config::child_map::ChildMappable;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::has_loader::SourcesHaveLoader;
use crate::core::rules::rule_config::has_metadata_keys::HasMetadata;
use crate::core::rules::rule_config::has_tags::Tagable;
use crate::core::rules::rule_config::has_unique_test::TestAble;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::dbt_objects::{Meta, Tags};
use dbt_artifact_parser::manifest::{Manifest, Source};

impl RuleTargetable for Source {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::Sources
    }
}

impl IncludeExcludable for Source {
    fn get_relative_path(&self) -> &String {
        self.get_relative_path()
    }
}

impl Identifiable for Source {
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

impl Descriptable for Source {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

impl NameAble for Source {
    fn name(&self) -> &str {
        self.get_name()
    }
}

impl Tagable for Source {
    fn get_tags(&self) -> Option<&Tags> {
        self.tags.as_ref()
    }
}

impl Columnable for Source {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        self.columns
            .as_ref()
            .map(|cols| cols.keys().collect::<Vec<&String>>())
    }

    fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
        self.columns.as_ref().map(|cols| {
            cols.iter()
                .filter_map(|(name, col)| col.description.as_ref().map(|desc| (name, desc)))
                .collect::<Vec<(&String, &String)>>()
        })
    }

    fn get_columns_with_types(&self) -> Option<Vec<(&String, &String)>> {
        self.columns.as_ref().map(|cols| {
            cols.iter()
                .filter_map(|(name, col)| {
                    col.data_type
                        .as_ref()
                        .or(col.datatype.as_ref())
                        .map(|dtype| (name, dtype))
                })
                .collect::<Vec<(&String, &String)>>()
        })
    }
}

impl ChildMappable for Source {
    fn get_childs<'a>(&self, manifest: &'a Manifest) -> Vec<&'a str> {
        let unique_id = self.get_unique_id();
        manifest
            .child_map
            .get(unique_id)
            .map(|children| children.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }
}

impl TestAble for Source {
    fn get_unique_id(&self) -> &String {
        self.get_unique_id()
    }
}

impl HasMetadata for Source {
    fn get_metadata(&self) -> Option<&Meta> {
        self.meta.as_ref()
    }
}

impl SourcesHaveLoader for Source {
    fn loader(&self) -> Option<&String> {
        self.loader.as_ref()
    }
}

impl PathCheckable for Source {
    fn get_rule_target(&self) -> RuleTarget {
        self.ruletarget()
    }
}
