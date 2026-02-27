---
title: has_refs
type: docs
prev: docs/rules
sidebar:
  open: true
---

### Rule: `has_refs`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>has_refs details</summary>
<br>
This rule ensures that dbt objects have at least one upstream reference. An upstream reference is created using <code>ref()</code> or <code>source()</code> in your dbt model. This rule checks the dependency graph (<code>depends_on.nodes</code>) in the manifest.

This may indicate that you're using hardcoded SQL to reference data directly from the warehouse instead of leveraging dbt's dependency management. Or that an object is simply not being used.

Also check out [`code_contains_refs`](../code#rule-code_contains_refs), which inspects the raw SQL text directly for <code>ref()</code> / <code>source()</code>. It differs since <code>has_refs</code> also checks the <a href="https://docs.getdbt.com/reference/dbt-jinja-functions/ref"><code>depends_on config</code></a>.

---

**Configuration**

- **type**: Must be `has_refs`.
- **applies_to**: _(optional)_ List of dbt object types to include.
  - Default: `["models", "snapshots", "analyses"]`
  - Options: `models`, `seeds`, `snapshots`, `analyses`, `semantic_models`, `functions`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "references_must_exist"
    type: "has_refs"
    description: "All dbt objects must reference at least one source or model."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds']  (optional)
    # includes: ["models/staging/*"]
    # excludes: ["models/base/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "references_must_exist"
type = "has_refs"
description = "All dbt objects must reference at least one source or model."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["models/staging/*"]
# excludes = ["models/base/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "references_must_exist"
type = "has_refs"
description = "All dbt objects must reference at least one source or model."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["models/staging/*"]
# excludes = ["models/base/*"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```sql
-- Valid model with references
select
    customer_id,
    first_name,
    last_name
from {{ source('raw', 'customers') }}
```

```sql
-- Valid model referencing another model
select
    customer_id,
    order_count
from {{ ref('stg_customers') }}
```

</details>

</details>
