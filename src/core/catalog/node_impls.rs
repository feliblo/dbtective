// Trait implementations for CatalogNode that stay in dbtective
use crate::core::rules::common_traits::Columnable;
use dbt_artifact_parser::catalog::CatalogNode;

impl Columnable for CatalogNode {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        self.get_base()
            .columns
            .keys()
            .collect::<Vec<&String>>()
            .into()
    }

    // Column descriptions are not available in the catalog.
    // Find them by corresponding with the unique_id to the manifest.
    fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
        None
    }

    fn get_columns_with_types(&self) -> Option<Vec<(&String, &String)>> {
        self.get_base()
            .columns
            .iter()
            .map(|(name, col)| (name, &col.type_))
            .collect::<Vec<(&String, &String)>>()
            .into()
    }

    fn get_object_type(&self) -> &str {
        self.get_object_type()
    }

    fn get_object_string(&self) -> &str {
        self.get_object_string()
    }

    // Paths are only available in manifest objects
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}

impl Columnable for &CatalogNode {
    fn get_column_names(&self) -> Option<Vec<&String>> {
        (*self).get_column_names()
    }

    fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>> {
        (*self).get_columns_with_descriptions()
    }

    fn get_columns_with_types(&self) -> Option<Vec<(&String, &String)>> {
        (*self).get_columns_with_types()
    }

    fn get_object_type(&self) -> &str {
        (*self).get_object_type()
    }

    fn get_object_string(&self) -> &str {
        (*self).get_object_string()
    }

    // Paths are only available in manifest objects
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}
