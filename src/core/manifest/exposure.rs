use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct ExposureOwner {
//     pub email: Option<String>,
//     pub name: Option<String>,
// }

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// pub struct ExposureDependsOn {
//     pub macros: Vec<String>,
//     pub nodes: Vec<String>,
// }

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Exposure {
    pub name: String,
    // pub resource_type: String,
    // pub package_name: String,
    // pub path: String,
    // pub original_file_path: String,
    // pub unique_id: String,
    // pub fqn: Vec<String>,
    // #[serde(rename = "type")]
    // pub exposure_type: String,
    // pub owner: ExposureOwner,
    pub description: Option<String>,
    // pub label: Option<String>,
    // pub maturity: Option<String>,
    // pub meta: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    // pub config: Option<serde_json::Value>,
    // pub unrendered_config: Option<serde_json::Value>,
    // pub url: Option<String>,
    // pub depends_on: Option<ExposureDependsOn>,
    // pub refs: Option<Vec<serde_json::Value>>,
    // pub sources: Option<Vec<serde_json::Value>>,
    // pub metrics: Option<Vec<serde_json::Value>>,
    // pub created_at: Option<f64>,
}
