---
title: Naming Conventions
type: docs
prev: docs/checks
sidebar:
  open: true
---


### Check: `naming_convention`

<br>
<details open>
<summary>naming_convention details</summary>
<br>
This check ensures that a dbt object's name applies to naming conventions given in the arguments.

---

**Configuration**

- **type**: Must be `naming_convention`.
- **applies_to**: *(optional)* List of node types to check.
  - Default: `["models", "sources", "seeds", "exposures", "snapshots", "analyses", "macros", "tests"]`
  - Options: `models`, `sources`, `seeds`, `exposures`, `snapshots`, `analyses`, `macros`, `tests`
- **pattern**: The naming convention pattern to enforce. Can be one of the following presets or a custom regex pattern.
  - Presets:
    - `snake_case`: lowercase letters, numbers, and underscores (e.g., `my_model_name`)
    - `kebab-case`: lowercase letters, numbers, and hyphens (e.g., `my-model-name`)
    - `camelCase`: starts with a lowercase letter, followed by uppercase letters for new words (e.g., `myModelName`)
    - `PascalCase`: starts with an uppercase letter, followed by uppercase letters for new words (e.g., `MyModelName`)
  - Custom Regex: Any valid regex pattern to match against the dbt object names.

{{< include-markdown "content/snippets/common_check_config.md" >}}

**Example Config**

```yaml
manifest_tests:
  - name: "all_objects_snake_case"
    type: "naming_convention"
    description: "All dbt dbt objects must be snake_case."
    pattern: "snake_case"
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds'] (optional
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]

```

<details closed>
<summary>Relevant dbt code</summary>

```yaml
models:
  - name: model_with_description
    description: This is a model with a description
  - name: model_without_description
```

</details>

</details>
