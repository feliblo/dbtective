---
title: has_metadata_keys
type: docs
prev: docs/rules
sidebar:
  open: true
---


### Rule: `has_metadata_keys`

<br>
<details open>
<summary>has_metadata_keys details</summary>
<br>
This rule ensures that dbt objects have specific required keys in their <code>meta</code> property. This is useful for enforcing governance standards such as requiring an owner, domain, or other organizational metadata.

---

**Configuration**

- **type**: Must be `has_metadata_keys`.
- **applies_to**: *(optional)* List of dbt object types to include.
  - Default: `["models", "sources"]`
  - Options: `models`, `seeds`, `snapshots`, `sources`, `macros`, `exposures`, `semantic_models`
- **required_keys**: List of metadata keys that must be present in the `meta` property of each dbt object.
- **custom_message** (Optional): Custom message to display when the rule fails. It will insert the {Object name} **before** the message.
  - The custom message `is missing an owner` would produce a message like `{Object name} is missing an owner`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "models_have_owner"
    type: "has_metadata_keys"
    description: "All models and sources must have an owner defined in meta."
    required_keys: ["owner"]
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds']  (optional)
    # includes: ["path/to/include/*"]  (optional)
    # excludes: ["path/to/exclude/*"]  (optional)
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "models_have_owner"
type = "has_metadata_keys"
description = "All models and sources must have an owner defined in meta."
required_keys = ["owner"]
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]  # (optional)
# excludes = ["path/to/exclude/*"]  # (optional)
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "models_have_owner"
type = "has_metadata_keys"
description = "All models and sources must have an owner defined in meta."
required_keys = ["owner"]
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
  - name: model_with_owner
    description: This model has an owner defined
    meta:
      owner: "data-team@example.com"
    # or
    config:
      meta:
        owner: "data-team@example.com"

  - name: model_without_owner
    description: This model is missing the owner key
```

</details>

</details>
