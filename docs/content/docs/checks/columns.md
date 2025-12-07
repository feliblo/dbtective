---
title: Columns
type: docs
prev: docs/checks
sidebar:
  open: true
---


### Check: `columns_all_documented`

<span class="check-category-badge badge-catalog">Catalog Check</span> {{< include-markdown "content/snippets/catalog_info.md" >}}

<details closed>
<summary>columns_all_documented details</summary>
<br>
This check ensures that every dbt object  (model, seed, source, macro, etc.) documented their columns (e.g. mentioned them in a `.yaml` file).

---

**Configuration**

- **type**: Must be `columns_all_documented`.
- **applies_to**: *(optional)* List of dbt object types to check.
  - Default: `["models", "seeds", "snapshots", "sources", "semantic_models"]`
  - Options: `models`, `seeds`, `snapshots`, `sources`, `macros`,`semantic_models`

{{< include-markdown "content/snippets/common_check_config.md" >}}

**Example Config**

```yaml
manifest_tests:
  - name: "all_columns_should_be_documented"
    type: "columns_all_documented"
    description: "Everything must have a description."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds'] (optional
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]

```

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

### Check: `columns_have_description`

<span class="check-category-badge badge-catalog">Catalog Check</span> {{< include-markdown "content/snippets/catalog_info.md" >}}

<details closed>
<summary>columns_have_description details</summary>
<br>
This check ensures that every documented column has a non-empty description. Unlike `columns_all_documented` which checks that columns are mentioned in YAML files, this check verifies that those columns actually have meaningful descriptions.

---

**Configuration**

- **type**: Must be `columns_have_description`.
- **applies_to**: *(optional)* List of dbt object types to check.
  - Default: `["models", "seeds", "snapshots", "sources", "semantic_models"]`
  - Options: `models`, `seeds`, `snapshots`, `sources`, `macros`,`semantic_models`

{{< include-markdown "content/snippets/common_check_config.md" >}}

**Example Config**

```yaml
catalog_tests:
  - name: "all_columns_must_have_descriptions"
    type: "columns_have_description"
    description: "All documented columns must have non-empty descriptions."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds'] (optional)
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]

```

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
