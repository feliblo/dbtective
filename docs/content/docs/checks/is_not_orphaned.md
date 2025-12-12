---
title: is_not_orphaned
type: docs
prev: docs/checks
sidebar:
  open: true
---

### Check: `is_not_orphaned`

<span class="check-category-badge badge-manifest">Manifest Check</span>

<details open>
<summary>is_not_orphaned details</summary>
<br>
This check ensures that dbt objects (models, seeds, sources) are being referenced by other objects in your project. An object is considered "orphaned" if it has no child objects consuming it, or if it's only referenced by non-allowed object types.

An object is not orphaned if it is referenced by at least one `allowed_references` using:

```
{{ source("source_schema", "source_name") }}
{{ ref("model_name") }}
```

This helps identify unused or underutilized data assets that may be candidates for removal or refactoring.

---

**Configuration**

- **type**: Must be `is_not_orphaned`.
- **allowed_references**: *(optional)* List of object types that count as valid consumers of the checked objects.
  - Default: `["models"]`
  - Options: `models`, `snapshots`, `exposures`, `unit_tests`
  - This indicates which objects count as val
- **applies_to**: *(optional)* List of dbt object types to check.
  - Default: `["sources"]`
  - Options: `models`, `seeds`, `sources`

{{< include-markdown "content/snippets/common_check_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "all_sources_cannot_be_orphaned"
    type: "is_not_orphaned"
    description: "All sources should be referenced by at least one model."

  - name: "models_and_sources_cannot_be_orphaned_except_marts"
    type: "is_not_orphaned"
    applies_to: ["models", "seeds"]
    excludes: ["models/marts/*"]
    allowed_references: ["models", "exposures"]
    severity: "warning"
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "all_sources_cannot_be_orphaned"
type = "is_not_orphaned"
description = "All sources should be referenced by at least one model."

[[manifest_tests]]
name = "models_and_sources_cannot_be_orphaned_except_marts"
type = "is_not_orphaned"
applies_to = ["models", "seeds"]
excludes = ["models/marts/*"]
allowed_references = ["models", "exposures"]
severity = "warning"
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "all_sources_cannot_be_orphaned"
type = "is_not_orphaned"
description = "All sources should be referenced by at least one model."

[[tool.dbtective.manifest_tests]]
name = "models_and_sources_cannot_be_orphaned_except_marts"
type = "is_not_orphaned"
applies_to = ["models", "seeds"]
excludes = ["models/marts/*"]
allowed_references = ["models", "exposures"]
severity = "warning"
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```sql
{{ ref("your_model_name") }}
{{ source("source_schema", "your_source_name") }}
```

</details>
