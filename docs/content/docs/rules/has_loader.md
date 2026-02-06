---
title: sources_have_loader
type: docs
prev: docs/rules
sidebar:
  open: true
---

### Rule: `sources_have_loader`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>sources_have_loader details</summary>
<br>
This rule ensures that dbt sources have a <a href="https://docs.getdbt.com/reference/resource-properties/loader" target="_blank">loader</a> property defined. The loader describes the tool that loads data into your warehouse (e.g., Fivetran, Stitch, Airflow) and functions only as documentation.

---

**Configuration**

- **type**: Must be `sources_have_loader`.
- **applies_to**: _(optional)_ List of dbt object types to check.
  - Default: `["sources"]`
  - Options: `sources`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "sources_have_loader"
    type: "sources_have_loader"
    description: "All sources must specify their loader."
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "sources_have_loader"
type = "sources_have_loader"
description = "All sources must specify their loader."
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "sources_have_loader"
type = "sources_have_loader"
description = "All sources must specify their loader."
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```yaml
sources:
  - name: jaffle_shop
    loader: fivetran
    tables:
      - name: orders
      - name: customers

  - name: stripe
    loader: stitch
    tables:
      - name: payments
```

</details>

</details>
