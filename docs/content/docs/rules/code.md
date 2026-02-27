---
title: code (5)
type: docs
prev: docs/rules
sidebar:
  open: true
---

These rules inspect the raw SQL/Jinja code of your dbt resources.

<hr style="border: 1px solid #666; margin: 2em 0;">

### Rule: `code_contains_refs`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>code_contains_refs details</summary>
<br>
This rule checks that the raw SQL code of a resource contains at least one <code>ref()</code> or <code>source()</code> function call. It inspects the actual code text (not the dependency graph), stripping SQL comments before checking. Detection is case-insensitive.

Resources without these calls may be hardcoding warehouse table names and bypassing dbt lineage tracking.
Also check out [`has_refs`](../has_refs), which checks the dependency graph (<code>depends_on.nodes</code>) rather than the raw SQL text. It differs since <code>has_refs</code> also checks the <a href="https://docs.getdbt.com/reference/dbt-jinja-functions/ref"><code>depends_on config</code></a>.

---

**Configuration**

- **type**: Must be `code_contains_refs`.
- **applies_to**: _(optional)_ List of dbt object types to include.
  - Default: `["models", "snapshots"]`
  - Options: `models`, `snapshots`, `macros`, `functions`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "code_must_use_refs"
    type: "code_contains_refs"
    description: "All SQL code must contain ref() or source() calls."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'snapshots']  (optional)
    # includes: ["models/staging/*"]
    # excludes: ["models/base/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "code_must_use_refs"
type = "code_contains_refs"
description = "All SQL code must contain ref() or source() calls."
# severity = "warning"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["models/staging/*"]
# excludes = ["models/base/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "code_must_use_refs"
type = "code_contains_refs"
description = "All SQL code must contain ref() or source() calls."
# severity = "warning"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["models/staging/*"]
# excludes = ["models/base/*"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```sql
-- models/good_model.sql (PASS - contains ref())
SELECT
    customer_id,
    first_name
FROM {{ ref('stg_customers') }}

-- models/good_source_model.sql (PASS - contains source())
SELECT *
FROM {{ source('raw', 'customers') }}

-- models/bad_model.sql (FAIL - hardcoded table name)
SELECT id, name
FROM raw_schema.users
WHERE active = true

-- models/tricky_model.sql (FAIL - ref is commented out)
-- SELECT * FROM {{ ref('stg_users') }}
SELECT id FROM raw_schema.users
```

</details>

</details>

<hr style="border: 2px solid #444; margin: 2em 0;">

### Rule: `code_forbidden_patterns`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>code_forbidden_patterns details</summary>
<br>
This rule checks if code contains forbidden patterns. Use it to enforce coding standards by flagging undesired patterns such as `SELECT *` statements, hardcoded references, or any other string patterns that should not appear in your dbt code.

---

**Configuration**

- **type**: Must be `code_forbidden_patterns`.
- **forbidden_patterns**: List of string patterns that are not allowed in the code. Each pattern is matched as a substring.
- **case_sensitive**: _(optional)_ Whether pattern matching should be case-sensitive. Defaults to `false` (case-insensitive).
- **applies_to**: _(optional)_ List of dbt object types to check.
  - Default: `["models", "snapshots", "macros", "functions"]`
  - Options: `models`, `snapshots`, `macros`, `functions`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  # Forbid SELECT * in models (case-insensitive by default)
  - name: "no_select_star"
    type: "code_forbidden_patterns"
    forbidden_patterns: ["SELECT *"]
    description: "Models should not use SELECT *."
    # case_sensitive: false  (optional, default)
    # severity: "warning"  (optional)
    # applies_to: ['models', 'snapshots'] (optional)
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]

  # Case-sensitive match for exact patterns
  - name: "no_hardcoded_schema"
    type: "code_forbidden_patterns"
    forbidden_patterns: ["raw_prod.", "analytics_prod."]
    case_sensitive: true
    severity: "warning"
    description: "Use dbt selectors"
```

{{< /tab >}}

{{< tab >}}

```toml
# Forbid SELECT * in models (case-insensitive by default)
[[manifest_tests]]
name = "no_select_star"
type = "code_forbidden_patterns"
forbidden_patterns = ["SELECT *"]
description = "Models should not use SELECT *."
# case_sensitive = false  # (optional, default)
# severity = "warning"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]

# Case-sensitive match for exact patterns
[[manifest_tests]]
name = "no_hardcoded_schema"
type = "code_forbidden_patterns"
forbidden_patterns = ["raw_prod.", "analytics_prod."]
case_sensitive = true
severity = "warning"
description = "Use dbt selectors"
```

{{< /tab >}}

{{< tab >}}

```toml
# Forbid SELECT * in models (case-insensitive by default)
[[tool.dbtective.manifest_tests]]
name = "no_select_star"
type = "code_forbidden_patterns"
forbidden_patterns = ["SELECT *"]
description = "Models should not use SELECT *."
# case_sensitive = false  # (optional, default)
# severity = "warning"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]

# Case-sensitive match for exact patterns
[[tool.dbtective.manifest_tests]]
name = "no_hardcoded_schema"
type = "code_forbidden_patterns"
forbidden_patterns = ["raw_prod.", "analytics_prod."]
case_sensitive = true
severity = "warning"
description = "Use dbt selectors"
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```sql
-- models/clean_model.sql (PASS - no forbidden patterns)
SELECT
    id,
    name
FROM users
WHERE active = true

-- models/star_model.sql (FAIL - contains 'SELECT *')
SELECT * FROM users
-- Also matches: select * from users (case-insensitive by default)
```

</details>

</details>

<hr style="border: 2px solid #444; margin: 2em 0;">

### Rule: `code_max_lines`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>code_max_lines details</summary>
<br>
This rule enforces a maximum line count for dbt code objects, helping to maintain code readability and encourage modular design. Objects with empty code will also be flagged by this rule.

---

**Configuration**

- **type**: Must be `code_max_lines`.
- **max_lines**: _(optional)_ The maximum number of lines allowed for the code. Defaults to `150`.
- **applies_to**: _(optional)_ List of dbt object types to include.
  - Default: `["models", "snapshots", "macros", "functions"]`
  - Options: `models`, `snapshots`, `macros`, `functions`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "models_max_100_lines"
    type: "code_max_lines"
    max_lines: 100
    description: "Models should not exceed 100 lines of code."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'snapshots'] (optional)
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "models_max_100_lines"
type = "code_max_lines"
max_lines = 100
description = "Models should not exceed 100 lines of code."
# severity = "warning"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "models_max_100_lines"
type = "code_max_lines"
max_lines = 100
description = "Models should not exceed 100 lines of code."
# severity = "warning"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```sql
-- models/short_model.sql (PASS)
SELECT
    id,
    name
FROM users

-- models/very_long_model.sql (FAIL - exceeds max_lines)
SELECT
    id,
    name,
    email,
    ...
    -- 101+ lines of SQL
    ...
FROM users

-- models/empty_model.sql (FAIL - empty code)
-- No content
```

</details>

<details closed>
<summary>Use cases</summary>

- Enforce code modularity by limiting file size
- Prevent overly complex transformations in single files
- Encourage breaking down large models into smaller, reusable CTEs or models
- Maintain consistent code readability across the project
- Catch accidentally empty SQL files

</details>

</details>

<hr style="border: 2px solid #444; margin: 2em 0;">

### Rule: `code_max_joins`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>code_max_joins details</summary>
<br>
This rule enforces a maximum number of JOINs in raw SQL code, helping to reduce code complexity and encourage modular design. SQL comments (single-line `--` and multi-line `/* */`) are stripped before counting, so commented-out JOINs are not counted. Detection is case-insensitive.

---

**Configuration**

- **type**: Must be `code_max_joins`.
- **max_joins**: _(optional)_ The maximum number of JOINs allowed. Defaults to `5`.
- **applies_to**: _(optional)_ List of dbt object types to include.
  - Default: `["models", "snapshots"]`
  - Options: `models`, `snapshots`, `macros`, `functions`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "limit_joins"
    type: "code_max_joins"
    max_joins: 3
    description: "Models should not exceed 3 JOINs."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'snapshots'] (optional)
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "limit_joins"
type = "code_max_joins"
max_joins = 3
description = "Models should not exceed 3 JOINs."
# severity = "warning"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "limit_joins"
type = "code_max_joins"
max_joins = 3
description = "Models should not exceed 3 JOINs."
# severity = "warning"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```sql
-- models/simple_model.sql (PASS - 1 JOIN, within limit of 3)
SELECT
    a.id,
    b.name
FROM {{ ref('users') }} a
JOIN {{ ref('orders') }} b ON a.id = b.user_id

-- models/complex_model.sql (FAIL - 4 JOINs, exceeds limit of 3)
SELECT a.id
FROM {{ ref('users') }} a
JOIN {{ ref('orders') }} b ON a.id = b.user_id
JOIN {{ ref('products') }} c ON b.product_id = c.id
JOIN {{ ref('categories') }} d ON c.cat_id = d.id
JOIN {{ ref('suppliers') }} e ON d.sup_id = e.id

-- models/commented_model.sql (PASS - commented JOINs are not counted)
-- JOIN old_table ON ...
/* LEFT JOIN another_table ON ... */
SELECT a.id
FROM {{ ref('users') }} a
JOIN {{ ref('orders') }} b ON a.id = b.user_id
```

</details>

</details>

<hr style="border: 2px solid #444; margin: 2em 0;">

### Rule: `code_no_hardcoded_refs`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>code_no_hardcoded_refs details</summary>
<br>
This rule detects hardcoded table references in raw SQL code — i.e. <code>schema.table</code> patterns after <code>FROM</code> or <code>JOIN</code> keywords. These should be replaced with <code>ref()</code> or <code>source()</code> calls to maintain dbt lineage tracking.

The rule supports:

- Two-part references: `schema.table`
- Three-part references: `db.schema.table`
- Quoted identifiers: `"schema"."table"`, `` `schema`.`table` ``, `[schema].[table]`
- Multiline: `FROM` or `JOIN` on one line, table reference on the next
- All JOIN types: `JOIN`, `LEFT JOIN`, `INNER JOIN`, `CROSS JOIN`, etc.

The rule does **not** trigger on:

- CTE references (single unqualified names like `FROM my_cte`)
- Column selects (e.g. `a.column_name` in SELECT)
- Commented-out code (SQL comments are stripped before checking)

---

**Configuration**

- **type**: Must be `code_no_hardcoded_refs`.
- **applies_to**: _(optional)_ List of dbt object types to include.
  - Default: `["models", "snapshots"]`
  - Options: `models`, `snapshots`, `macros`, `functions`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "no_hardcoded_refs"
    type: "code_no_hardcoded_refs"
    description: "SQL code must not contain hardcoded table references."
    # severity: "error"  (optional)
    # applies_to: ['models', 'snapshots']  (optional)
    # includes: ["models/staging/*"]
    # excludes: ["models/base/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "no_hardcoded_refs"
type = "code_no_hardcoded_refs"
description = "SQL code must not contain hardcoded table references."
# severity = "error"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["models/staging/*"]
# excludes = ["models/base/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "no_hardcoded_refs"
type = "code_no_hardcoded_refs"
description = "SQL code must not contain hardcoded table references."
# severity = "error"  # (optional)
# applies_to = ["models", "snapshots"]  # (optional)
# includes = ["models/staging/*"]
# excludes = ["models/base/*"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```sql
-- models/good_model.sql (PASS - uses ref())
SELECT a.id, b.name
FROM {{ ref('users') }} a
JOIN {{ ref('orders') }} b ON a.id = b.user_id

-- models/bad_model.sql (FAIL - hardcoded schema.table)
SELECT id, name
FROM analytics.orders
WHERE active = true

-- models/bad_join.sql (FAIL - hardcoded JOIN target)
SELECT a.id
FROM {{ ref('users') }} a
JOIN raw.customers b ON a.id = b.customer_id

-- models/bad_quoted.sql (FAIL - quoted identifiers)
SELECT id FROM "analytics"."orders"

-- models/bad_three_part.sql (FAIL - three-part reference)
SELECT id FROM db.analytics.orders

-- models/multiline.sql (FAIL - FROM on separate line)
SELECT id
FROM
  analytics.orders
WHERE 1=1

-- models/commented_ok.sql (PASS - hardcoded ref is in a comment)
-- FROM analytics.orders
SELECT id FROM {{ ref('orders') }}
```

</details>

</details>
