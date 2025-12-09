---
title: Naming Conventions
type: docs
prev: docs/checks
sidebar:
  open: true
---


### Check: `name_convention`

<br>
<details open>
<summary>name_convention details</summary>
<br>
This check ensures that a dbt object's name applies to naming conventions given in the arguments.

---

**Configuration**

- **type**: Must be `name_convention`.
- **applies_to**: *(optional)* List of dbt object types to check.
  - Default: `["models", "seeds", "snapshots", "analyses", "sources", "unit_tests", "macros", "exposures", "semantic_models"]`
  - Options: `models`, `seeds`, `snapshots`, `analyses`, `sources`, `unit_tests`, `macros`, `exposures`, `semantic_models`
- **pattern**: The naming convention pattern to enforce. Can be one of the following presets or a custom regex pattern.
  - Presets:
    - `snake_case`: lowercase letters, numbers, and underscores (e.g., `my_model_name`)
    - `kebab-case`: lowercase letters, numbers, and hyphens (e.g., `my-model-name`)
    - `camelCase`: starts with a lowercase letter, followed by uppercase letters for new words (e.g., `myModelName`)
    - `PascalCase`: starts with an uppercase letter, followed by uppercase letters for new words (e.g., `MyModelName`)
  - Custom Regex: Any valid regex pattern to match against the dbt object names.

{{< include-markdown "content/snippets/common_check_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "all_objects_snake_case"
    type: "name_convention"
    description: "All dbt objects must be snake_case."
    pattern: "snake_case"
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds']  (optional)
    # includes: ["path/to/include/*"]  (optional)
    # excludes: ["path/to/exclude/*"]  (optional)
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "all_objects_snake_case"
type = "name_convention"
description = "All dbt objects must be snake_case."
pattern = "snake_case"
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]  # (optional)
# excludes = ["path/to/exclude/*"]  # (optional)
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "all_objects_snake_case"
type = "name_convention"
description = "All dbt objects must be snake_case."
pattern = "snake_case"
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]  # (optional)
# excludes = ["path/to/exclude/*"]  # (optional)
```

{{< /tab >}}

{{< /tabs >}}

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
