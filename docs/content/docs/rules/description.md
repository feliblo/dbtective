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
- **applies_to**: *(optional)* List of dbt object types to include.
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
