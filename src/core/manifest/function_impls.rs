// Trait implementations for UDF that stay in dbtective
use crate::core::config::applies_to::{RuleTarget, RuleTargetable};
use crate::core::config::includes_excludes::IncludeExcludable;
use crate::core::rules::common_traits::Identifiable;
use crate::core::rules::rule_config::allowed_subfolders::PathCheckable;
use crate::core::rules::rule_config::code_max_lines::HasCode;
use crate::core::rules::rule_config::has_description::Descriptable;
use crate::core::rules::rule_config::has_metadata_keys::HasMetadata;
use crate::core::rules::rule_config::has_refs::CanReference;
use crate::core::rules::rule_config::has_tags::Tagable;
use crate::core::rules::rule_config::name_convention::NameAble;
use dbt_artifact_parser::manifest::dbt_objects::{Meta, Tags};
use dbt_artifact_parser::manifest::UDF;

impl RuleTargetable for UDF {
    fn ruletarget(&self) -> RuleTarget {
        RuleTarget::Functions
    }
}

impl IncludeExcludable for UDF {
    fn get_original_file_path(&self) -> &String {
        self.get_original_file_path()
    }
}

impl Identifiable for UDF {
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

impl Descriptable for UDF {
    fn description(&self) -> Option<&String> {
        if self.description.is_empty() {
            None
        } else {
            Some(&self.description)
        }
    }
}

impl NameAble for UDF {
    fn name(&self) -> &str {
        self.get_name()
    }
}

impl Tagable for UDF {
    fn get_tags(&self) -> Option<&Tags> {
        self.tags.as_ref()
    }
}

impl HasMetadata for UDF {
    fn get_metadata(&self) -> Option<&Meta> {
        Some(&self.meta)
    }
}

impl HasCode for UDF {
    fn get_raw_code(&self) -> Option<&str> {
        if self.raw_code.is_empty() {
            None
        } else {
            Some(&self.raw_code)
        }
    }
}

impl CanReference for UDF {
    fn get_depends_on_nodes(&self) -> &[String] {
        match &self.depends_on.nodes {
            Some(nodes) => nodes,
            None => &[],
        }
    }
}

impl PathCheckable for UDF {
    fn get_rule_target(&self) -> RuleTarget {
        self.ruletarget()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dbt_artifact_parser::manifest::nodes::node::DependsOn;
    use dbt_artifact_parser::manifest::udf::FunctionReturns;

    fn make_udf(
        patch_path: Option<&str>,
        description: &str,
        tags: Option<Vec<String>>,
        raw_code: &str,
        nodes: Option<Vec<String>>,
    ) -> UDF {
        UDF {
            name: "my_func".to_string(),
            description: description.to_string(),
            columns: None,
            meta: Meta(serde_json::Value::Object(serde_json::Map::default())),
            group: None,
            returns: FunctionReturns {
                data_type: "integer".to_string(),
                description: None,
            },
            database: None,
            schema: "main".to_string(),
            package_name: "my_project".to_string(),
            path: "my_func.sql".to_string(),
            original_file_path: "functions/my_func.sql".to_string(),
            unique_id: "function.my_project.my_func".to_string(),
            fqn: vec!["my_project".to_string(), "my_func".to_string()],
            alias: "my_func".to_string(),
            checksum: dbt_artifact_parser::manifest::nodes::node::FileHash {
                name: "sha256".to_string(),
                checksum: "abc".to_string(),
            },
            config: serde_json::from_str(
                r#"{
                    "enabled": true,
                    "materialized": "function",
                    "on_schema_change": "ignore",
                    "post-hook": [],
                    "pre-hook": [],
                    "packages": []
                }"#,
            )
            .unwrap(),
            tags,
            docs: None,
            patch_path: patch_path.map(String::from),
            build_path: None,
            unrendered_config: dbt_artifact_parser::manifest::udf::UnrenderedConfig(
                serde_json::Value::Object(serde_json::Map::default()),
            ),
            created_at: 1.0,
            config_call_dict: None,
            unrendered_config_call_dict: None,
            relation_name: None,
            raw_code: raw_code.to_string(),
            doc_blocks: vec![],
            language: "sql".to_string(),
            refs: vec![],
            sources: vec![],
            metrics: vec![],
            functions: vec![],
            depends_on: DependsOn {
                macros: None,
                nodes,
            },
            compiled_path: None,
            compiled: None,
            compiled_code: None,
            extra_ctes_injected: None,
            extra_ctes: vec![],
            pre_injected_sql: None,
            contract: None,
            arguments: vec![],
        }
    }

    #[test]
    fn test_ruletarget() {
        let f = make_udf(None, "", None, "", None);
        assert_eq!(f.ruletarget(), RuleTarget::Functions);
    }

    #[test]
    fn test_include_excludable() {
        let f = make_udf(None, "", None, "", None);
        assert_eq!(
            IncludeExcludable::get_original_file_path(&f),
            "functions/my_func.sql"
        );
    }

    #[test]
    fn test_identifiable_object_type() {
        let f = make_udf(None, "", None, "", None);
        assert_eq!(Identifiable::get_object_type(&f), "Function");
    }

    #[test]
    fn test_identifiable_object_string() {
        let f = make_udf(None, "", None, "", None);
        assert_eq!(f.get_object_string(), "my_func");
    }

    #[test]
    fn test_problematic_path_prefer_sql() {
        let f = make_udf(
            Some("my_project://functions/schema.yml"),
            "",
            None,
            "",
            None,
        );
        assert_eq!(f.get_problematic_path(true), Some("functions/my_func.sql"));
    }

    #[test]
    fn test_problematic_path_with_patch() {
        let f = make_udf(
            Some("my_project://functions/schema.yml"),
            "",
            None,
            "",
            None,
        );
        assert_eq!(f.get_problematic_path(false), Some("functions/schema.yml"));
    }

    #[test]
    fn test_problematic_path_without_patch() {
        let f = make_udf(None, "", None, "", None);
        assert_eq!(f.get_problematic_path(false), Some("functions/my_func.sql"));
    }

    #[test]
    fn test_description_some() {
        let f = make_udf(None, "A function", None, "", None);
        assert_eq!(f.description().unwrap(), "A function");
    }

    #[test]
    fn test_description_none() {
        let f = make_udf(None, "", None, "", None);
        assert!(f.description().is_none());
    }

    #[test]
    fn test_name() {
        let f = make_udf(None, "", None, "", None);
        assert_eq!(NameAble::name(&f), "my_func");
    }

    #[test]
    fn test_tags_some() {
        let f = make_udf(None, "", Some(vec!["tag1".to_string()]), "", None);
        let tags = f.get_tags().unwrap();
        assert_eq!(tags.len(), 1);
    }

    #[test]
    fn test_tags_none() {
        let f = make_udf(None, "", None, "", None);
        assert!(f.get_tags().is_none());
    }

    #[test]
    fn test_metadata() {
        let f = make_udf(None, "", None, "", None);
        assert!(f.get_metadata().is_some());
    }

    #[test]
    fn test_has_code_some() {
        let f = make_udf(None, "", None, "SELECT 1", None);
        assert_eq!(f.get_raw_code(), Some("SELECT 1"));
    }

    #[test]
    fn test_has_code_empty() {
        let f = make_udf(None, "", None, "", None);
        assert!(f.get_raw_code().is_none());
    }

    #[test]
    fn test_depends_on_some() {
        let f = make_udf(
            None,
            "",
            None,
            "",
            Some(vec!["model.my_project.users".to_string()]),
        );
        assert_eq!(f.get_depends_on_nodes().len(), 1);
    }

    #[test]
    fn test_depends_on_none() {
        let f = make_udf(None, "", None, "", None);
        assert!(f.get_depends_on_nodes().is_empty());
    }

    #[test]
    fn test_path_checkable() {
        let f = make_udf(None, "", None, "", None);
        assert_eq!(f.get_rule_target(), RuleTarget::Functions);
    }
}
