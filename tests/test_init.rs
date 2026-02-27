use dbtective::cli::commands::InitOptions;
use dbtective::core::config::catalog_rule::CatalogSpecificRuleConfig;
use dbtective::core::config::check_config_options::HasTagsCriteria;
use dbtective::core::config::manifest_rule::ManifestSpecificRuleConfig;
use dbtective::core::config::naming_convention::NamingConvention;
use dbtective::core::config::parse_config::Config;
use dbtective::core::init::init_with_result;
use dbtective::core::init::questionnaire::{
    ConfigFormat, DataModelMethodology, LayeringStrategy, QuestionnaireResult, Strictness,
};
use dbtective::core::init::{create_config, InitResult};
use std::fs;
use tempfile::TempDir;

fn default_options(temp_dir: &TempDir) -> InitOptions {
    InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    }
}

fn create_default_init_config_with_format(
    format: dbtective::core::init::questionnaire::ConfigFormat,
) -> dbtective::core::init::config_builder::InitConfig {
    use dbtective::core::config::naming_convention::NamingConvention;
    use dbtective::core::init::config_builder::InitConfig;
    use dbtective::core::init::questionnaire::{
        DataModelMethodology, LayeringStrategy, QuestionnaireResult,
    };

    let questionnaire_result = QuestionnaireResult {
        format,
        naming_convention: NamingConvention::default(),
        layering_strategy: LayeringStrategy::Common,
        methodology: DataModelMethodology::None,
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
        auto_parse_command: None,
    };

    InitConfig::from_questionnaire(&questionnaire_result)
}

fn create_default_init_config() -> dbtective::core::init::config_builder::InitConfig {
    create_default_init_config_with_format(dbtective::core::init::questionnaire::ConfigFormat::Yaml)
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
    };
    let config = create_default_init_config_with_format(ConfigFormat::Toml);

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
    };
    let init_config = create_default_init_config_with_format(ConfigFormat::Toml);

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
    };
    let config = create_default_init_config_with_format(ConfigFormat::Pyproject);

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
    };
    let config = create_default_init_config_with_format(ConfigFormat::Pyproject);

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
    };
    let config = create_default_init_config_with_format(ConfigFormat::Pyproject);

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
    };
    let config = create_default_init_config_with_format(ConfigFormat::Toml);

    let result = create_config(&options, &config, false);
    assert!(matches!(result, InitResult::AlreadyExists(_)));
}

// ===== ERROR HANDLING TESTS =====

#[test]
fn test_init_fails_for_nonexistent_directory() {
    let options = InitOptions {
        location: "/nonexistent/path/that/does/not/exist".to_string(),
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
    layering_strategy: LayeringStrategy,
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

    // Add layering-specific rules if a layering strategy is specified
    if !matches!(layering_strategy, LayeringStrategy::None) {
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
    layering_strategy: LayeringStrategy,
    strictness: Strictness,
) -> QuestionnaireResult {
    create_full_questionnaire_result_with_methodology(
        format,
        naming_convention,
        layering_strategy,
        DataModelMethodology::None,
        strictness,
    )
}

fn create_full_questionnaire_result_with_methodology(
    format: ConfigFormat,
    naming_convention: NamingConvention,
    layering_strategy: LayeringStrategy,
    methodology: DataModelMethodology,
    strictness: Strictness,
) -> QuestionnaireResult {
    QuestionnaireResult {
        format,
        naming_convention,
        layering_strategy,
        methodology,
        manifest_rules: get_manifest_rules(strictness, layering_strategy),
        catalog_rules: get_catalog_rules(strictness),
        auto_parse_command: None,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::Medallion,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Toml,
        NamingConvention::from_pattern("snake_case").unwrap(),
        LayeringStrategy::Common,
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
    assert!(content.contains("\"warehouse\""));
}

#[test]
fn test_none_strict_generates_valid_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::None,
        Strictness::Strict,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    // Verify NO allowed_subfolders when LayeringStrategy::None
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::None,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::None,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::None,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::from_pattern("snake_case").unwrap(),
        LayeringStrategy::None,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::from_pattern("kebab-case").unwrap(),
        LayeringStrategy::None,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::from_pattern("camelCase").unwrap(),
        LayeringStrategy::None,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::from_pattern("PascalCase").unwrap(),
        LayeringStrategy::None,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::Common,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Toml,
        NamingConvention::default(),
        LayeringStrategy::Common,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Pyproject,
        NamingConvention::default(),
        LayeringStrategy::Common,
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
fn test_all_layering_strategies_with_strict() {
    let strategies = vec![
        LayeringStrategy::Medallion,
        LayeringStrategy::Common,
        LayeringStrategy::None,
    ];

    for layering_strategy in strategies {
        let temp_dir = TempDir::new().unwrap();
        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
        };

        let questionnaire_result = create_full_questionnaire_result(
            ConfigFormat::Yaml,
            NamingConvention::default(),
            layering_strategy,
            Strictness::Strict,
        );

        let exit_code = init_with_result(&options, false, questionnaire_result);
        assert_eq!(
            exit_code, 0,
            "Layering strategy {layering_strategy:?} should succeed"
        );

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
        };

        let naming_convention = NamingConvention::from_pattern(convention_str).unwrap();
        let questionnaire_result = create_full_questionnaire_result(
            ConfigFormat::Yaml,
            naming_convention,
            LayeringStrategy::None,
            Strictness::Basic,
        );

        let exit_code = init_with_result(&options, false, questionnaire_result);
        assert_eq!(exit_code, 0, "Convention {convention_str} should succeed");
    }
}

#[test]
fn test_all_formats_generate_parseable_configs() {
    let formats = vec![
        (ConfigFormat::Yaml, "dbtective.yml"),
        (ConfigFormat::Toml, "dbtective.toml"),
    ];

    for (config_format, filename) in formats {
        let temp_dir = TempDir::new().unwrap();
        let options = InitOptions {
            location: temp_dir.path().to_string_lossy().to_string(),
        };

        let questionnaire_result = create_full_questionnaire_result(
            config_format,
            NamingConvention::default(),
            LayeringStrategy::Common,
            Strictness::Standard,
        );

        let exit_code = init_with_result(&options, false, questionnaire_result);
        assert_eq!(exit_code, 0, "Format {config_format:?} should succeed");

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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::None,
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
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::None,
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
    };

    // Add ColumnsCanonicalName explicitly
    let manifest_rules = get_manifest_rules(Strictness::Basic, LayeringStrategy::None);
    let mut catalog_rules = get_catalog_rules(Strictness::Basic);
    catalog_rules.push(CatalogSpecificRuleConfig::ColumnsCanonicalName {
        canonical: String::new(),
        invalid_names: vec![],
        exceptions: None,
    });

    let questionnaire_result = QuestionnaireResult {
        format: ConfigFormat::Toml,
        naming_convention: NamingConvention::default(),
        layering_strategy: LayeringStrategy::None,
        methodology: DataModelMethodology::None,
        manifest_rules,
        catalog_rules,
        auto_parse_command: None,
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

// ========================================================================
// Auto-parse Command Tests
// ========================================================================

#[test]
fn test_init_yaml_contains_auto_parse_comment_when_none() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);
    let config = create_default_init_config();

    create_config(&options, &config, false);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    // Should contain commented-out auto_parse_command
    assert!(content.contains("# config:"));
    assert!(content.contains("#   auto_parse_command:"));
}

#[test]
fn test_init_yaml_contains_auto_parse_command_when_set() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);

    let questionnaire_result = QuestionnaireResult {
        format: ConfigFormat::Yaml,
        naming_convention: NamingConvention::default(),
        layering_strategy: LayeringStrategy::None,
        methodology: DataModelMethodology::None,
        manifest_rules: vec![ManifestSpecificRuleConfig::HasDescription {
            min_length: None,
            forbidden_substrings: None,
        }],
        catalog_rules: vec![],
        auto_parse_command: Some("uv run dbt parse".to_string()),
    };

    let config = dbtective::core::init::config_builder::InitConfig::from_questionnaire(
        &questionnaire_result,
    );
    create_config(&options, &config, false);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("config:"));
    assert!(content.contains("auto_parse_command: \"uv run dbt parse\""));
}

#[test]
fn test_init_yaml_with_auto_parse_is_valid_parseable_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);

    let questionnaire_result = QuestionnaireResult {
        format: ConfigFormat::Yaml,
        naming_convention: NamingConvention::default(),
        layering_strategy: LayeringStrategy::None,
        methodology: DataModelMethodology::None,
        manifest_rules: vec![ManifestSpecificRuleConfig::HasDescription {
            min_length: None,
            forbidden_substrings: None,
        }],
        catalog_rules: vec![],
        auto_parse_command: Some("uv run dbt parse".to_string()),
    };

    let config = dbtective::core::init::config_builder::InitConfig::from_questionnaire(
        &questionnaire_result,
    );
    create_config(&options, &config, false);

    let config_path = temp_dir.path().join("dbtective.yml");
    let parsed = Config::from_file(&config_path);
    assert!(
        parsed.is_ok(),
        "YAML with auto_parse_command should parse successfully"
    );

    let parsed = parsed.unwrap();
    assert_eq!(parsed.auto_parse_command(), Some("uv run dbt parse"));
}

#[test]
fn test_init_toml_contains_auto_parse_command_when_set() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = QuestionnaireResult {
        format: ConfigFormat::Toml,
        naming_convention: NamingConvention::default(),
        layering_strategy: LayeringStrategy::None,
        methodology: DataModelMethodology::None,
        manifest_rules: vec![ManifestSpecificRuleConfig::HasDescription {
            min_length: None,
            forbidden_substrings: None,
        }],
        catalog_rules: vec![],
        auto_parse_command: Some("poetry run dbt parse".to_string()),
    };

    let config = dbtective::core::init::config_builder::InitConfig::from_questionnaire(
        &questionnaire_result,
    );
    create_config(&options, &config, false);

    let config_path = temp_dir.path().join("dbtective.toml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("[config]"));
    assert!(content.contains("auto_parse_command = \"poetry run dbt parse\""));
}

#[test]
fn test_init_toml_with_auto_parse_is_valid_parseable_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = QuestionnaireResult {
        format: ConfigFormat::Toml,
        naming_convention: NamingConvention::default(),
        layering_strategy: LayeringStrategy::None,
        methodology: DataModelMethodology::None,
        manifest_rules: vec![ManifestSpecificRuleConfig::HasDescription {
            min_length: None,
            forbidden_substrings: None,
        }],
        catalog_rules: vec![],
        auto_parse_command: Some("dbt parse".to_string()),
    };

    let config = dbtective::core::init::config_builder::InitConfig::from_questionnaire(
        &questionnaire_result,
    );
    create_config(&options, &config, false);

    let config_path = temp_dir.path().join("dbtective.toml");
    let parsed = Config::from_file(&config_path);
    assert!(
        parsed.is_ok(),
        "TOML with auto_parse_command should parse successfully"
    );

    let parsed = parsed.unwrap();
    assert_eq!(parsed.auto_parse_command(), Some("dbt parse"));
}

#[test]
fn test_init_pyproject_contains_auto_parse_command_when_set() {
    let temp_dir = TempDir::new().unwrap();
    let pyproject_path = temp_dir.path().join("pyproject.toml");
    fs::write(&pyproject_path, "[project]\nname = \"test\"\n").unwrap();

    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = QuestionnaireResult {
        format: ConfigFormat::Pyproject,
        naming_convention: NamingConvention::default(),
        layering_strategy: LayeringStrategy::None,
        methodology: DataModelMethodology::None,
        manifest_rules: vec![ManifestSpecificRuleConfig::HasDescription {
            min_length: None,
            forbidden_substrings: None,
        }],
        catalog_rules: vec![],
        auto_parse_command: Some("uv run dbt parse".to_string()),
    };

    let config = dbtective::core::init::config_builder::InitConfig::from_questionnaire(
        &questionnaire_result,
    );
    create_config(&options, &config, false);

    let content = fs::read_to_string(&pyproject_path).unwrap();

    assert!(content.contains("[tool.dbtective.config]"));
    assert!(content.contains("auto_parse_command = \"uv run dbt parse\""));
}

#[test]
fn test_init_without_auto_parse_generates_backward_compatible_config() {
    let temp_dir = TempDir::new().unwrap();
    let options = default_options(&temp_dir);
    let config = create_default_init_config();

    create_config(&options, &config, false);

    let config_path = temp_dir.path().join("dbtective.yml");
    let parsed = Config::from_file(&config_path);
    assert!(
        parsed.is_ok(),
        "Config without auto_parse should still parse"
    );

    let parsed = parsed.unwrap();
    assert_eq!(parsed.auto_parse_command(), None);
}

// ========================================================================
// Questionnaire Format Choice Tests
// ========================================================================

#[test]
fn test_questionnaire_toml_choice_creates_toml_file_not_yml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Toml,
        NamingConvention::default(),
        LayeringStrategy::Common,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    // TOML file should exist
    let toml_path = temp_dir.path().join("dbtective.toml");
    assert!(
        toml_path.exists(),
        "Choosing TOML format should create dbtective.toml"
    );

    // YAML file should NOT exist
    let yml_path = temp_dir.path().join("dbtective.yml");
    assert!(
        !yml_path.exists(),
        "Choosing TOML format should NOT create dbtective.yml"
    );

    // Config should be parseable
    let parsed = Config::from_file(&toml_path);
    assert!(parsed.is_ok(), "Generated TOML config should be valid");

    let parsed = parsed.unwrap();
    let manifest_tests = parsed.manifest_tests.expect("manifest_tests should exist");
    assert!(!manifest_tests.is_empty(), "Should have manifest rules");
}

#[test]
fn test_questionnaire_yaml_choice_creates_yml_file_not_toml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::Common,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    // YAML file should exist
    let yml_path = temp_dir.path().join("dbtective.yml");
    assert!(
        yml_path.exists(),
        "Choosing YAML format should create dbtective.yml"
    );

    // TOML file should NOT exist
    let toml_path = temp_dir.path().join("dbtective.toml");
    assert!(
        !toml_path.exists(),
        "Choosing YAML format should NOT create dbtective.toml"
    );

    // Config should be parseable
    let parsed = Config::from_file(&yml_path);
    assert!(parsed.is_ok(), "Generated YAML config should be valid");
}

#[test]
fn test_questionnaire_pyproject_choice_updates_pyproject_toml() {
    let temp_dir = TempDir::new().unwrap();
    let pyproject_path = temp_dir.path().join("pyproject.toml");
    fs::write(&pyproject_path, "[project]\nname = \"test\"\n").unwrap();

    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result(
        ConfigFormat::Pyproject,
        NamingConvention::default(),
        LayeringStrategy::Common,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    // pyproject.toml should contain dbtective section
    let content = fs::read_to_string(&pyproject_path).unwrap();
    assert!(
        content.contains("[tool.dbtective]"),
        "Should add dbtective section to pyproject.toml"
    );
    assert!(
        content.contains("[[tool.dbtective.manifest_tests]]"),
        "Should have manifest tests"
    );

    // No separate config files should be created
    let yml_path = temp_dir.path().join("dbtective.yml");
    let toml_path = temp_dir.path().join("dbtective.toml");
    assert!(
        !yml_path.exists(),
        "Choosing pyproject format should NOT create dbtective.yml"
    );
    assert!(
        !toml_path.exists(),
        "Choosing pyproject format should NOT create dbtective.toml"
    );
}

// ========================================================================
// Methodology Integration Tests
// ========================================================================

#[test]
fn test_kimball_medallion_strict_generates_valid_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result_with_methodology(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::Medallion,
        DataModelMethodology::Kimball,
        Strictness::Strict,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("gold_subfolders"));
    assert!(content.contains("dimension_naming"));
    assert!(content.contains("fact_naming"));
    assert!(content.contains("^dim_"));
    assert!(content.contains("^fact_"));
}

#[test]
fn test_kimball_yaml_contains_dimension_and_fact_naming_rules() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result_with_methodology(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::Common,
        DataModelMethodology::Kimball,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains(r#"includes: ["models/warehouse/dimensions"]"#));
    assert!(content.contains(r#"includes: ["models/warehouse/facts"]"#));
}

#[test]
fn test_data_vault_medallion_strict_generates_valid_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result_with_methodology(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::Medallion,
        DataModelMethodology::DataVault,
        Strictness::Strict,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("silver_vault_subfolders"));
    assert!(content.contains("hub_naming"));
    assert!(content.contains("link_naming"));
    assert!(content.contains("satellite_naming"));
    assert!(content.contains("vault_contracts"));
}

#[test]
fn test_data_vault_yaml_contains_hub_link_sat_naming_rules() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result_with_methodology(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::Medallion,
        DataModelMethodology::DataVault,
        Strictness::Basic,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("^hub_"));
    assert!(content.contains("^lnk_"));
    assert!(content.contains("^sat_"));
    assert!(content.contains("^t_lnk_"));
}

#[test]
fn test_inmon_medallion_strict_generates_valid_yaml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result_with_methodology(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::Medallion,
        DataModelMethodology::Inmon,
        Strictness::Strict,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains("silver_subfolders"));
    assert!(content.contains("gold_subfolders"));
    assert!(content.contains("enterprise_model_contracts"));
    assert!(content.contains("enterprise_model_unique_keys"));
}

#[test]
fn test_inmon_yaml_contains_enterprise_subfolders() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result_with_methodology(
        ConfigFormat::Yaml,
        NamingConvention::default(),
        LayeringStrategy::Common,
        DataModelMethodology::Inmon,
        Strictness::Basic,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.yml");
    let content = fs::read_to_string(&config_path).unwrap();
    assert!(content.contains(r#""core""#));
    assert!(content.contains(r#""reference""#));
    assert!(content.contains(r#""bridge""#));
}

#[test]
fn test_kimball_common_standard_generates_valid_toml() {
    let temp_dir = TempDir::new().unwrap();
    let options = InitOptions {
        location: temp_dir.path().to_string_lossy().to_string(),
    };

    let questionnaire_result = create_full_questionnaire_result_with_methodology(
        ConfigFormat::Toml,
        NamingConvention::default(),
        LayeringStrategy::Common,
        DataModelMethodology::Kimball,
        Strictness::Standard,
    );

    let exit_code = init_with_result(&options, false, questionnaire_result);
    assert_eq!(exit_code, 0);

    let config_path = temp_dir.path().join("dbtective.toml");
    let content = fs::read_to_string(&config_path).unwrap();

    assert!(content.contains(r#"name = "warehouse_subfolders""#));
    assert!(content.contains(r#"name = "dimension_naming""#));
    assert!(content.contains(r#"pattern = "^dim_""#));
}

#[test]
fn test_all_methodology_layering_combinations_generate_parseable_yaml() {
    let strategies = vec![
        LayeringStrategy::Medallion,
        LayeringStrategy::Common,
        LayeringStrategy::None,
    ];
    let methodologies = vec![
        DataModelMethodology::None,
        DataModelMethodology::Kimball,
        DataModelMethodology::Inmon,
        DataModelMethodology::DataVault,
    ];

    for strategy in &strategies {
        for methodology in &methodologies {
            let temp_dir = TempDir::new().unwrap();
            let options = InitOptions {
                location: temp_dir.path().to_string_lossy().to_string(),
            };

            let questionnaire_result = create_full_questionnaire_result_with_methodology(
                ConfigFormat::Yaml,
                NamingConvention::default(),
                *strategy,
                *methodology,
                Strictness::Standard,
            );

            let exit_code = init_with_result(&options, false, questionnaire_result);
            assert_eq!(exit_code, 0, "Failed for {strategy:?} + {methodology:?}");

            let config_path = temp_dir.path().join("dbtective.yml");
            let config = Config::from_file(&config_path);
            assert!(
                config.is_ok(),
                "Generated YAML should be parseable for {strategy:?} + {methodology:?}: {:?}",
                config.err()
            );
        }
    }
}

#[test]
fn test_all_methodology_layering_combinations_generate_parseable_toml() {
    let strategies = vec![
        LayeringStrategy::Medallion,
        LayeringStrategy::Common,
        LayeringStrategy::None,
    ];
    let methodologies = vec![
        DataModelMethodology::None,
        DataModelMethodology::Kimball,
        DataModelMethodology::Inmon,
        DataModelMethodology::DataVault,
    ];

    for strategy in &strategies {
        for methodology in &methodologies {
            let temp_dir = TempDir::new().unwrap();
            let options = InitOptions {
                location: temp_dir.path().to_string_lossy().to_string(),
            };

            let questionnaire_result = create_full_questionnaire_result_with_methodology(
                ConfigFormat::Toml,
                NamingConvention::default(),
                *strategy,
                *methodology,
                Strictness::Standard,
            );

            let exit_code = init_with_result(&options, false, questionnaire_result);
            assert_eq!(exit_code, 0, "Failed for {strategy:?} + {methodology:?}");

            let config_path = temp_dir.path().join("dbtective.toml");
            let config = Config::from_file(&config_path);
            assert!(
                config.is_ok(),
                "Generated TOML should be parseable for {strategy:?} + {methodology:?}: {:?}",
                config.err()
            );
        }
    }
}
