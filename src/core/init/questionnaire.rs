use crate::core::config::catalog_rule::CatalogSpecificRuleConfig;
use crate::core::config::check_config_options::HasTagsCriteria;
use crate::core::config::manifest_rule::ManifestSpecificRuleConfig;
use crate::core::config::naming_convention::NamingConvention;
use inquire::{MultiSelect, Select};
use owo_colors::OwoColorize;
use std::path::Path;
use strum::IntoEnumIterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Yaml,
    Toml,
    Pyproject,
}

impl ConfigFormat {
    #[allow(dead_code)]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Yaml => "yml",
            Self::Toml => "toml",
            Self::Pyproject => "pyproject",
        }
    }
}

impl std::fmt::Display for ConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yaml => write!(f, "YAML (recommended)"),
            Self::Toml => write!(f, "TOML"),
            Self::Pyproject => write!(f, "pyproject.toml"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataModel {
    Medallion,
    Common,
    None,
}

impl std::fmt::Display for DataModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Medallion => write!(f, "Medallion (bronze, silver, gold)"),
            Self::Common => write!(f, "Common (staging, intermediate, marts)"),
            Self::None => write!(f, "None (no folder structure enforcement)"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strictness {
    Basic,
    Standard,
    Strict,
}

impl std::fmt::Display for Strictness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Basic => write!(f, "Basic - Essential rules only"),
            Self::Standard => write!(f, "Standard - Balanced set of rules"),
            Self::Strict => write!(f, "Strict (recommended) - Comprehensive rule coverage"),
        }
    }
}

/// Trait for providing init-time descriptions of rule configs.
/// Adding a new variant to a rule config enum without updating this impl
/// will cause a compile error.
pub trait Init {
    fn init_description(&self) -> &'static str;
}

impl Init for ManifestSpecificRuleConfig {
    fn init_description(&self) -> &'static str {
        match self {
            Self::HasDescription { .. } => "has_description - Require descriptions for resources",
            Self::NameConvention { .. } => "name_convention - Enforce naming conventions",
            Self::HasTags { .. } => "has_tags - Require specific tags",
            Self::IsNotOrphaned { .. } => "is_not_orphaned - Check for orphaned sources",
            Self::HasUniqueTest { .. } => "has_unique_test - Require uniqueness tests",
            Self::HasContractEnforced { .. } => {
                "has_contract_enforced - Require enforced contracts"
            }
            Self::HasMetadataKeys { .. } => {
                "has_metadata_keys - Require metadata keys (e.g., owner)"
            }
            Self::HasRefs { .. } => "has_refs - Require use of ref() function",
            Self::MaxCodeLines { .. } => "max_code_lines - Limit code line count",
            Self::AllowedSubfolders { .. } => "allowed_subfolders - Restrict subfolder usage",
            Self::SourcesHaveLoader { .. } => "sources_have_loader - Require loader for sources",
            Self::HasForbiddenCode { .. } => {
                "has_forbidden_code - Check for forbidden code patterns"
            }
            Self::CodeContainsRefs { .. } => {
                "code_contains_refs - Require ref()/source() calls in SQL code"
            }
            Self::MaxJoins { .. } => "max_joins - Limit JOIN count to reduce code complexity",
            Self::MaxUpstreamDependencies { .. } => {
                "max_upstream_dependencies - Limit how many objects a node depends on"
            }
            Self::MaxDownstreamDependencies { .. } => {
                "max_downstream_dependencies - Limit how many objects depend on a node"
            }
            Self::SourcesHaveFreshness { .. } => {
                "sources_have_freshness - Require freshness for sources"
            }
        }
    }
}

impl std::fmt::Display for ManifestSpecificRuleConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.init_description())
    }
}

impl Init for CatalogSpecificRuleConfig {
    fn init_description(&self) -> &'static str {
        match self {
            Self::ColumnsAllDocumented { .. } => {
                "columns_all_documented - All columns must exist in docs"
            }
            Self::ColumnsHaveDescription { .. } => {
                "columns_have_description - All columns need descriptions"
            }
            Self::ColumnsNameConvention { .. } => {
                "columns_name_convention - Enforce column naming conventions"
            }
            Self::ColumnsCanonicalName { .. } => {
                "columns_canonical_name - Enforce canonical column names"
            }
            Self::ColumnsHaveDataType { .. } => {
                "columns_have_data_type - Columns must have data types defined"
            }
        }
    }
}

impl std::fmt::Display for CatalogSpecificRuleConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.init_description())
    }
}

#[derive(Debug, Clone)]
pub struct QuestionnaireResult {
    #[allow(dead_code)]
    pub format: ConfigFormat,
    pub naming_convention: NamingConvention,
    pub data_model: DataModel,
    pub manifest_rules: Vec<ManifestSpecificRuleConfig>,
    pub catalog_rules: Vec<CatalogSpecificRuleConfig>,
    pub auto_parse_command: Option<String>,
}

#[allow(clippy::too_many_lines)]
/// # Errors
/// - Returns: `QuestionnaireResult` on success, String error message on failure
pub fn run_questionnaire() -> Result<QuestionnaireResult, String> {
    println!("\n🕵️ {}", "Welcome to dbtective!".green().bold());
    println!(
        "Let's set up your configuration. {}\n",
        "(Don't worry, you can always change it later)".dimmed()
    );

    // 1. Ask for config format
    let format_options = vec![
        ConfigFormat::Yaml,
        ConfigFormat::Toml,
        ConfigFormat::Pyproject,
    ];
    let format = Select::new("What config format do you want?", format_options)
        .with_starting_cursor(0) // Default to YAML (recommended)
        .prompt()
        .map_err(|e| format!("Failed to get config format: {e}"))?;

    // 2. Ask for naming convention
    let convention_options = vec![
        "snake_case (recommended)",
        "kebab-case",
        "camelCase",
        "PascalCase",
    ];
    let naming_convention_str =
        Select::new("What naming convention do you use?", convention_options)
            .with_starting_cursor(0) // Default to snake_case (recommended)
            .prompt()
            .map_err(|e| format!("Failed to get naming convention: {e}"))?;

    #[allow(clippy::match_same_arms)]
    let naming_convention = match naming_convention_str {
        "snake_case (recommended)" => NamingConvention::from_pattern("snake_case"),
        "kebab-case" => NamingConvention::from_pattern("kebab-case"),
        "camelCase" => NamingConvention::from_pattern("camelCase"),
        "PascalCase" => NamingConvention::from_pattern("PascalCase"),
        _ => NamingConvention::from_pattern("snake_case"),
    }
    .map_err(|e| format!("Invalid naming convention: {e}"))?;

    // 3. Ask for data model
    let data_model_options = vec![DataModel::None, DataModel::Common, DataModel::Medallion];
    let data_model = Select::new(
        "What data model structure are you using?",
        data_model_options,
    )
    .with_starting_cursor(1) // Default to Common
    .prompt()
    .map_err(|e| format!("Failed to get data model: {e}"))?;

    // 4. Ask for strictness level
    let strictness_options = vec![Strictness::Basic, Strictness::Standard, Strictness::Strict];
    let strictness = Select::new("How strict do you want the rules?", strictness_options)
        .with_starting_cursor(2) // Default to Strict (recommended start)
        .prompt()
        .map_err(|e| format!("Failed to get strictness level: {e}"))?;

    let (mut manifest_rules, mut catalog_rules) = match strictness {
        Strictness::Basic => (
            vec![
                ManifestSpecificRuleConfig::HasDescription {
                    min_length: None,
                    forbidden_substrings: None,
                },
                ManifestSpecificRuleConfig::NameConvention {
                    convention: NamingConvention::default(),
                },
                ManifestSpecificRuleConfig::CodeContainsRefs {},
            ],
            vec![CatalogSpecificRuleConfig::ColumnsNameConvention {
                convention: NamingConvention::default(),
                data_types: None,
                use_database_columns: true,
            }],
        ),
        Strictness::Standard => (
            vec![
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
                ManifestSpecificRuleConfig::CodeContainsRefs {},
            ],
            vec![
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::default(),
                    data_types: None,
                    use_database_columns: true,
                },
                CatalogSpecificRuleConfig::ColumnsHaveDescription {},
            ],
        ),
        Strictness::Strict => (
            vec![
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
                ManifestSpecificRuleConfig::HasTags {
                    required_tags: vec![],
                    criteria: HasTagsCriteria::default(),
                },
                ManifestSpecificRuleConfig::HasUniqueTest {
                    allowed_test_names: vec![],
                },
                ManifestSpecificRuleConfig::HasForbiddenCode {
                    forbidden_patterns: vec![],
                    case_sensitive: false,
                },
                ManifestSpecificRuleConfig::CodeContainsRefs {},
                ManifestSpecificRuleConfig::MaxJoins { max_joins: 0 },
                ManifestSpecificRuleConfig::MaxUpstreamDependencies {
                    max_upstream: 0,
                    exclude_types: vec![],
                },
                ManifestSpecificRuleConfig::MaxDownstreamDependencies {
                    max_downstream: 0,
                    exclude_types: vec![],
                },
                ManifestSpecificRuleConfig::SourcesHaveLoader {},
                ManifestSpecificRuleConfig::SourcesHaveFreshness {},
            ],
            vec![
                CatalogSpecificRuleConfig::ColumnsNameConvention {
                    convention: NamingConvention::default(),
                    data_types: None,
                    use_database_columns: true,
                },
                CatalogSpecificRuleConfig::ColumnsHaveDescription {},
                CatalogSpecificRuleConfig::ColumnsAllDocumented {},
            ],
        ),
    };

    // Always add the is_not_orphaned rule
    manifest_rules.push(ManifestSpecificRuleConfig::IsNotOrphaned {
        allowed_references: vec![],
    });

    // Add data-model-specific rules if a data model structure was selected
    if data_model != DataModel::None {
        manifest_rules.push(ManifestSpecificRuleConfig::AllowedSubfolders {
            allowed_subfolders: vec![],
            path_prefix: None,
            path_postfix: None,
        });
        manifest_rules.push(ManifestSpecificRuleConfig::HasContractEnforced { access_level: None });
        catalog_rules.push(CatalogSpecificRuleConfig::ColumnsHaveDataType { min_coverage: None });

        // Add forbidden code patterns for direct schema references if not already present
        if !manifest_rules
            .iter()
            .any(|r| matches!(r, ManifestSpecificRuleConfig::HasForbiddenCode { .. }))
        {
            manifest_rules.push(ManifestSpecificRuleConfig::HasForbiddenCode {
                forbidden_patterns: vec![],
                case_sensitive: false,
            });
        }
    }

    // 5. Ask for additional manifest rules
    let available_manifest_rules: Vec<_> = ManifestSpecificRuleConfig::iter()
        .filter(|r| {
            !manifest_rules
                .iter()
                .any(|existing| std::mem::discriminant(existing) == std::mem::discriminant(r))
        })
        .collect();

    if !available_manifest_rules.is_empty() {
        let selected_manifest = MultiSelect::new(
            "Select additional manifest rules (optional). These will provide examples for you to use:",
            available_manifest_rules,
        )
        .prompt()
        .map_err(|e| format!("Failed to get additional manifest rules: {e}"))?;

        for rule in selected_manifest {
            manifest_rules.push(rule);
        }
    }

    // 6. Ask for additional catalog rules
    let available_catalog_rules: Vec<_> = CatalogSpecificRuleConfig::iter()
        .filter(|r| {
            !catalog_rules
                .iter()
                .any(|existing| std::mem::discriminant(existing) == std::mem::discriminant(r))
        })
        .collect();

    if !available_catalog_rules.is_empty() {
        let selected_catalog = MultiSelect::new(
            "Select additional catalog rules (optional). These will provide examples for you to use.",
            available_catalog_rules,
        )
        .prompt()
        .map_err(|e| format!("Failed to get additional catalog rules: {e}"))?;

        for rule in selected_catalog {
            catalog_rules.push(rule);
        }
    }

    // 7. Auto-parse command detection
    let auto_parse_command =
        detect_and_ask_auto_parse().map_err(|e| format!("Failed to configure auto-parse: {e}"))?;

    Ok(QuestionnaireResult {
        format,
        naming_convention,
        data_model,
        manifest_rules,
        catalog_rules,
        auto_parse_command,
    })
}

/// Detect whether `uv` or `poetry` is used (via lock files) and ask the user
/// to confirm the auto-parse command.
fn detect_and_ask_auto_parse() -> Result<Option<String>, inquire::InquireError> {
    let has_uv = Path::new("uv.lock").exists();
    let has_poetry = Path::new("poetry.lock").exists();

    let detected = if has_uv {
        println!(
            "\n{} {} detected!",
            "ℹ".bright_blue().bold(),
            "uv".bright_cyan().bold()
        );
        Some("uv run dbt parse")
    } else if has_poetry {
        println!(
            "\n{} {} detected!",
            "ℹ".bright_blue().bold(),
            "poetry".bright_cyan().bold()
        );
        Some("poetry run dbt parse")
    } else {
        None
    };

    let mut options = Vec::new();

    if let Some(cmd) = detected {
        // Detected tool goes first with "(recommended)" label
        options.push(format!("{cmd} (recommended)"));
        for opt in ["dbt parse", "uv run dbt parse", "poetry run dbt parse"] {
            if opt != cmd {
                options.push(opt.to_string());
            }
        }
    } else {
        // Nothing detected — plain list, no recommended label
        options.push("dbt parse".to_string());
        options.push("uv run dbt parse".to_string());
        options.push("poetry run dbt parse".to_string());
    }
    options.push("Skip (no auto-parse)".to_string());

    let selected = Select::new(
        "Which command should dbtective use for auto-parsing? (used with --auto-parse flag)",
        options,
    )
    .with_starting_cursor(0)
    .with_help_message(
        "Recommended for pre-commit hooks and CI. Adds a performance penalty as dbt parses your project first.",
    )
    .prompt()?;

    if selected.contains("Skip") {
        Ok(None)
    } else {
        // Strip the " (recommended)" suffix if present
        let command = selected.strip_suffix(" (recommended)").unwrap_or(&selected);
        Ok(Some(command.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    // ========================================================================
    // Enum Display Tests
    // ========================================================================

    #[test]
    fn test_config_format_display() {
        assert_eq!(ConfigFormat::Yaml.to_string(), "YAML (recommended)");
        assert_eq!(ConfigFormat::Toml.to_string(), "TOML");
        assert_eq!(ConfigFormat::Pyproject.to_string(), "pyproject.toml");
    }

    #[test]
    fn test_data_model_display() {
        assert_eq!(
            DataModel::Medallion.to_string(),
            "Medallion (bronze, silver, gold)"
        );
        assert_eq!(
            DataModel::Common.to_string(),
            "Common (staging, intermediate, marts)"
        );
        assert_eq!(
            DataModel::None.to_string(),
            "None (no folder structure enforcement)"
        );
    }

    #[test]
    fn test_strictness_display() {
        assert_eq!(
            Strictness::Basic.to_string(),
            "Basic - Essential rules only"
        );
        assert_eq!(
            Strictness::Standard.to_string(),
            "Standard - Balanced set of rules"
        );
        assert_eq!(
            Strictness::Strict.to_string(),
            "Strict (recommended) - Comprehensive rule coverage"
        );
    }

    // ========================================================================
    // Init Trait Tests
    // ========================================================================

    #[test]
    fn test_manifest_rule_init_description() {
        let descriptions: Vec<_> = ManifestSpecificRuleConfig::iter()
            .map(|r| r.init_description())
            .collect();
        assert!(descriptions.contains(&"has_description - Require descriptions for resources"));
        assert!(descriptions.contains(&"name_convention - Enforce naming conventions"));
        assert!(descriptions.contains(&"has_tags - Require specific tags"));
        assert!(descriptions.contains(&"is_not_orphaned - Check for orphaned sources"));
        assert!(descriptions.contains(&"has_unique_test - Require uniqueness tests"));
        assert!(descriptions.contains(&"has_contract_enforced - Require enforced contracts"));
        assert!(descriptions.contains(&"has_metadata_keys - Require metadata keys (e.g., owner)"));
        assert!(descriptions.contains(&"has_refs - Require use of ref() function"));
        assert!(descriptions.contains(&"max_code_lines - Limit code line count"));
        assert!(
            descriptions.contains(&"code_contains_refs - Require ref()/source() calls in SQL code")
        );
        assert!(descriptions.contains(&"allowed_subfolders - Restrict subfolder usage"));
        assert!(descriptions.contains(&"sources_have_loader - Require loader for sources"));
    }

    #[test]
    fn test_catalog_rule_init_description() {
        let descriptions: Vec<_> = CatalogSpecificRuleConfig::iter()
            .map(|r| r.init_description())
            .collect();
        assert!(descriptions.contains(&"columns_all_documented - All columns must exist in docs"));
        assert!(descriptions.contains(&"columns_have_description - All columns need descriptions"));
        assert!(
            descriptions.contains(&"columns_name_convention - Enforce column naming conventions")
        );
        assert!(descriptions.contains(&"columns_canonical_name - Enforce canonical column names"));
        assert!(
            descriptions.contains(&"columns_have_data_type - Columns must have data types defined")
        );
    }

    // ========================================================================
    // Enum Iterator Tests
    // ========================================================================

    #[test]
    fn test_manifest_rule_iter_count() {
        assert_eq!(ManifestSpecificRuleConfig::iter().count(), 17);
    }

    #[test]
    fn test_catalog_rule_iter_count() {
        assert_eq!(CatalogSpecificRuleConfig::iter().count(), 5);
    }
}
