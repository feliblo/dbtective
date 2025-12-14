use crate::cli::commands::InitOptions;
use log::debug;
use owo_colors::OwoColorize;
use std::fs;
use std::path::Path;

const DEFAULT_YAML_CONFIG: &str = r#"# dbtective configuration file
# Documentation: https://feliblo.github.io/dbtective/docs/config

manifest_tests:
  - name: "has_description"
    type: "has_description"
    # severity: "warning"
    # applies_to: ["models", "sources"]
    # includes: ["models/staging/**"]
    # excludes: ["models/deprecated/**"]

  - name: "naming_convention"
    type: "name_convention"
    pattern: "snake_case"

  - name: "require_execution_tags"
    type: "has_tags"
    required_tags: ["daily", "monthly", "yearly", "inactive"]
    criteria: "one_of"
    description: "Resources need to have at least one of the required tags. To decide when a resource should be run."

  - name: "has_owner"
    type: "has_metadata_keys"
    required_keys: ["owner"]

  - name: "refs_must_be_used"
    type: "has_refs"

catalog_tests:
  - name: "all_columns_documented"
    type: "columns_all_documented"
    description: "All columns must exist in the documentation."
    severity: "warning"

  - name: "all_columns_described"
    type: "columns_have_description"
    description: "All columns must have a description."
"#;

const DEFAULT_TOML_CONFIG: &str = r#"# dbtective configuration file
# Documentation: https://feliblo.github.io/dbtective/docs/config

[[manifest_tests]]
name = "has_description"
type = "has_description"
# severity = "warning"
# applies_to = ["models", "sources"]
# includes = ["models/staging/**"]
# excludes = ["models/deprecated/**"]

[[manifest_tests]]
name = "naming_convention"
type = "name_convention"
pattern = "snake_case"

[[manifest_tests]]
name = "has_owner"
type = "has_metadata_keys"
required_keys = ["owner"]

[[manifest_tests]]
name = "require_execution_tags"
type = "has_tags"
required_tags = ["daily", "monthly", "yearly", "inactive"]
criteria = "one_of"
description = "Resources need to have at least one of the required tags. To decide when a resource should be run."

[[manifest_tests]]
name = "refs_must_be_used"
type = "has_refs"

[[catalog_tests]]
name = "all_columns_documented"
type = "columns_all_documented"
description = "All columns must exist in the documentation."

[[catalog_tests]]
name = "all_columns_described"
type = "columns_have_description"
description = "All columns must have a description."
"#;

const DEFAULT_PYPROJECT_CONFIG: &str = r#"
# dbtective configuration
# Documentation: https://feliblo.github.io/dbtective/docs/config

[tool.dbtective]

[[tool.dbtective.manifest_tests]]
name = "has_description"
type = "has_description"
# applies_to = ["models", "sources"]
# includes = ["models/staging/**"]
# excludes = ["models/deprecated/**"]

[[tool.dbtective.manifest_tests]]
name = "naming_convention"
type = "name_convention"
pattern = "snake_case"

[[tool.dbtective.manifest_tests]]
name = "has_owner"
type = "has_metadata_keys"
required_keys = ["owner"]

[[tool.dbtective.manifest_tests]]
name = "require_execution_tags"
type = "has_tags"
required_tags = ["daily", "monthly", "yearly", "inactive"]
criteria = "one_of"
description = "Resources need to have at least one of the required tags. To decide when a resource should be run."

[[tool.dbtective.manifest_tests]]
name = "refs_must_be_used"
type = "has_refs"

[[tool.dbtective.catalog_tests]]
name = "all_columns_documented"
type = "columns_all_documented"
description = "All columns must exist in the documentation."

[[tool.dbtective.catalog_tests]]
name = "all_columns_described"
type = "columns_have_description"
description = "All columns must have a description."
"#;

#[derive(Debug, PartialEq, Eq)]
pub enum InitResult {
    Created(String),
    AlreadyExists(String),
    PyprojectUpdated(String),
    PyprojectAlreadyConfigured(String),
    Error(String),
}

pub fn init(options: &InitOptions, verbose: bool) -> i32 {
    if verbose {
        debug!("Init options: {options:#?}");
    }

    let result = create_config(options);

    match result {
        InitResult::Created(path) => {
            println!(
                "{} Created configuration file: {}",
                "✓".green().bold(),
                path.cyan()
            );
            println!("\nNext steps:");
            println!("  1. Review and customize the rules in {}", path.cyan());
            println!(
                "  2. (optional) Run {} to compile your dbt project manifest (this is also done automatically on most dbt commands, see \x1b]8;;https://docs.getdbt.com/reference/artifacts/manifest-json\x1b\\docs\x1b]8;;\x1b\\)",
                "dbt compile".yellow(),
            );
            println!(
                "  3. (optional) Run {} to compile your dbt project catalog",
                "dbt docs generate".yellow()
            );
            println!(
                "  4. Run {} to inspect your project",
                "dbtective run".yellow()
            );
            0
        }
        InitResult::PyprojectUpdated(path) => {
            println!(
                "{} Added dbtective configuration to: {}",
                "✓".green().bold(),
                path.cyan()
            );
            println!("\nNext steps:");
            println!(
                "  1. Review and customize the [tool.dbtective] section in {}",
                path.cyan()
            );
            println!(
                "  2. (optional) Run {} to compile your dbt project manifest (this is also done automatically on most dbt commands, see \x1b]8;;https://docs.getdbt.com/reference/artifacts/manifest-json\x1b\\docs\x1b]8;;\x1b\\)",
                "dbt compile".yellow(),
            );
            println!(
                "  3. (optional) Run {} to compile your dbt project catalog",
                "dbt docs generate".yellow()
            );
            println!(
                "  4. Run {} to inspect your project",
                "dbtective run".yellow()
            );
            0
        }
        InitResult::AlreadyExists(path) => {
            println!(
                "{} Configuration file already exists: {}, no changes made.",
                "!".yellow().bold(),
                path.cyan()
            );
            0
        }
        InitResult::PyprojectAlreadyConfigured(path) => {
            println!(
                "{} pyproject.toml already contains dbtective configuration: {}. No changes made.",
                "!".yellow().bold(),
                path.cyan()
            );
            0
        }
        InitResult::Error(msg) => {
            eprintln!("{} Error: {}", "✗".red().bold(), msg);
            1
        }
    }
}

pub fn create_config(options: &InitOptions) -> InitResult {
    let location = Path::new(&options.location);

    if !location.exists() {
        return InitResult::Error(format!("Directory does not exist: {}", options.location));
    }

    if !location.is_dir() {
        return InitResult::Error(format!("Path is not a directory: {}", options.location));
    }

    match options.format.as_str() {
        "yml" | "yaml" => create_yaml_config(location),
        "toml" => create_toml_config(location),
        "pyproject" => create_or_update_pyproject(location),
        _ => InitResult::Error(format!("Unknown format: {}", options.format)),
    }
}

fn create_yaml_config(location: &Path) -> InitResult {
    let file_path = location.join("dbtective.yml");
    let path_str = file_path.display().to_string();

    if file_path.exists() {
        return InitResult::AlreadyExists(path_str);
    }

    match fs::write(&file_path, DEFAULT_YAML_CONFIG) {
        Ok(()) => InitResult::Created(path_str),
        Err(e) => InitResult::Error(format!("Failed to write {path_str}: {e}")),
    }
}

fn create_toml_config(location: &Path) -> InitResult {
    let file_path = location.join("dbtective.toml");
    let path_str = file_path.display().to_string();

    if file_path.exists() {
        return InitResult::AlreadyExists(path_str);
    }

    match fs::write(&file_path, DEFAULT_TOML_CONFIG) {
        Ok(()) => InitResult::Created(path_str),
        Err(e) => InitResult::Error(format!("Failed to write {path_str}: {e}")),
    }
}

fn create_or_update_pyproject(location: &Path) -> InitResult {
    let file_path = location.join("pyproject.toml");
    let path_str = file_path.display().to_string();

    if !file_path.exists() {
        return InitResult::Error(format!(
            "pyproject.toml does not exist at: {path_str}. Unable to add dbtective configuration section."
        ));
    }

    let existing_content = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => return InitResult::Error(format!("Failed to read {path_str}: {e}")),
    };

    if existing_content.contains("[tool.dbtective]") {
        return InitResult::PyprojectAlreadyConfigured(path_str);
    }

    // Append dbtective section to existing pyproject.toml
    let new_content = format!(
        "{}{}",
        existing_content.trim_end(),
        DEFAULT_PYPROJECT_CONFIG
    );

    match fs::write(&file_path, new_content) {
        Ok(()) => InitResult::PyprojectUpdated(path_str),
        Err(e) => InitResult::Error(format!("Failed to write {path_str}: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn default_options(temp_dir: &TempDir) -> InitOptions {
        InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: "yml".to_string(),
        }
    }

    #[test]
    fn test_create_yaml_config() {
        let temp_dir = TempDir::new().unwrap();
        let options = default_options(&temp_dir);

        let result = create_config(&options);
        assert!(matches!(result, InitResult::Created(_)));

        let config_path = temp_dir.path().join("dbtective.yml");
        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("manifest_tests:"));
        assert!(content.contains("has_description"));
        assert!(content.contains("name_convention"));
        assert!(content.contains("has_metadata_keys"));
    }

    #[test]
    fn test_create_toml_config() {
        let temp_dir = TempDir::new().unwrap();
        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: "toml".to_string(),
        };

        let result = create_config(&options);
        assert!(matches!(result, InitResult::Created(_)));

        let config_path = temp_dir.path().join("dbtective.toml");
        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("[[manifest_tests]]"));
        assert!(content.contains("has_description"));
        assert!(content.contains("name_convention"));
    }

    #[test]
    fn test_dont_create_new_pyproject() {
        let temp_dir = TempDir::new().unwrap();
        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: "pyproject".to_string(),
        };

        let result = create_config(&options);
        assert!(matches!(result, InitResult::Error(_)));

        let config_path = temp_dir.path().join("pyproject.toml");
        assert!(!config_path.exists());
    }

    #[test]
    fn test_update_existing_pyproject() {
        let temp_dir = TempDir::new().unwrap();
        let pyproject_path = temp_dir.path().join("pyproject.toml");

        let existing_content = r#"[project]
name = "my-project"
version = "0.1.0"

[build-system]
requires = ["setuptools"]
"#;
        fs::write(&pyproject_path, existing_content).unwrap();

        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: "pyproject".to_string(),
        };

        let result = create_config(&options);
        assert!(matches!(result, InitResult::PyprojectUpdated(_)));

        let content = fs::read_to_string(&pyproject_path).unwrap();
        assert!(content.contains("[project]"));
        assert!(content.contains("[tool.dbtective]"));
        assert!(content.contains("[[tool.dbtective.manifest_tests]]"));
    }

    #[test]
    fn test_pyproject_already_configured() {
        let temp_dir = TempDir::new().unwrap();
        let pyproject_path = temp_dir.path().join("pyproject.toml");

        let existing_content = r#"[project]
name = "my-project"

[tool.dbtective]
# existing config
"#;
        fs::write(&pyproject_path, existing_content).unwrap();

        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: "pyproject".to_string(),
        };

        let result = create_config(&options);
        assert!(matches!(result, InitResult::PyprojectAlreadyConfigured(_)));
    }

    #[test]
    fn test_already_exists_does_not_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("dbtective.yml");
        fs::write(&config_path, "existing content").unwrap();

        let options = default_options(&temp_dir);

        let result = create_config(&options);
        assert!(matches!(result, InitResult::AlreadyExists(_)));

        let content = fs::read_to_string(&config_path).unwrap();
        assert_eq!(content, "existing content");
    }

    #[test]
    fn test_invalid_directory() {
        let options = InitOptions {
            location: "/nonexistent/path".to_string(),
            format: "yml".to_string(),
        };

        let result = create_config(&options);
        assert!(matches!(result, InitResult::Error(_)));
    }

    #[test]
    fn test_path_is_file_not_directory() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("somefile.txt");
        fs::write(&file_path, "content").unwrap();

        let options = InitOptions {
            location: file_path.to_string_lossy().to_string(),
            format: "yml".to_string(),
        };

        let result = create_config(&options);
        assert!(matches!(result, InitResult::Error(_)));
    }

    #[test]
    fn test_yaml_format_alias() {
        let temp_dir = TempDir::new().unwrap();
        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: "yaml".to_string(),
        };

        let result = create_config(&options);
        assert!(matches!(result, InitResult::Created(_)));

        let config_path = temp_dir.path().join("dbtective.yml");
        assert!(config_path.exists());
    }
}
