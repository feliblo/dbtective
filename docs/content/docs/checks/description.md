---
title: Description
type: docs
prev: docs/checks
sidebar:
  open: true
---


### Check: `has_description`

<br>
<details open>
<summary>has_description details</summary>
<br>
This check ensures that every dbt node (model, seed, source, macro, etc.) has a description provided in the configuration.

---

**Configuration**

- **type**: Must be `has_description`.
- **applies_to**: *(optional)* List of node types to check.
  - Default: `["models", "sources", "seeds", "exposures", "snapshots"]`
  - Options: `models`, `sources`, `seeds`, `exposures`, `snapshots`

{{< include-markdown "content/snippets/common_check_config.md" >}}

**Example Config**

```yaml
manifest_tests:
  - name: "everything_has_description"
    type: "has_description"
    description: "Everything must have a description."
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
