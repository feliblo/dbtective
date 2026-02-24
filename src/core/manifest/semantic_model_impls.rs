// Trait implementations for SemanticModel that stay in dbtective
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
use dbt_artifact_parser::manifest::SemanticModel;

impl RuleTargetable for SemanticModel {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::SemanticModels
    }
}

impl IncludeExcludable for SemanticModel {
    fn get_original_file_path(&self) -> &String {
        self.get_original_file_path()
    }
}

impl Tagable for SemanticModel {
    // Semantic models do not contain a tags field
    fn get_tags(&self) -> Option<&Tags> {
        None
    }
}

impl Identifiable for SemanticModel {
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

impl Descriptable for SemanticModel {
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}

impl NameAble for SemanticModel {
    fn name(&self) -> &str {
        self.get_name()
    }
}

impl HasMetadata for SemanticModel {
    fn get_metadata(&self) -> Option<&Meta> {
        self.metadata.as_ref()
    }
}

impl CanReference for SemanticModel {
    fn get_depends_on_nodes(&self) -> &[String] {
        match &self.depends_on.nodes {
            Some(nodes) => nodes,
            None => &[],
        }
    }
}

impl PathCheckable for SemanticModel {
    fn get_rule_target(&self) -> RuleTarget {
        self.ruletarget()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dbt_artifact_parser::manifest::SemanticModelDependsOn;

    fn make_semantic_model(
        patch_path: Option<&str>,
        description: Option<&str>,
        metadata: Option<Meta>,
        nodes: Option<Vec<String>>,
    ) -> SemanticModel {
        SemanticModel {
            name: "orders_sm".to_string(),
            package_name: "my_project".to_string(),
            original_file_path: "models/semantic/orders.yml".to_string(),
            patch_path: patch_path.map(String::from),
            description: description.map(String::from),
            metadata,
            depends_on: SemanticModelDependsOn {
                macros: None,
                nodes,
            },
        }
    }

    #[test]
    fn test_ruletarget() {
        let sm = make_semantic_model(None, None, None, None);
        assert_eq!(sm.ruletarget(), RuleTarget::SemanticModels);
    }

    #[test]
    fn test_include_excludable() {
        let sm = make_semantic_model(None, None, None, None);
        assert_eq!(
            IncludeExcludable::get_original_file_path(&sm),
            "models/semantic/orders.yml"
        );
    }

    #[test]
    fn test_tags_always_none() {
        let sm = make_semantic_model(None, None, None, None);
        assert!(sm.get_tags().is_none());
    }

    #[test]
    fn test_identifiable_object_type() {
        let sm = make_semantic_model(None, None, None, None);
        assert_eq!(Identifiable::get_object_type(&sm), "SemanticModel");
    }

    #[test]
    fn test_identifiable_object_string() {
        let sm = make_semantic_model(None, None, None, None);
        assert_eq!(sm.get_object_string(), "orders_sm");
    }

    #[test]
    fn test_problematic_path_prefer_sql() {
        let sm = make_semantic_model(Some("proj://patches/sm.yml"), None, None, None);
        assert_eq!(
            sm.get_problematic_path(true),
            Some("models/semantic/orders.yml")
        );
    }

    #[test]
    fn test_problematic_path_with_patch() {
        let sm = make_semantic_model(Some("proj://patches/sm.yml"), None, None, None);
        assert_eq!(sm.get_problematic_path(false), Some("patches/sm.yml"));
    }

    #[test]
    fn test_problematic_path_without_patch() {
        let sm = make_semantic_model(None, None, None, None);
        assert_eq!(
            sm.get_problematic_path(false),
            Some("models/semantic/orders.yml")
        );
    }

    #[test]
    fn test_description_some() {
        let sm = make_semantic_model(None, Some("A semantic model"), None, None);
        assert_eq!(sm.description().unwrap(), "A semantic model");
    }

    #[test]
    fn test_description_none() {
        let sm = make_semantic_model(None, None, None, None);
        assert!(sm.description().is_none());
    }

    #[test]
    fn test_name() {
        let sm = make_semantic_model(None, None, None, None);
        assert_eq!(NameAble::name(&sm), "orders_sm");
    }

    #[test]
    fn test_metadata_some() {
        let meta = Meta(serde_json::Value::Object(serde_json::Map::default()));
        let sm = make_semantic_model(None, None, Some(meta), None);
        assert!(sm.get_metadata().is_some());
    }

    #[test]
    fn test_metadata_none() {
        let sm = make_semantic_model(None, None, None, None);
        assert!(sm.get_metadata().is_none());
    }

    #[test]
    fn test_depends_on_some() {
        let sm = make_semantic_model(
            None,
            None,
            None,
            Some(vec!["model.proj.orders".to_string()]),
        );
        assert_eq!(sm.get_depends_on_nodes().len(), 1);
    }

    #[test]
    fn test_depends_on_none() {
        let sm = make_semantic_model(None, None, None, None);
        assert!(sm.get_depends_on_nodes().is_empty());
    }

    #[test]
    fn test_path_checkable() {
        let sm = make_semantic_model(None, None, None, None);
        assert_eq!(sm.get_rule_target(), RuleTarget::SemanticModels);
    }
}
