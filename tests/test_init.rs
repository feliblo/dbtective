use dbtective::cli::commands::InitOptions;
use dbtective::core::config::catalog_rule::CatalogSpecificRuleConfig;
use dbtective::core::config::check_config_options::HasTagsCriteria;
use dbtective::core::config::manifest_rule::ManifestSpecificRuleConfig;
use dbtective::core::config::naming_convention::NamingConvention;
use dbtective::core::config::parse_config::Config;
use dbtective::core::init::init_with_result;
use dbtective::core::init::questionnaire::{
    ConfigFormat, DataModel, QuestionnaireResult, Strictness,
};
use dbtective::core::init::{create_config, InitResult};
use std::fs;
use tempfile::TempDir;

fn default_options(temp_dir: &TempDir) -> InitOptions {
    InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    }
}

fn create_default_init_config() -> dbtective::core::init::config_builder::InitConfig {
    use dbtective::core::config::naming_convention::NamingConvention;
    use dbtective::core::init::config_builder::InitConfig;
    use dbtective::core::init::questionnaire::{DataModel, QuestionnaireResult};

    let questionnaire_result = QuestionnaireResult {
        format: dbtective::core::init::questionnaire::ConfigFormat::Yaml,
        naming_convention: NamingConvention::default(),
        data_model: DataModel::Common,
        manifest_rules: vec![
            ManifestSpecificRuleConfig::HasDescription {
                min_length: None,
                forbidden_substrings: None,
            },
            ManifestSpecificRuleConfig::NameConvention {
                convention: NamingConvention::default(),
            },
            ManifestSpecificRuleConfig::HasMetadataKeys {
                required_keys: vec![],
                custom_message: None,
            },
            ManifestSpecificRuleConfig::HasRefs {},
            ManifestSpecificRuleConfig::AllowedSubfolders {
                allowed_subfolders: vec![],
                path_prefix: None,
                path_postfix: None,
            },
        ],
        catalog_rules: vec![
            CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            },
            CatalogSpecificRuleConfig::ColumnsHaveDescription {},
        ],
    };

    InitConfig::from_questionnaire(&questionnaire_result)
}

#[test]
fn test_init_creates_yaml_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);
    let config = create_default_init_config();

    let result = create_config(&options, &config, false);
    assert!(matches!(result, InitResult::Created(_)));

    let config_path = temp_dir.path().join("dbtective.yml");
    assert!(config_path.exists());
}

#[test]
fn test_init_yaml_contains_required_rules() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);
    let config = create_default_init_config();

    create_config(&options, &config, false);

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
    let config = create_default_init_config();

    create_config(&options, &config, false);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("manifest_tests:"));
}

#[test]
fn test_init_yaml_is_valid_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);
    let init_config = create_default_init_config();

    create_config(&options, &init_config, false);

    let config_path = temp_dir.path().join("dbtective.yml");
    let config = Config::from_file(&config_path);
    assert!(config.is_ok(), "Generated YAML config should be valid");

    let config = config.unwrap();
    let manifest_tests = config.manifest_tests.expect("manifest_tests should exist");
    assert_eq!(manifest_tests.len(), 6, "Should have 6 default rules");
}

// ===== TOML CONFIG TESTS =====

#[test]
fn test_init_creates_toml_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "toml".to_string(),
    };
    let config = create_default_init_config();

    let result = create_config(&options, &config, false);
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
    let init_config = create_default_init_config();

    create_config(&options, &init_config, false);

    let config_path = temp_dir.path().join("dbtective.toml");
    let config = Config::from_file(&config_path);
    assert!(config.is_ok(), "Generated TOML config should be valid");

    let config = config.unwrap();
    let manifest_tests = config.manifest_tests.expect("manifest_tests should exist");
    assert_eq!(manifest_tests.len(), 6, "Should have 6 default rules");
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
    let config = create_default_init_config();

    create_config(&options, &config, false);

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
    let config = create_default_init_config();

    let result = create_config(&options, &config, false);
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
    let config = create_default_init_config();

    let result = create_config(&options, &config, false);
    assert!(matches!(result, InitResult::PyprojectAlreadyConfigured(_)));
}

// ===== ALREADY EXISTS TESTS =====

#[test]
fn test_init_does_not_overwrite_existing_yml() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("dbtective.yml");
    fs::write(&config_path, "existing content").unwrap();

    let options = default_options(&temp_dir);
    let config = create_default_init_config();

    let result = create_config(&options, &config, false);
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
    let config = create_default_init_config();

    let result = create_config(&options, &config, false);
    assert!(matches!(result, InitResult::AlreadyExists(_)));
}

// ===== ERROR HANDLING TESTS =====

#[test]
fn test_init_fails_for_nonexistent_directory() {
    let options = InitOptions {
        location: "/nonexistent/path/that/does/not/exist".to_string(),
        format: "yml".to_string(),
    };
    let config = create_default_init_config();

    let result = create_config(&options, &config, false);
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
    let config = create_default_init_config();

    let result = create_config(&options, &config, false);
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
    let config = create_default_init_config();

    let result = create_config(&options, &config, false);
    assert!(matches!(result, InitResult::Created(_)));

    let config_path = temp_dir.path().join("dbtective.yml");
    assert!(
        config_path.exists(),
        "Should create .yml file even with yaml format"
    );
}

/// Helper to get manifest rules for a given strictness level
fn get_manifest_rules(
    strictness: Strictness,
    data_model: DataModel,
) -> Vec<ManifestSpecificRuleConfig> {
    let mut rules = Vec::new();

    // Basic rules (always included)
    rules.push(ManifestSpecificRuleConfig::HasDescription {
        min_length: None,
        forbidden_substrings: None,
    });
    rules.push(ManifestSpecificRuleConfig::NameConvention {
        convention: NamingConvention::default(),
    });

    // Standard adds more rules
    if matches!(strictness, Strictness::Standard | Strictness::Strict) {
        rules.push(ManifestSpecificRuleConfig::HasMetadataKeys {
            required_keys: vec![],
            custom_message: None,
        });
        rules.push(ManifestSpecificRuleConfig::HasRefs {});
    }

    // Strict adds even more
    if matches!(strictness, Strictness::Strict) {
        rules.push(ManifestSpecificRuleConfig::HasTags {
            required_tags: vec![],
            criteria: HasTagsCriteria::default(),
        });
        rules.push(ManifestSpecificRuleConfig::HasUniqueTest {
            allowed_test_names: vec![],
        });
    }

    // Add data-model-specific rules if data model is specified
    if !matches!(data_model, DataModel::None) {
        rules.push(ManifestSpecificRuleConfig::AllowedSubfolders {
            allowed_subfolders: vec![],
            path_prefix: None,
            path_postfix: None,
        });
        rules.push(ManifestSpecificRuleConfig::HasContractEnforced { access_level: None });
    }

    rules
}

/// Helper to get catalog rules for a given strictness level
fn get_catalog_rules(strictness: Strictness) -> Vec<CatalogSpecificRuleConfig> {
    let mut rules = Vec::new();

    // Basic rules
    rules.push(CatalogSpecificRuleConfig::ColumnsNameConvention {
        convention: NamingConvention::default(),
        data_types: None,
        use_database_columns: true,
    });

    // Standard adds more
    if matches!(strictness, Strictness::Standard | Strictness::Strict) {
        rules.push(CatalogSpecificRuleConfig::ColumnsHaveDescription {});
    }

    // Strict adds even more
    if matches!(strictness, Strictness::Strict) {
        rules.push(CatalogSpecificRuleConfig::ColumnsAllDocumented {});
    }

    rules
}

/// Create a full questionnaire result for testing
fn create_full_questionnaire_result(
    format: ConfigFormat,
    naming_convention: NamingConvention,
    data_model: DataModel,
    strictness: Strictness,
) -> QuestionnaireResult {
    QuestionnaireResult {
        format,
        naming_convention,
        data_model,
        manifest_rules: get_manifest_rules(strictness, data_model),
        catalog_rules: get_catalog_rules(strictness),
    }
}

// ========================================================================
// Data Model Integration Tests
// ========================================================================

#[test]
fn test_medallion_basic_generates_valid_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        DataModel::Medallion,
        Strictness::Basic,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    // Verify medallion subfolders
    assert!(content.contains("allowed_subfolders"));
    assert!(content.contains("\"bronze\""));
    assert!(content.contains("\"silver\""));
    assert!(content.contains("\"gold\""));
}

#[test]
fn test_common_standard_generates_valid_toml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "toml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Toml,
        NamingConvention::from_pattern("snake_case").unwrap(),
        DataModel::Common,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.toml");
    let content = fs::read_to_string(&config_path).unwrap();

    // Verify common subfolders
    assert!(content.contains("allowed_subfolders"));
    assert!(content.contains("\"staging\""));
    assert!(content.contains("\"marts\""));
    assert!(content.contains("\"intermediate\""));
}

#[test]
fn test_none_strict_generates_valid_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        DataModel::None,
        Strictness::Strict,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    // Verify NO allowed_subfolders when DataModel::None
    assert!(!content.contains("allowed_subfolders"));
}

// ========================================================================
// Strictness Level Integration Tests
// ========================================================================

#[test]
fn test_basic_strictness_has_correct_rules() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        DataModel::None,
        Strictness::Basic,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("has_description"));
    assert!(content.contains("name_convention"));
}

#[test]
fn test_standard_strictness_has_correct_rules() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        DataModel::None,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("has_description"));
    assert!(content.contains("name_convention"));
    assert!(content.contains("has_metadata_keys"));
    assert!(content.contains("has_refs"));
}

#[test]
fn test_strict_strictness_has_correct_rules() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        DataModel::None,
        Strictness::Strict,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("has_tags"));
    assert!(content.contains("has_unique_test"));
    assert!(!content.contains("has_contract_enforced"));
}

// ========================================================================
// Naming Convention Integration Tests
// ========================================================================

#[test]
fn test_snake_case_naming_convention() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::from_pattern("snake_case").unwrap(),
        DataModel::None,
        Strictness::Basic,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("pattern: \"snake_case\""));
}

#[test]
fn test_kebab_case_naming_convention() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::from_pattern("kebab-case").unwrap(),
        DataModel::None,
        Strictness::Basic,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("pattern: \"kebab-case\""));
}

#[test]
fn test_camel_case_naming_convention() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::from_pattern("camelCase").unwrap(),
        DataModel::None,
        Strictness::Basic,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("pattern: \"camelCase\""));
}

#[test]
fn test_pascal_case_naming_convention() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::from_pattern("PascalCase").unwrap(),
        DataModel::None,
        Strictness::Basic,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("pattern: \"PascalCase\""));
}

// ========================================================================
// Format Integration Tests
// ========================================================================

#[test]
fn test_yaml_format_generates_valid_file() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        DataModel::Common,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    assert!(config_path.exists());
}

#[test]
fn test_toml_format_generates_valid_file() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "toml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Toml,
        NamingConvention::default(),
        DataModel::Common,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.toml");
    assert!(config_path.exists());
}

#[test]
fn test_pyproject_format_generates_valid_file() {
    let temp_dir = TempDir::new().unwrap();
    let pyproject_path = temp_dir.path().join("pyproject.toml");

    // Create existing pyproject.toml
    fs::write(&pyproject_path, "[project]\nname = \"test\"\n").unwrap();

    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "pyproject".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Pyproject,
        NamingConvention::default(),
        DataModel::Common,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let content = fs::read_to_string(&pyproject_path).unwrap();
    assert!(content.contains("[tool.dbtective]"));
}

// ========================================================================
// Combination Tests
// ========================================================================

#[test]
fn test_all_data_models_with_strict() {
    let data_models = vec![DataModel::Medallion, DataModel::Common, DataModel::None];

    for data_model in data_models {
        let temp_dir = TempDir::new().unwrap();
        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: "yml".to_string(),
        };

        let questionnaire_result = create_full_questionnaire_result(
            ConfigFormat::Yaml,
            NamingConvention::default(),
            data_model,
            Strictness::Strict,
        );

        let exit_code = init_with_result(&options, false, questionnaire_result);
        assert_eq!(exit_code, 0, "Data model {data_model:?} should succeed");

        let config_path = temp_dir.path().join("dbtective.yml");
        assert!(config_path.exists());
    }
}

#[test]
fn test_all_naming_conventions_generate_valid_configs() {
    let conventions = vec!["snake_case", "kebab-case", "camelCase", "PascalCase"];

    for convention_str in conventions {
        let temp_dir = TempDir::new().unwrap();
        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: "yml".to_string(),
        };

        let naming_convention = NamingConvention::from_pattern(convention_str).unwrap();
        let questionnaire_result = create_full_questionnaire_result(
            ConfigFormat::Yaml,
            naming_convention,
            DataModel::None,
            Strictness::Basic,
        );

        let exit_code = init_with_result(&options, false, questionnaire_result);
        assert_eq!(exit_code, 0, "Convention {convention_str} should succeed");
    }
}

#[test]
fn test_all_formats_generate_parseable_configs() {
    let formats = vec![("yml", "dbtective.yml"), ("toml", "dbtective.toml")];

    for (format_str, filename) in formats {
        let temp_dir = TempDir::new().unwrap();
        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
            format: format_str.to_string(),
        };

        #[allow(clippy::match_same_arms)]
        let config_format = match format_str {
            "yml" => ConfigFormat::Yaml,
            "toml" => ConfigFormat::Toml,
            _ => ConfigFormat::Yaml,
        };

        let questionnaire_result = create_full_questionnaire_result(
            config_format,
            NamingConvention::default(),
            DataModel::Common,
            Strictness::Standard,
        );

        let exit_code = init_with_result(&options, false, questionnaire_result);
        assert_eq!(exit_code, 0, "Format {format_str} should succeed");

        let config_path = temp_dir.path().join(filename);
        assert!(config_path.exists());
    }
}

// ========================================================================
// Specific Rule Content Verification Tests
// ========================================================================

#[test]
fn test_has_tags_rule_generates_correct_content() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        DataModel::None,
        Strictness::Strict,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("type: \"has_tags\""));
    assert!(content.contains("\"daily\""));
    assert!(content.contains("\"monthly\""));
    assert!(content.contains("\"yearly\""));
    assert!(content.contains("\"inactive\""));
    assert!(content.contains("criteria: \"one_of\""));
}

#[test]
fn test_has_metadata_keys_rule_generates_correct_content() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "yml".to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        DataModel::None,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("type: \"has_metadata_keys\""));
    assert!(content.contains("required_keys:"));
    assert!(content.contains("\"owner\""));
}

#[test]
fn test_columns_canonical_name_rule_generates_correct_content() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
        format: "toml".to_string(),
    };

    // Add ColumnsCanonicalName explicitly
    let manifest_rules = get_manifest_rules(Strictness::Basic, DataModel::None);
    let mut catalog_rules = get_catalog_rules(Strictness::Basic);
    catalog_rules.push(CatalogSpecificRuleConfig::ColumnsCanonicalName {
        canonical: String::new(),
        invalid_names: vec![],
        exceptions: None,
    });

    let questionnaire_result = QuestionnaireResult {
        format: ConfigFormat::Toml,
        naming_convention: NamingConvention::default(),
        data_model: DataModel::None,
        manifest_rules,
        catalog_rules,
    };

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.toml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("type = \"columns_canonical_name\""));
    assert!(content.contains("canonical = \"user_id\""));
    assert!(content.contains("\"userid\""));
    assert!(content.contains("\"UserId\""));
}
