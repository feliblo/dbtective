---
title: allowed_subfolders
type: docs
prev: docs/rules
sidebar:
  open: true
---


### Rule: `allowed_subfolders`

<br>
<details open>
<summary>allowed_subfolders details</summary>
<br>
This rule enforces that dbt objects are organized within specific allowed subfolders of a given path. This helps maintain a consistent project structure and ensures models are properly categorized (e.g., by data source or domain).

---

**Configuration**

- **type**: Must be `allowed_subfolders`.
- **allowed_subfolders**: *(required)* List of subfolder names that are allowed within the specified path.
- Both the `path_prefix` and `path_postfix` can be used to create this path `{path_prefix}/{resource_type}/{path_postfix}/`.
  - **path_prefix**: *(optional)* Path segment that appears before the resource type (e.g., `dbt` for paths like `dbt/models/...`).
  - **path_postfix**: *(optional)* Path segment that appears after the resource type (e.g., `bronze` for paths like `models/bronze/...`).
- **applies_to**: *(optional)* List of dbt object types to include.
  - Default: `["models"]`
  - Options: `models`, `seeds`, `snapshots`, `analyses`, `metrics`, `hook_nodes`, `sql_operations`, `saved_queries`, `sources`, `unit_tests`, `macros`, `exposures`, `semantic_models`, `functions`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "medallion_structure"
    type: "allowed_subfolders"
    description: "Models must follow medallion architecture (bronze/silver/gold)."
    allowed_subfolders:
      - "bronze"
      - "silver"
      - "gold"

  - name: "bronze_models_by_source"
    type: "allowed_subfolders"
    description: "Bronze models must be organized by source system."
    path_postfix: "bronze"
    allowed_subfolders:
      - "salesforce"
      - "postgres"
      - "snowflake"
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "medallion_structure"
type = "allowed_subfolders"
description = "Models must follow medallion architecture (bronze/silver/gold)."
applies_to = ["models"]
allowed_subfolders = ["bronze", "silver", "gold"]

[[manifest_tests]]
name = "bronze_models_by_source"
type = "allowed_subfolders"
description = "Bronze models must be organized by source system."
applies_to = ["models"]
path_postfix = "bronze"
allowed_subfolders = ["salesforce", "postgres", "snowflake"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "medallion_structure"
type = "allowed_subfolders"
description = "Models must follow medallion architecture (bronze/silver/gold)."
applies_to = ["models"]
allowed_subfolders = ["bronze", "silver", "gold"]

[[tool.dbtective.manifest_tests]]
name = "bronze_models_by_source"
type = "allowed_subfolders"
description = "Bronze models must be organized by source system."
applies_to = ["models"]
path_postfix = "bronze"
allowed_subfolders = ["salesforce", "postgres", "snowflake"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt structure</summary>

The following example shows the expected folder structure enforced by the two rules above:

- The `medallion_structure` rule ensures that models are organized into `bronze`, `silver`, and `gold` folders.
- The `bronze_models_by_source` rule ensures that within the `bronze` folder, models are further organized by source system (e.g., `salesforce`, `postgres`, `snowflake`).

This results in the following structure: <span style="color:#27ae60;font-weight:bold">✓ = compliant</span>, <span style="color:#e74c3c;font-weight:bold">✗ = non-compliant</span>

{{< filetree/container >}}
  {{< filetree/folder name="dbt_project" >}}
    {{< filetree/folder name="models" >}}
      {{< filetree/folder name="bronze" >}}
        {{< filetree/folder name="salesforce ✓" state="open" >}}
          {{< filetree/file name="stg_accounts.sql ✓" >}}
        {{< /filetree/folder >}}
        {{< filetree/folder name="postgres ✓" state="open" >}}
          {{< filetree/file name="stg_users.sql ✓" >}}
        {{< /filetree/folder >}}
        {{< filetree/folder name="other_source ✗" state="open" >}}
          {{< filetree/file name="stg_data.sql ✗ violates rule 2 (non-allowed subfolder)" >}}
        {{< /filetree/folder >}}
        {{< filetree/file name="stg_orders.sql ✗ violates rule 2 (root)" >}}
      {{< /filetree/folder >}}
      {{< filetree/folder name="silver" state="open" >}}
        {{< filetree/file name="dim_customers.sql ✓" >}}
      {{< /filetree/folder >}}
    {{< /filetree/folder >}}
  {{< /filetree/folder >}}
{{< /filetree/container >}}

</details>

<!-- Find the ✓ and ✗ symbols and color them accordingly -->
<style>
.hextra-filetree-folder span:has-text("✓"),
.hextra-filetree-folder .hx\:ltr\:ml-1:has-text("✓"),
span.hx\:ltr\:ml-1.hx\:rtl\:mr-1 {
  background: linear-gradient(to right, transparent 0%, transparent calc(100% - 1.2em), #27ae60 calc(100% - 1.2em), #27ae60 100%);
  background-clip: text;
  -webkit-background-clip: text;
}
</style>

<script>
document.addEventListener('DOMContentLoaded', function() {
  // Color all ✓ symbols green
  document.querySelectorAll('.hx\\:ltr\\:ml-1, .hx\\:rtl\\:mr-1').forEach(function(el) {
    if (el.textContent.includes('✓')) {
      el.innerHTML = el.innerHTML.replace(/✓/g, '<span style="color:#27ae60;font-weight:bold">✓</span>');
    }
    if (el.textContent.includes('✗')) {
      el.innerHTML = el.innerHTML.replace(/✗/g, '<span style="color:#e74c3c;font-weight:bold">✗</span>');
    }
  });
});
</script>
