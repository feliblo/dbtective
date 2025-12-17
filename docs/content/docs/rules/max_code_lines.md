---
title: max_code_lines
type: docs
prev: docs/rules
sidebar:
  open: true
---


### Rule: `max_code_lines`

<br>
<details open>
<summary>max_code_lines details</summary>
<br>
This rule enforces a maximum line count for dbt code objects, helping to maintain code readability and encourage modular design. Objects with empty code will also be flagged by this rule.

---

**Configuration**

- **type**: Must be `max_code_lines`.
- **max_lines**: *(optional)* The maximum number of lines allowed for the code. Defaults to `150`.
- **applies_to**: *(optional)* List of dbt object types to include.
  - Default: `["models", "snapshots", "macros"]`
  - Options: `models`, `snapshots`, `macros`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "models_max_100_lines"
    type: "max_code_lines"
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
type = "max_code_lines"
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
type = "max_code_lines"
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
