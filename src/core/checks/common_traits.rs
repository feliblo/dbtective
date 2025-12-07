// Contains common traits for both manifest and catalog objects
// Multiple object types can have descriptions, tags, columns, etc.
// Define traits for these common properties here.

#[allow(dead_code)]
pub trait Columnable {
    // Returns a vector of column names
    fn get_column_names(&self) -> Option<Vec<&String>>;
    // Returns a vector of tuples containing column names and their descriptions
    fn get_columns_with_descriptions(&self) -> Option<Vec<(&String, &String)>>;
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}
