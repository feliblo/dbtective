---
title: property_file_colocation
type: docs
prev: docs/rules
sidebar:
  open: true
---


### Rule: `property_file_colocation`

<br>
<details open>
<summary>property_file_colocation details</summary>
<br>
This rule checks that property (YAML) files are colocated with the object's primary file (SQL, CSV, etc.). It helps enforce a consistent project structure where schema definitions live alongside the code they describe.

Two modes are available:

- **`same_directory`** _(default)_ — the property file must be in the same directory as the primary file.
- **`relative_subdirectory`** — the property file may live in one of the specified `allowed_subdirectories` beneath the primary file's directory.

---

**Configuration**

- **type**: Must be `property_file_colocation`.
- **mode**: *(optional)* Colocation strategy.
  - Default: `same_directory`
  - Options: `same_directory`, `relative_subdirectory`
- **allowed_subdirectories**: *(optional)* List of subdirectory names allowed when using `relative_subdirectory` mode. If empty or omitted, falls back to `same_directory` behavior.
- **applies_to**: *(optional)* List of dbt object types to include.
  - Default: `["models", "seeds", "snapshots", "sources", "macros", "exposures"]`
  - Options: `models`, `seeds`, `snapshots`, `sources`, `macros`, `exposures`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  # Ensure YAML files are in the same directory as their SQL files
  - name: "yaml_colocation"
    type: "property_file_colocation"
    description: "Property files must be colocated with their SQL files"
    severity: "warning"
    applies_to: ["models"]

  # Allow YAML files in a 'properties' subdirectory
  - name: "yaml_in_properties_folder"
    type: "property_file_colocation"
    description: "Property files must be in a 'properties' subdirectory"
    mode: "relative_subdirectory"
    allowed_subdirectories:
      - "properties"
      - "schema"
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "yaml_colocation"
type = "property_file_colocation"
description = "Property files must be colocated with their SQL files"
severity = "warning"
applies_to = ["models"]

[[manifest_tests]]
name = "yaml_in_properties_folder"
type = "property_file_colocation"
description = "Property files must be in a 'properties' subdirectory"
mode = "relative_subdirectory"
allowed_subdirectories = ["properties", "schema"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "yaml_colocation"
type = "property_file_colocation"
description = "Property files must be colocated with their SQL files"
severity = "warning"
applies_to = ["models"]

[[tool.dbtective.manifest_tests]]
name = "yaml_in_properties_folder"
type = "property_file_colocation"
description = "Property files must be in a 'properties' subdirectory"
mode = "relative_subdirectory"
allowed_subdirectories = ["properties", "schema"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt structure</summary>

**Same directory mode** — YAML file alongside SQL:

{{< filetree/container >}}
  {{< filetree/folder name="models" >}}
    {{< filetree/folder name="staging" state="open" >}}
      {{< filetree/file name="stg_orders.sql" >}}
      {{< filetree/file name="_stg_orders.yml ✓ same directory" >}}
    {{< /filetree/folder >}}
  {{< /filetree/folder >}}
{{< /filetree/container >}}

**Relative subdirectory mode** — YAML in an allowed subdirectory:

{{< filetree/container >}}
  {{< filetree/folder name="models" >}}
    {{< filetree/folder name="staging" state="open" >}}
      {{< filetree/file name="stg_orders.sql" >}}
      {{< filetree/folder name="properties" state="open" >}}
        {{< filetree/file name="_stg_orders.yml ✓ allowed subdirectory" >}}
      {{< /filetree/folder >}}
    {{< /filetree/folder >}}
  {{< /filetree/folder >}}
{{< /filetree/container >}}

**Violation** — YAML in a different directory:

{{< filetree/container >}}
  {{< filetree/folder name="models" >}}
    {{< filetree/folder name="staging" state="open" >}}
      {{< filetree/file name="stg_orders.sql" >}}
    {{< /filetree/folder >}}
    {{< filetree/file name="schema.yml ✗ not colocated" >}}
  {{< /filetree/folder >}}
{{< /filetree/container >}}

</details>

<script>
document.addEventListener('DOMContentLoaded', function() {
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
</details>
