---
title: columns (4)
type: docs
prev: docs/rules
sidebar:
  open: true
---

### Rule: `columns_name_convention`

For object naming conventions, see the [`name_convention`](../naming_conventions#name_convention) rule.

<span class="rule-category-badge badge-catalog">Catalog Rule</span> {{< include-markdown "content/snippets/catalog_info.md" >}}

<br>
<details closed>
<summary>columns_name_convention details</summary>
<br>
This rule ensures that column names follow naming conventions based on a specified pattern.

---

**Configuration**

- **type**: Must be `columns_name_convention`.
- **applies_to**: *(optional)* List of dbt object types to include.
  - Default: `["models", "seeds", "snapshots"]`
  - Options: `models`, `seeds`, `snapshots`, `sources`
- **pattern**: The naming convention pattern to enforce. Can be one of the following presets or a custom regex pattern.
  - Presets:
    - `snake_case`: lowercase letters, numbers, and underscores (e.g., `user_id`, `created_at`)
    - `kebab-case`: lowercase letters, numbers, and hyphens (e.g., `user-id`, `created-at`)
    - `camelCase`: starts with a lowercase letter, followed by uppercase letters for new words (e.g., `userId`, `createdAt`)
    - `PascalCase`: starts with an uppercase letter, followed by uppercase letters for new words (e.g., `UserId`, `CreatedAt`)
  - Custom Regex: Any valid regex pattern to match against column names.
- **data_types**: *(optional)* List of SQL data types to filter columns by. Only columns with these data types will be checked included in the naming convention rule. If not specified, all columns are included.
  - *Default*: All data types
  - *Example*: If you want all datetime columns to end with 'dt', you can set `data_types: ['date', 'date_time', 'timestamp', 'timestamptz']` with pattern `.*_dt$`
  - *Available types*: `integer`, `big_int`, `small_int`, `tiny_int`, `decimal`, `numeric`, `float`, `double`, `real`, `string`, `text`, `varchar`, `char`, `date`, `date_time`, `time`, `timestamp`, `timestamptz`, `boolean`, `json`, `jsonb`, `array`, `object`, `variant`, `binary`, `varbinary`, `uuid`, `interval`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
catalog_tests:
  - name: "columns_snake_case"
    type: "columns_name_convention"
    description: "All column names must be snake_case."
    pattern: "snake_case"
    # severity: "warning"  (optional)
    # applies_to: ['models', 'sources']  (optional)
    # includes: ["path/to/include/*"]  (optional)
    # excludes: ["path/to/exclude/*"]  (optional)
```

{{< /tab >}}

{{< tab >}}

```toml
[[catalog_tests]]
name = "columns_snake_case"
type = "columns_name_convention"
description = "All column names must be snake_case."
pattern = "snake_case"
# severity = "warning"  # (optional)
# applies_to = ["models", "sources"]  # (optional)
# includes = ["path/to/include/*"]  # (optional)
# excludes = ["path/to/exclude/*"]  # (optional)
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.catalog_tests]]
name = "columns_snake_case"
type = "columns_name_convention"
description = "All column names must be snake_case."
pattern = "snake_case"
# severity = "warning"  # (optional)
# applies_to = ["models", "sources"]  # (optional)
# includes = ["path/to/include/*"]  # (optional)
# excludes = ["path/to/exclude/*"]  # (optional)
```

{{< /tab >}}

{{< /tabs >}}

**Example with Custom Regex**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
catalog_tests:
  - name: "columns_snake_case"
    type: "columns_name_convention"
    description: "All column must be snake_case."
    pattern: "snake_case"
```

{{< /tab >}}

{{< tab >}}

```toml
[[catalog_tests]]
name = "columns_snake_case"
type = "columns_name_convention"
description = "All column names must be snake_case."
pattern = "snake_case"
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.catalog_tests]]
name = "columns_snake_case"
type = "columns_name_convention"
description = "All column names must be snake_case."
pattern = "snake_case"
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```sql
-- Example model SQL
SELECT
    snake_case,           -- PASS: snake_case
    camelCase,            -- PASS: camelCase
    PascalCase            -- PASS: PascalCase
FROM users
```

</details>
</details>

<hr style="border: 2px solid #444; margin: 2em 0;">

### Rule: `columns_all_documented`

<span class="rule-category-badge badge-catalog">Catalog Rule</span> {{< include-markdown "content/snippets/catalog_info.md" >}}

<details closed>
<summary>columns_all_documented details</summary>
<br>
This rule ensures that every dbt object  (model, seed, source, macro, etc.) documented their columns (e.g. mentioned them in a `.yaml` file).

---

**Configuration**

- **type**: Must be `columns_all_documented`.
- **applies_to**: *(optional)* List of dbt object types to include.
  - Default: `["models", "seeds", "snapshots", "sources", "semantic_models"]`
  - Options: `models`, `seeds`, `snapshots`, `sources`, `macros`,`semantic_models`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
catalog_tests:
  - name: "all_columns_should_be_documented"
    type: "columns_all_documented"
    description: "Everything must have a description."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds']  (optional)
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[catalog_tests]]
name = "all_columns_should_be_documented"
type = "columns_all_documented"
description = "Everything must have a description."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.catalog_tests]]
name = "all_columns_should_be_documented"
type = "columns_all_documented"
description = "Everything must have a description."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```yaml
models:
  - name: model_without_columns_documented
    columns:
      - column_1
      - column_2
  # Example if the model has 2 columns
  - name: model_with_missing_documentation_for_column_2
    columns:
      - column_1
  - name: model_without_columns_documented
```

</details>

</details>

<hr style="border: 2px solid #444; margin: 2em 0;">

### Rule: `columns_have_description`

<span class="rule-category-badge badge-catalog">Catalog Rule</span> {{< include-markdown "content/snippets/catalog_info.md" >}}

<details closed>
<summary>columns_have_description details</summary>
<br>
This rule ensures that every documented column has a non-empty description. Unlike `columns_all_documented` which checks that columns are mentioned in YAML files, this rule verifies that those columns actually have meaningful descriptions.

---

**Configuration**

- **type**: Must be `columns_have_description`.
- **applies_to**: *(optional)* List of dbt object types to include.
  - Default: `["models", "seeds", "snapshots", "sources", "semantic_models"]`
  - Options: `models`, `seeds`, `snapshots`, `sources`, `macros`,`semantic_models`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
catalog_tests:
  - name: "all_columns_must_have_descriptions"
    type: "columns_have_description"
    description: "All documented columns must have non-empty descriptions."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds']  (optional)
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[catalog_tests]]
name = "all_columns_must_have_descriptions"
type = "columns_have_description"
description = "All documented columns must have non-empty descriptions."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.catalog_tests]]
name = "all_columns_must_have_descriptions"
type = "columns_have_description"
description = "All documented columns must have non-empty descriptions."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```yaml
models:
  - name: customers
    columns:
      - name: id
        description: "Customer ID"  # PASS: has description
      - name: name
        description: ""  # FAIL: empty description
      - name: email
        # FAIL: no description field
```

</details>

</details>

<hr style="border: 2px solid #444; margin: 2em 0;">

### Rule: `columns_canonical_name`

<span class="rule-category-badge badge-catalog">Catalog Rule</span> {{< include-markdown "content/snippets/catalog_info.md" >}}

<details closed>
<summary>columns_canonical_name details</summary>
<br>

Identifies columns that match specified "invalid" patterns and flags them as violations, suggesting they should be renamed to the canonical name. You can also define exceptions for columns that should be allowed even when matched.
Can be both regex or strings.

---

**Configuration**

- **type**: Must be `columns_canonical_name`.
- **canonical**: The preferred/canonical column name (e.g., `zip_code`).
- **invalid_names**: List of patterns that should be flagged as violations. Each pattern can be:
  - *Strings*: An exact string match (e.g., `postal_code`)
  - *Regex*: A pattern starting with `^`, ending with `$`, or containing `.*` or `.+` (e.g., `^zip.*`)
- **exceptions**: *(optional)* List of patterns to exclude from violations. Columns matching these patterns will not be flagged even if they match `invalid_names`. Uses the same literal/regex format as `invalid_names`.
- **applies_to**: *(optional)* List of dbt object types to include.
  - Default: `["models", "seeds", "snapshots"]`
  - Options: `models`, `seeds`, `snapshots`, `sources`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
catalog_tests:
  - name: "canonical_zip_code"
    type: "columns_canonical_name"
    description: "All zip-related columns should be named 'zip_code'."
    canonical: "zip_code"
    invalid_names:
      - "postal_code"     # literal match
      - "^zip"            # regex: matches zipcode, zip_cd, etc.
    # exceptions:
    #   - "zip_code_legacy"  # allow this specific column
    # severity: "warning"  (optional)
    # applies_to: ['models', 'sources']  (optional)
    # includes: ["path/to/include/*"]  (optional)
    # excludes: ["path/to/exclude/*"]  (optional)
```

{{< /tab >}}

{{< tab >}}

```toml
[[catalog_tests]]
name = "canonical_zip_code"
type = "columns_canonical_name"
description = "All zip-related columns should be named 'zip_code'."
canonical = "zip_code"
invalid_names = ["postal_code", "^zip"]
# exceptions = ["zip_code_legacy"]  # (optional)
# severity = "warning"  # (optional)
# applies_to = ["models", "sources"]  # (optional)
# includes = ["path/to/include/*"]  # (optional)
# excludes = ["path/to/exclude/*"]  # (optional)
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.catalog_tests]]
name = "canonical_zip_code"
type = "columns_canonical_name"
description = "All zip-related columns should be named 'zip_code'."
canonical = "zip_code"
invalid_names = ["postal_code", "^zip"]
# exceptions = ["zip_code_legacy"]  # (optional)
# severity = "warning"  # (optional)
# applies_to = ["models", "sources"]  # (optional)
# includes = ["path/to/include/*"]  # (optional)
# excludes = ["path/to/exclude/*"]  # (optional)
```

{{< /tab >}}

{{< /tabs >}}


<details closed>
<summary>Relevant dbt code</summary>

```sql
SELECT
    zip_code,            -- PASS: canonical name
    postal_code,         -- FAIL: matches invalid_names literal
    zipcode,             -- FAIL: matches invalid_names regex ^zip
    zip_code_legacy,     -- PASS: matches exception
    other_column         -- PASS: not in invalid_names
FROM source_table
```

</details>

</details>
