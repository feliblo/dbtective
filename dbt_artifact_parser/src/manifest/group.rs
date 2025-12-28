use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GroupOwner {
    pub email: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Group {
    pub name: String,
    pub resource_type: String,
    pub package_name: String,
    pub path: String,
    pub original_file_path: String,
    pub unique_id: String,
    pub owner: GroupOwner,
    pub description: Option<String>,
    pub config: Option<serde_json::Value>,
}
