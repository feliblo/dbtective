---
title: Creating a New Rule
weight: 2
---

This guide walks you through creating a new rule for dbtective.

## Prerequisites

Before starting, determine whether your rule is a **Manifest Rule** or a **Catalog Rule**:

- **Manifest rules** use only `manifest.json` - contains model definitions, configs, and metadata
- **Catalog rules** use both `manifest.json` and `catalog.json` - includes actual database column information

See the [rules overview](/docs/rules) or the dbt docs on [manifest](https://docs.getdbt.com/reference/artifacts/manifest-json) and [catalog](https://docs.getdbt.com/reference/artifacts/catalog-json) artifacts.

---

## Creating a new rule

Here I explain how to add a new `ManifestRule`. For a catalog rule, add your rule to the `CatalogSpecificRuleConfig` enum in `src/core/config/catalog_rule.rs` and follow the same steps as above (with the compiler helping you along the way). It works almost identically.

{{% steps %}}

### Add the Rule Enum Variant

Add a new entry to the `ManifestSpecificRuleConfig` enum in `src/core/config/manifest_rule.rs`:

```rust
#[derive(Debug, Deserialize, EnumIter, AsRefStr, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ManifestSpecificRuleConfig {
    HasDescription {},
    // ... existing rules ...

    // Add your new rule here
    YourRuleName {
        your_field_name_for_the_rule: String,
    },
}
```

- Use `PascalCase` for the enum variant name
- The `snake_case` serialization converts it automatically (e.g., `HasOwner` → `has_owner` in confor the user config)
- Add rule-specific arguments inside the `{}` braces

### Configure Applies To

In the same file, update two functions:

**`default_applies_to_for_manifest_rule`** - Default targets when user doesn't specify:

```rust
pub fn default_applies_to_for_manifest_rule(rule_type: &ManifestSpecificRuleConfig) -> AppliesTo {
    match rule_type {
        // ... existing rules ...

        ManifestSpecificRuleConfig::HasOwner { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models],
            source_objects: vec![RuleTarget::Sources],
            unit_test_objects: vec![],
            macro_objects: vec![],
            exposure_objects: vec![],
            semantic_model_objects: vec![],
            custom_objects: vec![],
        },
    }
}
```

**`applies_to_options_for_manifest_rule`** - All valid targets users can choose:

```rust
fn applies_to_options_for_manifest_rule(rule_type: &ManifestSpecificRuleConfig) -> AppliesTo {
    match rule_type {
        // ... existing rules ...

        ManifestSpecificRuleConfig::HasOwner { .. } => AppliesTo {
            node_objects: vec![RuleTarget::Models, RuleTarget::Seeds, RuleTarget::Snapshots],
            source_objects: vec![RuleTarget::Sources],
            // ... etc
        },
    }
}
```

### Create or Find a Trait

Check `src/core/rules/rule_config/` for an existing trait that matches your rule's needs:

| Trait | Purpose | File |
|-------|---------|------|
| `Descriptable` | Objects with descriptions | `has_description.rs` |
| `HasTags` | Objects with tags | `has_tags.rs` |
| `HasMetadata` | Objects with metadata | `has_metadata_keys.rs` |
| `Nameable` | Objects with names | `name_convention.rs` |

**If a trait exists**, add your function to it. **If not**, create a new file in `src/core/rules/rule_config/`.

All traits need to extend the `Identifiable` supertrait (defined in `src/core/rules/common_traits.rs`), which provides the following methods needed for RuleResult (table reporting):

```rust
pub trait Identifiable {
    fn get_object_type(&self) -> &str;      // Returns the dbt object type (e.g., "model", "source")
    fn get_object_string(&self) -> &str;    // Returns a string representation
    fn get_relative_path(&self) -> Option<&String> { None }  // Returns the object's relative file path
}
```

Your trait needs to extend `Identifiable`:

```rust
pub trait YourTrait: Identifiable {
    // Your trait-specific methods here
}
```

### Implement the Rule Logic

Create your rule function. Here's an example pattern:

```rust
// src/core/rules/rule_config/your_rule.rs
use crate::{
    cli::table::RuleResult,
    core::{config::manifest_rule::ManifestRule, rules::common_traits::Identifiable},
};

/// Trait for objects that can have an owner - extends Identifiable
pub trait YourRule: Identifiable {
    fn get_owner(&self) -> Option<&str>;
}

/// Check if an object has a valid owner configured
pub fn has_your_rule<T: YourRule>(
    obj: &T,
    rule: &ManifestRule,
    your_field_name_for_the_rule: &str,
) -> Option<RuleResult> {
    // Your rule logic here
    // Use obj.get_object_type(), obj.get_object_string(), obj.get_relative_path()
    // from the Identifiable supertrait
}
```

### Implement the Trait for dbt Objects

Implement your trait for the relevant structs in `src/core/manifest/`.
Don't worry if you miss any, the Rust compiler will guide you, (so you can also skip this for now and move to step 6).

Note: The `Identifiable` trait is already implemented for all dbt objects (Node, Source, Exposure, etc.) in their respective `*_impls.rs` files. You only need to implement your trait-specific methods:

```rust
// In the appropriate *_impls.rs file (e.g., src/core/manifest/node_impls.rs)

// Identifiable is already implemented for Node, so you only need:
impl YourRule for Node {
    fn get_owner(&self) -> Option<&str> {
        self.config.as_ref()?.meta.as_ref()?.get("owner")?.as_str()
    }
}
```

### Add the Rule in Node Rules

Add your rule to `src/core/rules/manifest/node_rules.rs`. If you haven't implemented the trait yet, you will get a compile error prompting you to do so.

```rust
use crate::core::rules::rule_config::your_rule;

// In the apply_node_rules function, add to the match statement:
let row_result = match &rule.rule {
    // ... existing rules ...

    ManifestSpecificRuleConfig::YourRuleName {
        your_field_name_for_the_rule,
        allow_empty,
    } => your_rule(node, rule, your_field_name_for_the_rule, *allow_empty),
};
```

Similarly, update `src/core/rules/manifest/apply_other_manifest_object_rules.rs`.

If any object doesn't apply to your rule, simply return the accumulator of ruleresults (acc) unchanged.

### Export the Module

Add your new module to `src/core/rules/rule_config/mod.rs`:

```rust
mod your_rule;
pub use your_rule::{your_rule, YourRule};
```

### Write Unit Tests

Add tests in your rule file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};
    use crate::core::rules::common_traits::Identifiable;

    struct TestObject {
        your_field_name_for_the_rule: Option<String>,
    }

    // First implement Identifiable for the test struct
    impl Identifiable for TestObject {
        fn get_object_type(&self) -> &str { "model" }
        fn get_object_string(&self) -> &str { "test_model" }
        fn get_relative_path(&self) -> Option<&String> { None }
    }

    // Then implement your trait (which extends Identifiable)
    impl YourRule for TestObject {
        fn get_owner(&self) -> Option<&str> {
            self.your_field_name_for_the_rule.as_deref()
        }
    }

    #[test]
    fn test_missing_owner() {
        let obj = TestObject { your_field_name_for_the_rule: None };
        let rule = create_test_rule();

        let result = your_rule(&obj, &rule, "your_field_name_for_the_rule", false);

        assert!(result.is_some());
        assert!(result.unwrap().message.contains("some message e.g. missing owner"));
    }

    #[test]
    fn test_valid_owner() {
        let obj = TestObject { your_field_name_for_the_rule: Some("team-data".to_string()) };
        let rule = create_test_rule();

        let result = your_rule(&obj, &rule, "your_field_name_for_the_rule", false);

        assert!(result.is_none());
    }
}
```

### Write Integration Tests

Create tests in the `tests/` folder. Copy the structure from existing tests and adapt it to your rule.

### Add to Init Config

Update the init system so users can select your rule during `dbtective init`:

**1. Add Init trait description** in `src/core/init/questionnaire.rs`:

```rust
impl Init for ManifestSpecificRuleConfig {
    fn init_description(&self) -> &'static str {
        match self {
            // ... existing rules ...

            Self::YourRuleName { .. } => "your_rule_name - Brief description of the rule",
        }
    }
}
```

**2. Add rule creation logic** in `src/core/init/config_builder.rs`:

```rust
fn create_manifest_rule(
    rule: &ManifestSpecificRuleConfig,
    result: &QuestionnaireResult,
) -> ManifestSpecificRuleConfig {
    match rule {
        // ... existing rules ...

        ManifestSpecificRuleConfig::YourRuleName { .. } => {
            ManifestSpecificRuleConfig::YourRuleName {
                your_field_name_for_the_rule: "default_value".to_string(),
            }
        }
    }
}
```

**3. Add YAML/TOML serialization** in `config_builder.rs`:

```rust
fn manifest_rule_to_yaml(rule: &ManifestSpecificRuleConfig) -> String {
    match rule {
        // ... existing rules ...

        ManifestSpecificRuleConfig::YourRuleName { your_field_name_for_the_rule } => {
            format!(
                r#"  - name: "your_rule_name"
    type: "your_rule_name"
    your_field_name_for_the_rule: "{your_field_name_for_the_rule}""#
            )
        }
    }
}

fn manifest_rule_to_toml(rule: &ManifestSpecificRuleConfig, section: &str) -> String {
    match rule {
        // ... existing rules ...

        ManifestSpecificRuleConfig::YourRuleName { your_field_name_for_the_rule } => {
            format!(
                r#"[[{section}]]
name = "your_rule_name"
type = "your_rule_name"
your_field_name_for_the_rule = "{your_field_name_for_the_rule}""#
            )
        }
    }
}
```

### Document the Rule

Create documentation in `docs/content/docs/rules/your_rule.md`. Copy the structure from existing rule docs & fill in the details to fit your rule. Remember to include the applies_to options from the `src/core/config/manifest_rule.rs` file.

{{% /steps %}}

## Tips

- The Rust compiler will guide you through missing implementations after you filled in the original Enum, so relax and take it step by step.
- Look at existing rules for patterns
- Ctrl+F on existing rules to show what needs to be updated.
