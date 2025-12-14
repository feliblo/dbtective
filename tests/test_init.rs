use dbtective::cli::commands::InitOptions;
use dbtective::core::config::parse_config::Config;
use dbtective::core::init::{create_config, InitResult};
use std::fs;
use tempfile::TempDir;

fn default_options(temp_dir: &TempDir) -> InitOptions {
    InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    }
}

#[test]
fn test_init_creates_yaml_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);

    let result = create_config(&options);
    assert!(matches!(result, InitResult::Created(_)));

    let config_path = temp_dir.path().join("dbtective.yml");
    assert!(config_path.exists());
}

#[test]
fn test_init_yaml_contains_required_rules() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);

    create_config(&options);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("manifest_tests:"));
    assert!(content.contains("has_description"));
    assert!(content.contains("name_convention"));
    assert!(content.contains("snake_case"));
    assert!(content.contains("has_metadata_keys"));
    assert!(content.contains("owner"));
}

#[test]
fn test_init_yaml_contains_commented_examples() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);

    create_config(&options);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("# applies_to:"));
    assert!(content.contains("# includes:"));
    assert!(content.contains("# excludes:"));
}

#[test]
fn test_init_yaml_is_valid_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);

    create_config(&options);

    let config_path = temp_dir.path().join("dbtective.yml");
    let config = Config::from_file(&config_path);
    assert!(config.is_ok(), "Generated YAML config should be valid");

    let config = config.unwrap();
    let manifest_tests = config.manifest_tests.expect("manifest_tests should exist");
    assert_eq!(manifest_tests.len(), 4, "Should have 4 default rules");
}

// ===== TOML CONFIG TESTS =====

#[test]
fn test_init_creates_toml_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "toml".to_string(),
    };

    let result = create_config(&options);
    assert!(matches!(result, InitResult::Created(_)));

    let config_path = temp_dir.path().join("dbtective.toml");
    assert!(config_path.exists());
}

#[test]
fn test_init_toml_is_valid_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "toml".to_string(),
    };

    create_config(&options);

    let config_path = temp_dir.path().join("dbtective.toml");
    let config = Config::from_file(&config_path);
    assert!(config.is_ok(), "Generated TOML config should be valid");

    let config = config.unwrap();
    let manifest_tests = config.manifest_tests.expect("manifest_tests should exist");
    assert_eq!(manifest_tests.len(), 4, "Should have 4 default rules");
}

// ===== PYPROJECT.TOML TESTS =====
#[test]
fn test_init_pyproject_contains_tool_section() {
    let temp_dir = TempDir::new().unwrap();
    let pyproject_path = temp_dir.path().join("pyproject.toml");

    // Create an existing pyproject.toml first
    let existing_content = r#"[project]
name = "test-project"
version = "0.1.0"
"#;
    fs::write(&pyproject_path, existing_content).unwrap();

    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "pyproject".to_string(),
    };

    create_config(&options);

    let content = fs::read_to_string(&pyproject_path).unwrap();

    assert!(content.contains("[tool.dbtective]"));
    assert!(content.contains("[[tool.dbtective.manifest_tests]]"));
}

#[test]
fn test_init_updates_existing_pyproject() {
    let temp_dir = TempDir::new().unwrap();
    let pyproject_path = temp_dir.path().join("pyproject.toml");

    let existing_content = r#"[project]
name = "my-dbt-project"
version = "1.0.0"

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
    assert!(
        content.contains("[project]"),
        "Should preserve existing content"
    );
    assert!(
        content.contains("my-dbt-project"),
        "Should preserve existing content"
    );
    assert!(
        content.contains("[tool.dbtective]"),
        "Should add dbtective section"
    );
}

#[test]
fn test_init_pyproject_already_configured() {
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

// ===== ALREADY EXISTS TESTS =====

#[test]
fn test_init_does_not_overwrite_existing_yml() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("dbtective.yml");
    fs::write(&config_path, "existing content").unwrap();

    let options = default_options(&temp_dir);

    let result = create_config(&options);
    assert!(matches!(result, InitResult::AlreadyExists(_)));

    let content = fs::read_to_string(&config_path).unwrap();
    assert_eq!(content, "existing content", "Should not overwrite");
}

#[test]
fn test_init_does_not_overwrite_existing_toml() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("dbtective.toml");
    fs::write(&config_path, "existing content").unwrap();

    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "toml".to_string(),
    };

    let result = create_config(&options);
    assert!(matches!(result, InitResult::AlreadyExists(_)));
}

// ===== ERROR HANDLING TESTS =====

#[test]
fn test_init_fails_for_nonexistent_directory() {
    let options = InitOptions {
        location: "/nonexistent/path/that/does/not/exist".to_string(),
        format: "yml".to_string(),
    };

    let result = create_config(&options);
    assert!(matches!(result, InitResult::Error(_)));

    if let InitResult::Error(msg) = result {
        assert!(msg.contains("does not exist"));
    }
}

#[test]
fn test_init_fails_when_path_is_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("somefile.txt");
    fs::write(&file_path, "content").unwrap();

    let options = InitOptions {
        location: file_path.to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let result = create_config(&options);
    assert!(matches!(result, InitResult::Error(_)));

    if let InitResult::Error(msg) = result {
        assert!(msg.contains("not a directory"));
    }
}

// ===== FORMAT ALIAS TESTS =====

#[test]
fn test_init_yaml_alias_creates_yml_file() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yaml".to_string(),
    };

    let result = create_config(&options);
    assert!(matches!(result, InitResult::Created(_)));

    let config_path = temp_dir.path().join("dbtective.yml");
    assert!(
        config_path.exists(),
        "Should create .yml file even with yaml format"
    );
}
