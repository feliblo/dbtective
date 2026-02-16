---
title: has_forbidden_code
type: docs
prev: docs/rules
sidebar:
  open: true
---

### Rule: `has_forbidden_code`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>has_forbidden_code details</summary>
<br>
This rule checks if code contains forbidden patterns. Use it to enforce coding standards by flagging undesired patterns such as `SELECT *` statements, hardcoded references, or any other string patterns that should not appear in your dbt code.

---

**Configuration**

- **type**: Must be `has_forbidden_code`.
- **forbidden_patterns**: List of string patterns that are not allowed in the code. Each pattern is matched as a substring.
- **case_sensitive**: _(optional)_ Whether pattern matching should be case-sensitive. Defaults to `false` (case-insensitive).
- **applies_to**: _(optional)_ List of dbt object types to check.
  - Default: `["models", "snapshots", "macros"]`
  - Options: `models`, `snapshots`, `macros`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  # Forbid SELECT * in models (case-insensitive by default)
  - name: "no_select_star"
    type: "has_forbidden_code"
    forbidden_patterns: ["SELECT *"]
    description: "Models should not use SELECT *."
    # case_sensitive: false  (optional, default)
    # severity: "warning"  (optional)
    # applies_to: ['models', 'snapshots'] (optional)
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]

  # Case-sensitive match for exact patterns
  - name: "no_hardcoded_schema"
    type: "has_forbidden_code"
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
type = "has_forbidden_code"
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
type = "has_forbidden_code"
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
type = "has_forbidden_code"
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
type = "has_forbidden_code"
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
