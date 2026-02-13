---
title: has_description
type: docs
prev: docs/rules
sidebar:
  open: true
---

### Rule: `has_description`

<br>
<details open>
<summary>has_description details</summary>
<br>
This rule ensures that every dbt object has a description provided in the configuration.

---

**Configuration**

- **type**: Must be `has_description`.
- **min_length**: _(optional)_ Minimum number of characters (after trimming whitespace) the description must have. If not set, any non-empty description passes.
- **forbidden_substrings**: _(optional)_ A list of strings that the description must **not** contain. Useful for catching placeholder descriptions like `"TODO"` or `"test_description"`.
- **applies_to**: _(optional)_ List of dbt object types to include.
  - Default: `["models", "seeds", "snapshots", "sources", "unit_tests", "macros", "exposures", "semantic_models"]`
  - Options: `models`, `seeds`, `snapshots`, `sources`, `unit_tests`, `macros`, `exposures`, `semantic_models`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "everything_has_description"
    type: "has_description"
    description: "Everything must have a description."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds'] (optional)
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]

  - name: "meaningful_descriptions"
    type: "has_description"
    description: "Descriptions must be meaningful."
    min_length: 10
    forbidden_substrings: ["TODO", "test_description", "placeholder"]
    applies_to:
      - "models"
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "everything_has_description"
type = "has_description"
description = "Everything must have a description."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]

[[manifest_tests]]
name = "meaningful_descriptions"
type = "has_description"
description = "Descriptions must be meaningful."
min_length = 10
forbidden_substrings = ["TODO", "test_description", "placeholder"]
applies_to = ["models"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "everything_has_description"
type = "has_description"
description = "Everything must have a description."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]

[[tool.dbtective.manifest_tests]]
name = "meaningful_descriptions"
type = "has_description"
description = "Descriptions must be meaningful."
min_length = 10
forbidden_substrings = ["TODO", "test_description", "placeholder"]
applies_to = ["models"]
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
