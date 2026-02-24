// Trait implementations for Source that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::common_traits::{Columnable, Identifiable};
use crate::core::rules::rule_config::allowed_subfolders::PathCheckable;
use crate::core::rules::rule_config::child_map::ChildMappable;
use crate::core::rules::rule_config::fan_in_out::ParentMappable;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::has_freshness::SourcesHaveFreshness;
use crate::core::rules::rule_config::has_loader::SourcesHaveLoader;
use crate::core::rules::rule_config::has_metadata_keys::HasMetadata;
use crate::core::rules::rule_config::has_tags::Tagable;
use crate::core::rules::rule_config::has_unique_test::TestAble;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::dbt_objects::{Meta, Tags};
use dbt_artifact_parser::manifest::{Manifest, Source, SourceFreshness};

impl RuleTargetable for Source {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::Sources
    }
}

impl IncludeExcludable for Source {
    fn get_original_file_path(&self) -> &String {
        self.get_original_file_path()
    }
}

impl Identifiable for Source {
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

impl ParentMappable for Source {
    fn get_parents<'a>(&self, manifest: &'a Manifest) -> Vec<&'a str> {
        let unique_id = self.get_unique_id();
        manifest
            .parent_map
            .get(unique_id)
            .map(|parents| parents.iter().map(String::as_str).collect())
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

impl SourcesHaveFreshness for Source {
    fn has_freshness_configured(&self) -> bool {
        self.freshness
            .as_ref()
            .is_some_and(SourceFreshness::is_configured)
    }
}

impl PathCheckable for Source {
    fn get_rule_target(&self) -> RuleTarget {
        self.ruletarget()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dbt_artifact_parser::manifest::dbt_objects::Column;
    use dbt_artifact_parser::manifest::source::FreshnessThreshold;
    use std::collections::HashMap;

    fn make_source(
        columns: Option<HashMap<String, Column>>,
        loader: Option<&str>,
        freshness: Option<SourceFreshness>,
        patch_path: Option<&str>,
        description: Option<&str>,
    ) -> Source {
        Source {
            database: Some("my_db".to_string()),
            name: "my_source".to_string(),
            description: description.map(String::from),
            package_name: "my_project".to_string(),
            original_file_path: "models/sources.yml".to_string(),
            patch_path: patch_path.map(String::from),
            unique_id: "source.my_project.my_source".to_string(),
            columns,
            meta: None,
            tags: Some(vec!["raw".to_string()]),
            loader: loader.map(String::from),
            freshness,
        }
    }

    fn make_column(description: Option<&str>, data_type: Option<&str>) -> Column {
        let desc_json = description.map_or_else(|| "null".to_string(), |d| format!(r#""{d}""#));
        let dt_json = data_type.map_or_else(|| "null".to_string(), |d| format!(r#""{d}""#));
        let json = format!(
            r#"{{"name": "col", "description": {desc_json}, "data_type": {dt_json}, "tags": []}}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn test_ruletarget() {
        let s = make_source(None, None, None, None, None);
        assert_eq!(s.ruletarget(), RuleTarget::Sources);
    }

    #[test]
    fn test_identifiable() {
        let s = make_source(None, None, None, None, None);
        assert_eq!(Identifiable::get_object_type(&s), "Source");
        assert_eq!(s.get_object_string(), "my_source");
    }

    #[test]
    fn test_problematic_path_prefer_sql() {
        let s = make_source(None, None, None, Some("proj://patches/src.yml"), None);
        assert_eq!(s.get_problematic_path(true), Some("models/sources.yml"));
    }

    #[test]
    fn test_problematic_path_with_patch() {
        let s = make_source(None, None, None, Some("proj://patches/src.yml"), None);
        assert_eq!(s.get_problematic_path(false), Some("patches/src.yml"));
    }

    #[test]
    fn test_problematic_path_without_patch() {
        let s = make_source(None, None, None, None, None);
        assert_eq!(s.get_problematic_path(false), Some("models/sources.yml"));
    }

    #[test]
    fn test_description() {
        let s = make_source(None, None, None, None, Some("Raw data"));
        assert_eq!(s.description().unwrap(), "Raw data");
    }

    #[test]
    fn test_tags() {
        let s = make_source(None, None, None, None, None);
        let tags = s.get_tags().unwrap();
        assert_eq!(tags, &vec!["raw".to_string()]);
    }

    #[test]
    fn test_column_names_none() {
        let s = make_source(None, None, None, None, None);
        assert!(s.get_column_names().is_none());
    }

    #[test]
    fn test_column_names_some() {
        let mut cols = HashMap::new();
        cols.insert("id".to_string(), make_column(None, None));
        let s = make_source(Some(cols), None, None, None, None);
        let names = s.get_column_names().unwrap();
        assert_eq!(names.len(), 1);
    }

    #[test]
    fn test_columns_with_descriptions() {
        let mut cols = HashMap::new();
        cols.insert("id".to_string(), make_column(Some("The ID"), None));
        cols.insert("name".to_string(), make_column(None, None));
        let s = make_source(Some(cols), None, None, None, None);
        let with_desc = s.get_columns_with_descriptions().unwrap();
        assert_eq!(with_desc.len(), 1);
    }

    #[test]
    fn test_columns_with_types() {
        let mut cols = HashMap::new();
        cols.insert("id".to_string(), make_column(None, Some("INTEGER")));
        cols.insert("name".to_string(), make_column(None, None));
        let s = make_source(Some(cols), None, None, None, None);
        let with_types = s.get_columns_with_types().unwrap();
        assert_eq!(with_types.len(), 1);
    }

    #[test]
    fn test_unique_id() {
        let s = make_source(None, None, None, None, None);
        assert_eq!(TestAble::get_unique_id(&s), "source.my_project.my_source");
    }

    #[test]
    fn test_loader_some() {
        let s = make_source(None, Some("stitch"), None, None, None);
        assert_eq!(s.loader().unwrap(), "stitch");
    }

    #[test]
    fn test_loader_none() {
        let s = make_source(None, None, None, None, None);
        assert!(s.loader().is_none());
    }

    #[test]
    fn test_freshness_configured() {
        let freshness = SourceFreshness {
            warn_after: Some(FreshnessThreshold {
                count: Some(24),
                period: Some("hour".to_string()),
            }),
            error_after: None,
        };
        let s = make_source(None, None, Some(freshness), None, None);
        assert!(s.has_freshness_configured());
    }

    #[test]
    fn test_freshness_not_configured() {
        let freshness = SourceFreshness {
            warn_after: Some(FreshnessThreshold {
                count: None,
                period: None,
            }),
            error_after: None,
        };
        let s = make_source(None, None, Some(freshness), None, None);
        assert!(!s.has_freshness_configured());
    }

    #[test]
    fn test_freshness_none() {
        let s = make_source(None, None, None, None, None);
        assert!(!s.has_freshness_configured());
    }

    #[test]
    fn test_path_checkable() {
        let s = make_source(None, None, None, None, None);
        assert_eq!(s.get_rule_target(), RuleTarget::Sources);
    }
}
