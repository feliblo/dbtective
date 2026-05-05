---
title: node_dependency
type: docs
prev: docs/rules
sidebar:
  open: true
---

### Rule: `node_dependency`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>node_dependency details</summary>
<br>
This rule forbids dependency relationships between nodes that match configurable name, path, or type patterns. It checks the <code>parent_map</code> in <code>manifest.json</code> to find the direct upstream dependencies of each node.

Use it to enforce DAG layering conventions — for example: staging models must not depend on other staging models, intermediate models must not bypass staging by referencing <code>source()</code> directly, or staging models must not reference downstream layers (inverted DAG).

One violation is produced per forbidden parent found.

---

**Configuration**

- **type**: Must be `node_dependency`.
- **from_name_pattern**: _(optional)_ Regex matched against the **dependent node's name**. If omitted, all nodes in `applies_to` are checked.
- **parent_name_pattern**: _(optional)_ Regex matched against the **parent node's name** (last segment of its unique ID). A parent must match all specified filters to trigger a violation.
- **parent_path_pattern**: _(optional)_ Regex matched against the **parent node's file path**. Only applies when the parent exists in `manifest.json` nodes or sources.
- **parent_type**: _(optional)_ Type filter for the parent. Valid values: `model`, `source`, `seed`, `snapshot`. AND-combined with any name/path patterns.
- **applies_to**: _(optional)_ List of dbt object types this rule checks.
  - Default: `["models"]`
  - Options: `models`, `snapshots`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  # Use case 1 — staging must not depend on staging
  - name: "no_stg_to_stg"
    type: "node_dependency"
    from_name_pattern: "^stg_"
    parent_name_pattern: "^stg_"
    severity: "error"

  # Use case 2 — intermediate/mart layers must not bypass staging
  - name: "no_bypass_staging"
    type: "node_dependency"
    from_name_pattern: "^(int_|fct_|dim_|obt_|rpt_)"
    parent_type: "source"
    severity: "error"

  # Use case 3 — no inverted DAG (staging must not depend on downstream)
  - name: "no_inverted_dag"
    type: "node_dependency"
    from_name_pattern: "^stg_"
    parent_name_pattern: "^(int_|fct_|dim_|obt_|rpt_)"
    severity: "error"
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "no_stg_to_stg"
type = "node_dependency"
from_name_pattern = "^stg_"
parent_name_pattern = "^stg_"
severity = "error"

[[manifest_tests]]
name = "no_bypass_staging"
type = "node_dependency"
from_name_pattern = "^(int_|fct_|dim_|obt_|rpt_)"
parent_type = "source"
severity = "error"

[[manifest_tests]]
name = "no_inverted_dag"
type = "node_dependency"
from_name_pattern = "^stg_"
parent_name_pattern = "^(int_|fct_|dim_|obt_|rpt_)"
severity = "error"
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "no_stg_to_stg"
type = "node_dependency"
from_name_pattern = "^stg_"
parent_name_pattern = "^stg_"
severity = "error"

[[tool.dbtective.manifest_tests]]
name = "no_bypass_staging"
type = "node_dependency"
from_name_pattern = "^(int_|fct_|dim_|obt_|rpt_)"
parent_type = "source"
severity = "error"

[[tool.dbtective.manifest_tests]]
name = "no_inverted_dag"
type = "node_dependency"
from_name_pattern = "^stg_"
parent_name_pattern = "^(int_|fct_|dim_|obt_|rpt_)"
severity = "error"
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Examples</summary>

**Use case 1 — No staging → staging dependency**

```sql
-- ❌ Non-compliant: stg_order_items selects from another staging model
-- models/staging/stg_order_items.sql
select order_id, product_id
from {{ ref('stg_orders') }}

-- ✅ Compliant
select order_id, product_id
from {{ source('shop', 'order_items') }}
```

**Use case 2 — Intermediate/mart models must not bypass staging**

```sql
-- ❌ Non-compliant: intermediate model reads source() directly
-- models/intermediate/int_order_summary.sql
select customer_id, count(*) as order_count
from {{ source('shop', 'orders') }}
group by 1

-- ✅ Compliant
select customer_id, count(*) as order_count
from {{ ref('stg_orders') }}
group by 1
```

**Use case 3 — No inverted DAG**

```sql
-- ❌ Non-compliant: staging model references a downstream layer
-- models/staging/stg_customers.sql
select * from {{ ref('int_customer_aggregates') }}

-- ✅ Compliant
select customer_id, first_name, last_name
from {{ source('crm', 'customers') }}
```

**Directory-based conventions also work** via `includes`/`excludes` for the `from` side and `parent_path_pattern` for the parent side:

```yaml
manifest_tests:
  - name: "no_staging_to_staging_by_path"
    type: "node_dependency"
    includes: ["models/staging"]
    parent_path_pattern: "models/staging"
    severity: "error"
```

</details>

</details>
