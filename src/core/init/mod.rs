pub mod config_builder;
pub mod questionnaire;

use crate::cli::commands::InitOptions;
use config_builder::InitConfig;
use inquire::Confirm;
use owo_colors::OwoColorize;
use questionnaire::run_questionnaire;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum InitResult {
    Created(String),
    AlreadyExists(String),
    PyprojectUpdated(String),
    PyprojectAlreadyConfigured(String),
    Error(String),
}

pub fn init(options: &InitOptions, verbose: bool) -> i32 {
    init_impl(options, verbose, None)
}

/// Test helper that allows injecting a questionnaire result
#[doc(hidden)]
#[allow(dead_code)]
pub fn init_with_result(
    options: &InitOptions,
    verbose: bool,
    questionnaire_result: questionnaire::QuestionnaireResult,
) -> i32 {
    init_impl(options, verbose, Some(questionnaire_result))
}

#[allow(clippy::too_many_lines)]
fn init_impl(
    options: &InitOptions,
    _verbose: bool,
    test_questionnaire_result: Option<questionnaire::QuestionnaireResult>,
) -> i32 {
    // Run interactive questionnaire if using defaults
    let config = if let Some(result) = test_questionnaire_result {
        // Test mode: use injected result
        InitConfig::from_questionnaire(&result)
    } else if options.format == "yml" && options.location == "." {
        match run_questionnaire() {
            Ok(result) => InitConfig::from_questionnaire(&result),
            Err(e) => {
                eprintln!("{} {}", "✗".red().bold(), e);
                return 1;
            }
        }
    } else {
        // Use provided options for non-interactive mode
        // For now, just create a minimal config
        let result = questionnaire::QuestionnaireResult {
            format: match options.format.as_str() {
                "toml" => questionnaire::ConfigFormat::Toml,
                "pyproject" => questionnaire::ConfigFormat::Pyproject,
                _ => questionnaire::ConfigFormat::Yaml,
            },
            naming_convention: crate::core::config::naming_convention::NamingConvention::default(),
            data_model: questionnaire::DataModel::None,
            manifest_rules: Vec::new(),
            catalog_rules: Vec::new(),
        };
        InitConfig::from_questionnaire(&result)
    };

    // Try without overwrite first; if the file already exists, prompt interactively
    let result = create_config(options, &config, false);
    let result = match result {
        InitResult::AlreadyExists(ref path) => {
            let should_overwrite = Confirm::new(&format!(
                "Configuration file {} already exists. Overwrite?",
                path.as_str().bright_cyan()
            ))
            .with_default(false)
            .prompt();

            match should_overwrite {
                Ok(true) => create_config(options, &config, true),
                _ => result,
            }
        }
        other => other,
    };

    match result {
        InitResult::Created(path) => {
            println!(
                "\n{} {}",
                "✓".bright_green().bold(),
                format!("Created configuration file: {path}").bright_white()
            );
            println!("\n{}", "Next steps:".bright_blue().bold());
            println!(
                "  {} Review and customize the rules in {}",
                "1.".bright_yellow(),
                path.bright_cyan()
            );
            println!(
                "  {} (optional) Run {} to compile your dbt project manifest",
                "2.".bright_yellow(),
                "dbt compile".bright_magenta(),
            );
            println!(
                "  {} (optional) Run {} to compile your dbt project catalog",
                "3.".bright_yellow(),
                "dbt docs generate".bright_magenta()
            );
            println!(
                "  {} Run {} to inspect your project",
                "4.".bright_yellow(),
                "dbtective run".bright_green().bold()
            );
            0
        }
        InitResult::PyprojectUpdated(path) => {
            println!(
                "\n{} {}",
                "✓".bright_green().bold(),
                format!("Added dbtective configuration to: {path}").bright_white()
            );
            println!("\n{}", "Next steps:".bright_blue().bold());
            println!(
                "  {} Review and customize the [tool.dbtective] section in {}",
                "1.".bright_yellow(),
                path.bright_cyan()
            );
            println!(
                "  {} (optional) Run {} to compile your dbt project manifest",
                "2.".bright_yellow(),
                "dbt compile".bright_magenta(),
            );
            println!(
                "  {} (optional) Run {} to compile your dbt project catalog",
                "3.".bright_yellow(),
                "dbt docs generate".bright_magenta()
            );
            println!(
                "  {} Run {} to inspect your project",
                "4.".bright_yellow(),
                "dbtective run".bright_green().bold()
            );
            0
        }
        InitResult::AlreadyExists(path) => {
            println!(
                "\n{} Configuration file already exists: {}",
                "ℹ".bright_blue().bold(),
                path.bright_cyan()
            );
            println!("{}", "No changes made.".bright_white());
            0
        }
        InitResult::PyprojectAlreadyConfigured(path) => {
            println!(
                "\n{} pyproject.toml already contains dbtective configuration: {}",
                "ℹ".bright_blue().bold(),
                path.bright_cyan()
            );
            println!("{}", "No changes made.".bright_white());
            0
        }
        InitResult::Error(msg) => {
            eprintln!("{} Error: {}", "✗".red().bold(), msg);
            1
        }
    }
}

pub fn create_config(options: &InitOptions, config: &InitConfig, overwrite: bool) -> InitResult {
    let location = Path::new(&options.location);

    if !location.exists() {
        return InitResult::Error(format!("Directory does not exist: {}", options.location));
    }

    if !location.is_dir() {
        return InitResult::Error(format!("Path is not a directory: {}", options.location));
    }

    match options.format.as_str() {
        "yml" | "yaml" => create_yaml_config(location, config, overwrite),
        "toml" => create_toml_config(location, config, overwrite),
        "pyproject" => create_or_update_pyproject(location, config),
        _ => InitResult::Error(format!("Unknown format: {}", options.format)),
    }
}

fn create_yaml_config(location: &Path, config: &InitConfig, overwrite: bool) -> InitResult {
    let file_path = location.join("dbtective.yml");
    let path_str = file_path.display().to_string();

    if file_path.exists() && !overwrite {
        return InitResult::AlreadyExists(path_str);
    }

    let content = config.to_yaml();

    match fs::write(&file_path, content) {
        Ok(()) => InitResult::Created(path_str),
        Err(e) => InitResult::Error(format!("Failed to write {path_str}: {e}")),
    }
}

fn create_toml_config(location: &Path, config: &InitConfig, overwrite: bool) -> InitResult {
    let file_path = location.join("dbtective.toml");
    let path_str = file_path.display().to_string();

    if file_path.exists() && !overwrite {
        return InitResult::AlreadyExists(path_str);
    }

    let content = config.to_toml();

    match fs::write(&file_path, content) {
        Ok(()) => InitResult::Created(path_str),
        Err(e) => InitResult::Error(format!("Failed to write {path_str}: {e}")),
    }
}

fn create_or_update_pyproject(location: &Path, config: &InitConfig) -> InitResult {
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

    let content = config.to_pyproject();

    // Append dbtective section to existing pyproject.toml
    let new_content = format!("{}{}", existing_content.trim_end(), content);

    match fs::write(&file_path, new_content) {
        Ok(()) => InitResult::PyprojectUpdated(path_str),
        Err(e) => InitResult::Error(format!("Failed to write {path_str}: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::naming_convention::NamingConvention;
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

        // Create a minimal config for testing
        let questionnaire_result = questionnaire::QuestionnaireResult {
            format: questionnaire::ConfigFormat::Yaml,
            naming_convention: crate::core::config::naming_convention::NamingConvention::default(),
            data_model: questionnaire::DataModel::None,
            manifest_rules: vec![
                crate::core::config::manifest_rule::ManifestSpecificRuleConfig::HasDescription { min_length: None, forbidden_substrings: None },
                crate::core::config::manifest_rule::ManifestSpecificRuleConfig::NameConvention { convention: NamingConvention::default() },
            ],
            catalog_rules: vec![
                crate::core::config::catalog_rule::CatalogSpecificRuleConfig::ColumnsNameConvention { convention: NamingConvention::default(), data_types: None, use_database_columns: true },
            ],
        };
        let config = InitConfig::from_questionnaire(&questionnaire_result);

        let result = create_config(&options, &config, false);
        assert!(matches!(result, InitResult::Created(_)));

        let config_path = temp_dir.path().join("dbtective.yml");
        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("manifest_tests:"));
        assert!(content.contains("has_description"));
        assert!(content.contains("name_convention"));
        assert!(content.contains("catalog_tests:"));
    }

    #[test]
    fn test_dont_create_new_pyproject() {
        let temp_dir = TempDir::new().unwrap();
        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: "pyproject".to_string(),
        };
        let questionnaire_result = questionnaire::QuestionnaireResult {
            format: questionnaire::ConfigFormat::Pyproject,
            naming_convention: crate::core::config::naming_convention::NamingConvention::default(),
            data_model: questionnaire::DataModel::None,
            manifest_rules: vec![],
            catalog_rules: vec![],
        };
        let config = InitConfig::from_questionnaire(&questionnaire_result);

        let result = create_config(&options, &config, false);
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
        let questionnaire_result = questionnaire::QuestionnaireResult {
            format: questionnaire::ConfigFormat::Pyproject,
            naming_convention: crate::core::config::naming_convention::NamingConvention::default(),
            data_model: questionnaire::DataModel::None,
            manifest_rules: vec![
                crate::core::config::manifest_rule::ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
            ],
            catalog_rules: vec![],
        };
        let config = InitConfig::from_questionnaire(&questionnaire_result);

        let result = create_config(&options, &config, false);
        assert!(matches!(result, InitResult::PyprojectUpdated(_)));

        let content = fs::read_to_string(&pyproject_path).unwrap();
        assert!(content.contains("[project]"));
        assert!(content.contains("[tool.dbtective]"));
    }

    #[test]
    fn test_already_exists_does_not_overwrite() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("dbtective.yml");
        fs::write(&config_path, "existing content").unwrap();

        let options = default_options(&temp_dir);
        let questionnaire_result = questionnaire::QuestionnaireResult {
            format: questionnaire::ConfigFormat::Yaml,
            naming_convention: crate::core::config::naming_convention::NamingConvention::default(),
            data_model: questionnaire::DataModel::None,
            manifest_rules: vec![],
            catalog_rules: vec![],
        };
        let config = InitConfig::from_questionnaire(&questionnaire_result);

        let result = create_config(&options, &config, false);
        assert!(matches!(result, InitResult::AlreadyExists(_)));

        let content = fs::read_to_string(&config_path).unwrap();
        assert_eq!(content, "existing content");
    }
}
