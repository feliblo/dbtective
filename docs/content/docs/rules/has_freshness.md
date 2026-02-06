---
title: sources_have_freshness
type: docs
prev: docs/rules
sidebar:
  open: true
---

### Rule: `sources_have_freshness`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>sources_have_freshness details</summary>
<br>
This rule ensures that dbt sources have a <a href="https://docs.getdbt.com/docs/deploy/source-freshness" target="_blank">freshness</a> configuration defined. Freshness defines the acceptable amount of time between the most recent record and now, for a table to be considered "fresh". At least one of `warn_after` or `error_after` must have a non-null count.

---

**Configuration**

- **type**: Must be `sources_have_freshness`.
- **applies_to**: _(optional)_ List of dbt object types to check.
  - Default: `["sources"]`
  - Options: `sources`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "sources_have_freshness"
    type: "sources_have_freshness"
    description: "All sources must have freshness configured."
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "sources_have_freshness"
type = "sources_have_freshness"
description = "All sources must have freshness configured."
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "sources_have_freshness"
type = "sources_have_freshness"
description = "All sources must have freshness configured."
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```yaml
sources:
  - name: jaffle_shop
    freshness:
      warn_after:
        count: 24
        period: hour
      error_after:
        count: 48
        period: hour
    loaded_at_field: updated_at
    tables:
      - name: orders
      - name: customers

  - name: stripe
    freshness:
      warn_after:
        count: 12
        period: hour
    loaded_at_field: created_at
    tables:
      - name: payments
```

</details>

</details>
