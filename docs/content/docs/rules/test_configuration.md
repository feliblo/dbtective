---
title: tests configuration (2)
type: docs
prev: docs/rules
sidebar:
  open: true
---

### Rule: `has_required_tests`

<details open>
<summary>has_required_tests details</summary>
<br>
This rule ensures that dbt objects (models, sources, etc.) have a configurable set of mandatory tests attached to them. Each entry in the <code>required_tests</code> list is checked independently — a violation is raised for each missing required test.

Each entry can be either:

- A **plain string** — matches exactly that test metadata name (e.g., `"not_null"`)
- An **object** with `name` and `allowed_names` — any of the `allowed_names` satisfies the requirement (e.g., treat `unique` and `dbt_utils.unique_combination_of_columns` as equivalent)

---

**Configuration**

- **type**: Must be `has_required_tests`.
- **applies_to**: _(optional)_ List of dbt object types to include.
  - Default: `["models", "sources"]`
  - Options: `models`, `sources`, `seeds`, `snapshots`
- **required_tests**: List of required test entries. Each entry is either a string or an object with `name` and `allowed_names`.

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "primary_key_integrity"
    type: "has_required_tests"
    description: "All models must have not_null and uniqueness tests"
    severity: "error"
    applies_to: ["models"]
    required_tests:
      - "not_null"
      - name: "uniqueness"
        allowed_names:
          - "unique"
          - "dbt_utils.unique_combination_of_columns"
          - "dbt_expectations.expect_compound_columns_to_be_unique"
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "primary_key_integrity"
type = "has_required_tests"
description = "All models must have not_null and uniqueness tests"
severity = "error"
applies_to = ["models"]

[[manifest_tests.required_tests]]
name = "not_null"

[[manifest_tests.required_tests]]
name = "uniqueness"
allowed_names = [
  "unique",
  "dbt_utils.unique_combination_of_columns",
  "dbt_expectations.expect_compound_columns_to_be_unique"
]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "primary_key_integrity"
type = "has_required_tests"
description = "All models must have not_null and uniqueness tests"
severity = "error"
applies_to = ["models"]

[[tool.dbtective.manifest_tests.required_tests]]
name = "not_null"

[[tool.dbtective.manifest_tests.required_tests]]
name = "uniqueness"
allowed_names = [
  "unique",
  "dbt_utils.unique_combination_of_columns",
  "dbt_expectations.expect_compound_columns_to_be_unique"
]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```yaml
models:
  - name: model_with_required_tests
    columns:
      - name: customer_id
        tests:
          - unique
          - not_null
    tests:
      - dbt_utils.unique_combination_of_columns:
          combination_of_columns:
            - customer_id
            - order_id
```

</details>
</details>

---

### Rule: `has_unique_test`

<details open>
<summary>has_unique_test details</summary>
<br>
This rule ensures that dbt objects (models, sources, etc.) have at least one uniqueness test attached to them. By default, it recognizes the standard <code>unique</code> test and <code>dbt_utils.unique_combination_of_columns</code> and <code>dbt_expectations.expect_compound_columns_to_be_unique</code>, but can be configured to accept custom uniqueness test names.

> **Note:** This is a special case of `has_required_tests` that checks for uniqueness tests, since that is the most common usecase. It does the same thing, but with pre-configured defaults, since uniqueness tests are so common. It is also kept for backwards compatibility.

---

**Configuration**

- **type**: Must be `has_unique_test`.
- **applies_to**: _(optional)_ List of dbt object types to include.
  - Default: `["models", "sources"]`
  - Options: `models`, `sources`, `seeds`, `snapshots`
- **allowed_test_names**: _(optional)_ List of test names that qualify as uniqueness tests.
  - Default: `["unique", "dbt_utils.unique_combination_of_columns", "dbt_expectations.expect_compound_columns_to_be_unique"]`
  - Accepts any custom test names (e.g., `["unique", "my_custom_unique_test"]`)

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "models_should_have_unique_test"
    type: "has_unique_test"
    description: "All models should have a unique test"
    severity: "error"
    applies_to: ["models"]
    # allowed_test_names: ["unique", "dbt_utils.unique_combination_of_columns"]  (optional)
    # includes: ["path/to/include/*"]  (optional)
    # excludes: ["path/to/exclude/*"]  (optional)

  - name: "sources_should_have_unique_test"
    type: "has_unique_test"
    description: "All sources must have uniqueness validation"
    severity: "warning"
    applies_to: ["sources"]
    allowed_test_names:
      - "unique"
      - "dbt_utils.unique_combination_of_columns"
      - "my_custom_unique_test"
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "models_should_have_unique_test"
type = "has_unique_test"
description = "All models should have a unique test"
severity = "error"
applies_to = ["models"]
# allowed_test_names = ["unique", "dbt_utils.unique_combination_of_columns"]  # (optional)
# includes = ["path/to/include/*"]  # (optional)
# excludes = ["path/to/exclude/*"]  # (optional)

[[manifest_tests]]
name = "sources_should_have_unique_test"
type = "has_unique_test"
description = "All sources must have uniqueness validation"
severity = "warning"
applies_to = ["sources"]
allowed_test_names = [
  "unique",
  "dbt_utils.unique_combination_of_columns",
  "my_custom_unique_test"
]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "models_should_have_unique_test"
type = "has_unique_test"
description = "All models should have a unique test"
severity = "error"
applies_to = ["models"]
# allowed_test_names = ["unique", "dbt_utils.unique_combination_of_columns"]  # (optional)
# includes = ["path/to/include/*"]  # (optional)
# excludes = ["path/to/exclude/*"]  # (optional)

[[tool.dbtective.manifest_tests]]
name = "sources_should_have_unique_test"
type = "has_unique_test"
description = "All sources must have uniqueness validation"
severity = "warning"
applies_to = ["sources"]
allowed_test_names = [
  "unique",
  "dbt_utils.unique_combination_of_columns",
  "my_custom_unique_test"
]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```yaml
models:
  - name: model_with_unique_tests
    tests:
      # dbt_utils built-in uniqueness test
      - dbt_utils.unique_combination_of_columns:
          combination_of_columns:
            - customer_id
            - order_id
    columns:
      - name: customer_id
        tests:
          - unique # dbt built-in uniqueness test
          - my_custom_unique_test # Custom uniqueness test
```

</details>
</details>
