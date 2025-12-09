---

title: Object Tests
type: docs
prev: docs/checks
sidebar:
  open: true
---

### Check: `has_unique_test`

<details open>
<summary>has_unique_test details</summary>
<br>
This check ensures that dbt objects (models, sources, etc.) have at least one uniqueness test attached to them. By default, it recognizes the standard <code>unique</code> test and <code>dbt_utils.unique_combination_of_columns</code> and <code>dbt_expectations.expect_compound_columns_to_be_unique</code>, but can be configured to accept custom uniqueness test names.

---

**Configuration**

- **type**: Must be `has_unique_test`.
- **applies_to**: *(optional)* List of dbt object types to check.
  - Default: `["models", "sources"]`
  - Options: `models`, `sources`, `seeds`, `snapshots`
- **allowed_test_names**: *(optional)* List of test names that qualify as uniqueness tests.
  - Default: `["unique", "dbt_utils.unique_combination_of_columns", "dbt_expectations.expect_compound_columns_to_be_unique"]`
  - Accepts any custom test names (e.g., `["unique", "my_custom_unique_test"]`)

{{< include-markdown "content/snippets/common_check_config.md" >}}

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
