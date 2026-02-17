// Trait implementations for CatalogSource that stay in dbtective
use crate::core::rules::common_traits::{Columnable, Identifiable};
use dbt_artifact_parser::catalog::CatalogSource;

impl Identifiable for CatalogSource {
    fn get_object_type(&self) -> &str {
        Self::get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_name()
    }

    // Paths are only available in manifest objects
    fn get_problematic_path(&self, __prefer_sql: bool) -> Option<&str> {
        None
    }
}

impl Columnable for CatalogSource {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        self.columns.keys().collect::<Vec<&String>>().into()
    }

    // Column descriptions are not available in the catalog.
    // Find them by corresponding with the unique_id to the manifest.
    fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
        None
    }

    fn get_columns_with_types(&self) -> Option<Vec<(&String, &String)>> {
        self.columns
            .iter()
            .map(|(name, col)| (name, &col.type_))
            .collect::<Vec<(&String, &String)>>()
            .into()
    }
}
