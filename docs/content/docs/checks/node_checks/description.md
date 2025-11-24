---
title: Description
type: docs
prev: docs/node_checks 
sidebar:
  open: true
---


### Check: `has_description`
<br>
<details open>
<summary>has_description details</summary>
<br>
This check ensures that every dbt node (model, seed, source, macro, etc.) has a description provided in the configuration.



---


**Configuration**
<details>
<summary>Common Check Config</summary>

- **name**: Human-readable name of the check
- **severity**: `"error"` (fail) or `"warning"` (warn only).  
  -  *(optional, defaults to `"error"` if not specified)*
- **description**: Human-readable explanation of the rule.
- **applies_to**: *(optional)* List of node types to check.  
  - If omitted, defaults to `["models", "seeds", "sources", "macros"]`.
  - Options: `TODO`

</details>


- **type**: Must be `has_description`.



**Example Config**

```yaml
manifest_tests:
  - name: "models_must_have_description"
    type: "has_description"
    description: "All nodes must have a description."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds'] (optional)
```

<details closed>
<summary>Relevant dbt code</summary>

```yaml
models:
  - name: model_with_description
    description: This is a model with a description
  - name: model_without_description
```
</details>


</details>
