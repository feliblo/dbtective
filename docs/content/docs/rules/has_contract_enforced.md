---
title: has_contract_enforced
type: docs
prev: docs/rules
sidebar:
  open: true
---

### Rule: `has_contract_enforced`

<span class="rule-category-badge badge-manifest">Manifest Rule</span>

<details open>
<summary>has_contract_enforced details</summary>
<br>
This rule ensures that models have contracts enforced. Model contracts in dbt allow you to define explicit expectations for your data models, such as schema, data types, and constraints.  See the [dbt model contracts documentation](https://docs.getdbt.com/docs/mesh/govern/model-contracts) for more details.

---

**Configuration**

- **type**: Must be `has_contract_enforced`.
- **access_level**: *(optional)* Only check models with this access level. When not set, all models are checked.
  - Options: `public`, `protected`, `private`
- **applies_to**: *(optional)* List of dbt object types to check.
  - Default: `["models"]`
  - Options: `models`

{{< include-markdown "content/snippets/common_rule_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  # Enforce contracts on all models
  - name: "enforce_model_contracts"
    type: "has_contract_enforced"
    description: "All models must have contracts enforced."

  # Only enforce contracts on public models
  - name: "public_models_have_contracts"
    type: "has_contract_enforced"
    access_level: "public"
    description: "Public models must have contracts enforced."
```

{{< /tab >}}

{{< tab >}}

```toml
# Enforce contracts on all models
[[manifest_tests]]
name = "enforce_model_contracts"
type = "has_contract_enforced"
description = "All models must have contracts enforced."

# Only enforce contracts on public models
[[manifest_tests]]
name = "public_models_have_contracts"
type = "has_contract_enforced"
access_level = "public"
description = "Public models must have contracts enforced."
```

{{< /tab >}}

{{< tab >}}

```toml
# Enforce contracts on all models
[[tool.dbtective.manifest_tests]]
name = "enforce_model_contracts"
type = "has_contract_enforced"
description = "All models must have contracts enforced."

# Only enforce contracts on public models
[[tool.dbtective.manifest_tests]]
name = "public_models_have_contracts"
type = "has_contract_enforced"
access_level = "public"
description = "Public models must have contracts enforced."
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
