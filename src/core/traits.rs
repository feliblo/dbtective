pub trait Descriptable {
    fn description(&self) -> Option<&String>;
    fn original_file_path(&self) -> &String;
    fn ruletarget(&self) -> crate::core::config::RuleTarget;
}
