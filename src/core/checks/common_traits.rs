// Contains common traits for both manifest and catalog objects
// Multiple object types can have descriptions, tags, columns, etc.
// Define traits for these common properties here.

#[allow(dead_code)]
pub trait Columnable {
    fn get_columns(&self) -> Vec<String>;
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String> {
        None
    }
}
