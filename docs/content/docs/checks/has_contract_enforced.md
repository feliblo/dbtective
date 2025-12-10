---
title: Has Contract Enforced
type: docs
prev: docs/checks
sidebar:
  open: true
---

### Check: `has_contract_enforced`

<span class="check-category-badge badge-manifest">Manifest Check</span>

<details open>
<summary>has_contract_enforced details</summary>
<br>
This check ensures that models have contracts enforced. Model contracts in dbt allow you to define explicit expectations for your data models, such as schema, data types, and constraints.  See the [dbt model contracts documentation](https://docs.getdbt.com/docs/mesh/govern/model-contracts) for more details.

---

**Configuration**

- **type**: Must be `has_contract_enforced`.
- **applies_to**: *(optional)* List of dbt object types to check.
  - Default: `["models"]`
  - Options: `models`

{{< include-markdown "content/snippets/common_check_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "enforce_model_contracts"
    type: "has_contract_enforced"
    description: "All models must have contracts enforced."
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "enforce_model_contracts"
type = "has_contract_enforced"
description = "All models must have contracts enforced."
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "enforce_model_contracts"
type = "has_contract_enforced"
description = "All models must have contracts enforced."
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```yml
models:
  your_model_name:
    config:
        contract:
            enforced: true
```

</details>
