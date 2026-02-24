// Trait implementations for Exposure that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::common_traits::Identifiable;
use crate::core::rules::rule_config::allowed_subfolders::PathCheckable;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::has_metadata_keys::HasMetadata;
use crate::core::rules::rule_config::has_refs::CanReference;
use crate::core::rules::rule_config::has_tags::Tagable;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::dbt_objects::{Meta, Tags};
use dbt_artifact_parser::manifest::Exposure;

impl RuleTargetable for Exposure {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::Exposures
    }
}

impl IncludeExcludable for Exposure {
    fn get_original_file_path(&self) -> &String {
        self.get_original_file_path()
    }
}

impl Identifiable for Exposure {
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

impl Descriptable for Exposure {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

impl NameAble for Exposure {
    fn name(&self) -> &str {
        self.get_name()
    }
}

impl Tagable for Exposure {
    fn get_tags(&self) -> Option<&Tags> {
        self.tags.as_ref()
    }
}

impl HasMetadata for Exposure {
    fn get_metadata(&self) -> Option<&Meta> {
        self.meta.as_ref()
    }
}

impl CanReference for Exposure {
    fn get_depends_on_nodes(&self) -> &[String] {
        match &self.depends_on.nodes {
            Some(nodes) => nodes,
            None => &[],
        }
    }
}

impl PathCheckable for Exposure {
    fn get_rule_target(&self) -> RuleTarget {
        self.ruletarget()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dbt_artifact_parser::manifest::ExposureDependsOn;

    fn make_exposure(
        patch_path: Option<&str>,
        description: Option<&str>,
        tags: Option<Vec<String>>,
        meta: Option<Meta>,
        nodes: Option<Vec<String>>,
    ) -> Exposure {
        Exposure {
            name: "my_exposure".to_string(),
            package_name: "my_project".to_string(),
            original_file_path: "models/exposures/my_exposure.yml".to_string(),
            patch_path: patch_path.map(String::from),
            description: description.map(String::from),
            meta,
            tags,
            depends_on: ExposureDependsOn {
                macros: None,
                nodes,
            },
        }
    }

    #[test]
    fn test_ruletarget() {
        let e = make_exposure(None, None, None, None, None);
        assert_eq!(e.ruletarget(), RuleTarget::Exposures);
    }

    #[test]
    fn test_include_excludable() {
        let e = make_exposure(None, None, None, None, None);
        assert_eq!(
            IncludeExcludable::get_original_file_path(&e),
            "models/exposures/my_exposure.yml"
        );
    }

    #[test]
    fn test_identifiable_object_type() {
        let e = make_exposure(None, None, None, None, None);
        assert_eq!(Identifiable::get_object_type(&e), "Exposure");
    }

    #[test]
    fn test_identifiable_object_string() {
        let e = make_exposure(None, None, None, None, None);
        assert_eq!(e.get_object_string(), "my_exposure");
    }

    #[test]
    fn test_problematic_path_prefer_sql() {
        let e = make_exposure(
            Some("my_project://models/exposures/my_exposure.yml"),
            None,
            None,
            None,
            None,
        );
        assert_eq!(
            e.get_problematic_path(true),
            Some("models/exposures/my_exposure.yml")
        );
    }

    #[test]
    fn test_problematic_path_with_patch() {
        let e = make_exposure(
            Some("my_project://patches/exposure.yml"),
            None,
            None,
            None,
            None,
        );
        assert_eq!(e.get_problematic_path(false), Some("patches/exposure.yml"));
    }

    #[test]
    fn test_problematic_path_without_patch() {
        let e = make_exposure(None, None, None, None, None);
        assert_eq!(
            e.get_problematic_path(false),
            Some("models/exposures/my_exposure.yml")
        );
    }

    #[test]
    fn test_description_some() {
        let e = make_exposure(None, Some("A description"), None, None, None);
        assert_eq!(e.description().unwrap(), "A description");
    }

    #[test]
    fn test_description_none() {
        let e = make_exposure(None, None, None, None, None);
        assert!(e.description().is_none());
    }

    #[test]
    fn test_name() {
        let e = make_exposure(None, None, None, None, None);
        assert_eq!(NameAble::name(&e), "my_exposure");
    }

    #[test]
    fn test_tags_some() {
        let e = make_exposure(None, None, Some(vec!["tag1".to_string()]), None, None);
        let tags = e.get_tags().unwrap();
        assert_eq!(tags.len(), 1);
    }

    #[test]
    fn test_tags_none() {
        let e = make_exposure(None, None, None, None, None);
        assert!(e.get_tags().is_none());
    }

    #[test]
    fn test_metadata_some() {
        let meta = Meta(serde_json::Value::Object(serde_json::Map::default()));
        let e = make_exposure(None, None, None, Some(meta), None);
        assert!(e.get_metadata().is_some());
    }

    #[test]
    fn test_metadata_none() {
        let e = make_exposure(None, None, None, None, None);
        assert!(e.get_metadata().is_none());
    }

    #[test]
    fn test_depends_on_some() {
        let e = make_exposure(
            None,
            None,
            None,
            None,
            Some(vec!["model.my_project.users".to_string()]),
        );
        assert_eq!(e.get_depends_on_nodes().len(), 1);
    }

    #[test]
    fn test_depends_on_none() {
        let e = make_exposure(None, None, None, None, None);
        assert!(e.get_depends_on_nodes().is_empty());
    }

    #[test]
    fn test_path_checkable() {
        let e = make_exposure(None, None, None, None, None);
        assert_eq!(e.get_rule_target(), RuleTarget::Exposures);
    }
}
