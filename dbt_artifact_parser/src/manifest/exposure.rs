use serde::Deserialize;

use super::dbt_objects::{Meta, Tags};

#[derive(Debug, Deserialize)]
pub struct ExposureDependsOn {
    #[allow(dead_code)]
    pub macros: Option<Vec<String>>,
    pub nodes: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Exposure {
    pub name: String,
    pub package_name: String,
    pub original_file_path: String,
    pub patch_path: Option<String>,
    pub description: Option<String>,
    pub meta: Option<Meta>,
    pub tags: Option<Tags>,
    pub depends_on: ExposureDependsOn,
}

impl Exposure {
    pub const fn get_name(&self) -> &String {
        &self.name
    }

    pub const fn get_package_name(&self) -> &String {
        &self.package_name
    }

    pub const fn get_object_type() -> &'static str {
        "Exposure"
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

    fn make_exposure(patch_path: Option<&str>) -> Exposure {
        Exposure {
            name: "my_exposure".to_string(),
            package_name: "my_project".to_string(),
            original_file_path: "models/exposures/my_exposure.yml".to_string(),
            patch_path: patch_path.map(String::from),
            description: Some("An exposure".to_string()),
            meta: None,
            tags: None,
            depends_on: ExposureDependsOn {
                macros: None,
                nodes: Some(vec!["model.my_project.users".to_string()]),
            },
        }
    }

    #[test]
    fn test_get_name() {
        let e = make_exposure(None);
        assert_eq!(e.get_name(), "my_exposure");
    }

    #[test]
    fn test_get_package_name() {
        let e = make_exposure(None);
        assert_eq!(e.get_package_name(), "my_project");
    }

    #[test]
    fn test_get_object_type() {
        assert_eq!(Exposure::get_object_type(), "Exposure");
    }

    #[test]
    fn test_get_original_file_path() {
        let e = make_exposure(None);
        assert_eq!(
            e.get_original_file_path(),
            "models/exposures/my_exposure.yml"
        );
    }

    #[test]
    fn test_get_patch_path_none() {
        let e = make_exposure(None);
        assert_eq!(e.get_patch_path(), None);
    }

    #[test]
    fn test_get_patch_path_with_prefix() {
        let e = make_exposure(Some("my_project://models/exposures/my_exposure.yml"));
        assert_eq!(e.get_patch_path(), Some("models/exposures/my_exposure.yml"));
    }

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "name": "weekly_report",
            "package_name": "analytics",
            "original_file_path": "models/exposures.yml",
            "patch_path": null,
            "description": "Weekly metrics report",
            "meta": {},
            "tags": ["reporting"],
            "depends_on": {
                "macros": [],
                "nodes": ["model.analytics.dim_users"]
            }
        }"#;
        let e: Exposure = serde_json::from_str(json).unwrap();
        assert_eq!(e.get_name(), "weekly_report");
        assert_eq!(e.get_package_name(), "analytics");
        assert_eq!(e.depends_on.nodes.unwrap().len(), 1);
    }
}
