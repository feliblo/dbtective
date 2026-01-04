// Contains common traits for both manifest and catalog objects
// Multiple object types can have descriptions, tags, columns, etc.
// Define traits for these common properties here.

/// Base trait for all objects that can be identified in rule results.
/// Provides common methods for object identification and error reporting.
pub trait Identifiable {
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}

#[allow(dead_code)]
pub trait Columnable: Identifiable {
    // Returns a vector of column names
    fn get_column_names(&self) -> Option<Vec<&String>>;
    // Returns a vector of tuples containing column names and their descriptions
    fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>>;
    // Returns a vector of tuples containing column names and their types
    fn get_columns_with_types(&self) -> Option<Vec<(&String, &String)>>;
}
