use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CatalogColumn {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub index: i32,
    pub comment: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_catalog_column() {
        let json_str = r#"
        {
            "name": "id",
            "type": "INTEGER",
            "index": 0,
            "comment": "Primary key"
        }
        "#;

        let column: CatalogColumn = serde_json::from_str(json_str).unwrap();
        assert_eq!(column.name, "id");
        assert_eq!(column.type_, "INTEGER");
        assert_eq!(column.index, 0);
        assert_eq!(column.comment.unwrap(), "Primary key");
    }
}
