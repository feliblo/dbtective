---
title: Creating a New Rule
weight: 2
---

This guide walks you through creating a new rule (check) for dbtective.

## Prerequisites

Before starting, determine whether your rule is a **Manifest Check** or a **Catalog Check**:

- **Manifest checks** use only `manifest.json` - contains model definitions, configs, and metadata
- **Catalog checks** use both `manifest.json` and `catalog.json` - includes actual database column information

See the [checks overview](/docs/checks) or the dbt docs on [manifest](https://docs.getdbt.com/reference/artifacts/manifest-json) and [catalog](https://docs.getdbt.com/reference/artifacts/catalog-json) artifacts.

---

## Creating a new rule

Here I explain how to add a new `ManifestRule`. For a catalog rule, add your rule to the `CatalogSpecificRuleConfig` enum in `src/core/config/catalog_rule.rs` and follow the same steps as above (with the compiler helping you along the way). It works almost identically.

### Step 1: Add the Rule Enum Variant

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

### Step 2: Configure Applies To

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

### Step 3: Create or Find a Trait

Check `src/core/checks/rules/` for an existing trait that matches your rule's needs:

| Trait | Purpose | File |
|-------|---------|------|
| `Descriptable` | Objects with descriptions | `has_description.rs` |
| `HasTags` | Objects with tags | `has_tags.rs` |
| `HasMetadata` | Objects with metadata | `has_metadata_keys.rs` |
| `Nameable` | Objects with names | `name_convention.rs` |

**If a suitable trait exists**, add your function to it. **If not**, create a new file in `src/core/checks/rules/`.

If re-using a trait, we might need to rename some files. This is okay since the trait represents a broader concept.

All traits contain at least the following methods (needed for RuleResult (table reporting)):
- `fn get_object_type(&self) -> &str;` - Returns the dbt object type (e.g., "model", "source")
- `fn get_object_string(&self) -> &str;` - Returns a string representation
- `fn get_relative_path(&self) -> Option<&String>;` - Returns the object's relative file path


### Step 4: Implement the Rule Logic

Create your rule function. Here's an example pattern:

```rust
// src/core/checks/rules/has_owner.rs
use crate::{
    cli::table::RuleResult,
    core::config::manifest_rule::ManifestRule,
};

/// Trait for objects that can have an owner
pub trait YourRule {
    fn get_owner(&self) -> Option<&str>;
    fn get_object_type(&self) -> &str;
    fn get_object_string(&self) -> &str;
    fn get_relative_path(&self) -> Option<&String>;
}

/// Check if an object has a valid owner configured
pub fn has_your_rule<T: YourRule>(
    obj: &T,
    rule: &ManifestRule,
    your_field_name_for_the_rule: &str,
) -> Option<RuleResult> {
    // Your rule logic here
}
```

### Step 5: Implement the Trait for dbt Objects

Implement your trait for the relevant structs in `src/core/manifest/dbt_objects/`.
Don't worry if you miss any, the Rust compiler will guide you, (so you can also skip this for now and move to step 6).

```rust
// In the appropriate dbt_objects file

impl YourRule for Node {
    fn get_owner(&self) -> Option<&str> {
        self.config.as_ref()?.meta.as_ref()?.get("owner")?.as_str()
    }

    fn get_object_type(&self) -> &str {
        &self.resource_type
    }

    fn get_object_string(&self) -> &str {
        &self.name
    }

    fn get_relative_path(&self) -> Option<&String> {
        self.original_file_path.as_ref()
    }
}
```

### Step 6: Add the Rule in Node Checks

Add your rule to `src/core/checks/manifest/node_checks.rs`. If you haven't implemented the trait yet, you will get a compile error prompting you to do so.

```rust
use crate::core::checks::rules::your_rule;

// In the apply_node_checks function, add to the match statement:
let check_row_result = match &rule.rule {
    // ... existing rules ...

    ManifestSpecificRuleConfig::YourRuleName {
        your_field_name_for_the_rule,
        allow_empty,
    } => your_rule(node, rule, your_field_name_for_the_rule, *allow_empty),
};
```

Similarly, update `src/core/checks/manifest/other_manifest_object_checks.rs`.

If any object doesn't apply to your rule, simply return the accumulator of ruleresults (acc) unchanged.

### Step 7: Export the Module

Add your new module to `src/core/checks/rules/mod.rs`:

```rust
mod your_rule;
pub use your_rule::{your_rule, YourRule};
```

### Step 8: Write Unit Tests

Add tests in your rule file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{manifest_rule::ManifestSpecificRuleConfig, severity::Severity};

    struct TestObject {
        your_field_name_for_the_rule: Option<String>,
    }

    impl YourRule for TestObject {
        fn get_owner(&self) -> Option<&str> {
            self.your_field_name_for_the_rule.as_deref()
        }
        fn get_object_type(&self) -> &str { "model" }
        fn get_object_string(&self) -> &str { "test_model" }
        fn get_relative_path(&self) -> Option<&String> { None }
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

### Step 9: Write Integration Tests

Create tests in the `tests/` folder. Copy the structure from existing tests and adapt it to your rule.


### Step 10: Document the Rule

Create documentation in `docs/content/docs/checks/your_rule.md`. Copy the structure from existing rule docs & fill in the details to fit your rule. Remember to include the applies_to options from the `src/core/config/manifest_rule.rs` file.

## Tips

- The Rust compiler will guide you through missing implementations after you filled in the original Enum, so relax and take it step by step.
- Look at existing rules for patterns
- Ctrll+F on existing rules to show what needs to be updated.
