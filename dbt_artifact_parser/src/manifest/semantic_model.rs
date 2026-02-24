use serde::Deserialize;

use super::dbt_objects::Meta;

#[derive(Debug, Deserialize)]
pub struct SemanticModelDependsOn {
    #[allow(dead_code)]
    pub macros: Option<Vec<String>>,
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SemanticModel {
    pub name: String,
    pub package_name: String,
    pub original_file_path: String,
    pub patch_path: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<Meta>,
    pub depends_on: SemanticModelDependsOn,
}

impl SemanticModel {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    pub const fn get_package_name(&self) -> &String {
        &self.package_name
    }

    pub const fn get_object_type() -> &'static str {
        "SemanticModel"
    }

    pub const fn get_original_file_path(&self) -> &String {
        &self.original_file_path
    }

    pub fn get_patch_path(&self) -> Option<&str> {
        self.patch_path
            .as_deref()
            .and_then(super::strip_patch_path_prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_semantic_model(patch_path: Option<&str>) -> SemanticModel {
        SemanticModel {
            name: "orders_sm".to_string(),
            package_name: "my_project".to_string(),
            original_file_path: "models/semantic_models/orders.yml".to_string(),
            patch_path: patch_path.map(String::from),
            description: Some("Orders semantic model".to_string()),
            metadata: None,
            depends_on: SemanticModelDependsOn {
                macros: None,
                nodes: Some(vec!["model.my_project.orders".to_string()]),
            },
        }
    }

    #[test]
    fn test_get_name() {
        let sm = make_semantic_model(None);
        assert_eq!(sm.get_name(), "orders_sm");
    }

    #[test]
    fn test_get_package_name() {
        let sm = make_semantic_model(None);
        assert_eq!(sm.get_package_name(), "my_project");
    }

    #[test]
    fn test_get_object_type() {
        assert_eq!(SemanticModel::get_object_type(), "SemanticModel");
    }

    #[test]
    fn test_get_original_file_path() {
        let sm = make_semantic_model(None);
        assert_eq!(
            sm.get_original_file_path(),
            "models/semantic_models/orders.yml"
        );
    }

    #[test]
    fn test_get_patch_path_none() {
        let sm = make_semantic_model(None);
        assert_eq!(sm.get_patch_path(), None);
    }

    #[test]
    fn test_get_patch_path_with_prefix() {
        let sm = make_semantic_model(Some("my_project://models/semantic_models/orders.yml"));
        assert_eq!(
            sm.get_patch_path(),
            Some("models/semantic_models/orders.yml")
        );
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "name": "revenue_sm",
            "package_name": "analytics",
            "original_file_path": "models/semantic/revenue.yml",
            "patch_path": null,
            "description": "Revenue semantic model",
            "metadata": {},
            "depends_on": {
                "macros": [],
                "nodes": ["model.analytics.fct_revenue"]
            }
        }"#;
        let sm: SemanticModel = serde_json::from_str(json).unwrap();
        assert_eq!(sm.get_name(), "revenue_sm");
        assert_eq!(sm.get_package_name(), "analytics");
        assert_eq!(sm.depends_on.nodes.unwrap().len(), 1);
    }
}
