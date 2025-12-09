---
title: Configuration
description: Learn how to configure dbtective for your dbt project
weight: 2
---


dbtective supports multiple configuration formats to fit your project needs. You can use YAML, TOML, or integrate with your existing `pyproject.toml` file.

**Supported formats:**

- `dbtective.yml` or `dbtective.yaml` - YAML format (recommended for dbt projects)
- `dbtective.toml` - TOML format
- `pyproject.toml` - For Python projects, config goes under `[tool.dbtective]`

## Complete Example

Since everyone hates reading documentation they don't need. Let's start with a complete example:

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  # Ensure all models and sources have descriptions
  - name: "models_must_have_description"
    type: "has_description"
    severity: "error"
    applies_to: ["models", "sources"]
    description: "All models and sources must have a description."

  # Enforce snake_case naming for all objects
  - name: "naming_convention"
    type: "name_convention"
    description: "All objects must follow the snake_case naming convention."
    pattern: "snake_case"
    severity: "error"

  # Warn if staging models lack descriptions
  - name: "staging_description_warning"
    type: "has_description"
    severity: "warning"
    applies_to: ["models"]
    includes:
      - "models/staging/**"
    description: "Staging models should have descriptions."

  # Ensure mart models have descriptions (excluding deprecated)
  - name: "marts_must_have_description"
    type: "has_description"
    severity: "error"
    applies_to: ["models"]
    includes:
      - "models/marts/**"
    excludes:
      - "models/marts/deprecated/**"
    description: "All mart models must have descriptions."
```

{{< /tab >}}

{{< tab >}}

```toml
# Ensure all models and sources have descriptions
[[manifest_tests]]
name = "models_must_have_description"
type = "has_description"
severity = "error"
applies_to = ["models", "sources"]
description = "All models and sources must have a description."

# Enforce snake_case naming for all objects
[[manifest_tests]]
name = "naming_convention"
type = "name_convention"
description = "All objects must follow the snake_case naming convention."
pattern = "snake_case"
severity = "error"

# Warn if staging models lack descriptions
[[manifest_tests]]
name = "staging_description_warning"
type = "has_description"
severity = "warning"
applies_to = ["models"]
includes = ["models/staging/**"]
description = "Staging models should have descriptions."

# Ensure mart models have descriptions (excluding deprecated)
[[manifest_tests]]
name = "marts_must_have_description"
type = "has_description"
severity = "error"
applies_to = ["models"]
includes = ["models/marts/**"]
excludes = ["models/marts/deprecated/**"]
description = "All mart models must have descriptions."
```

{{< /tab >}}

{{< tab >}}

```toml
[tool.dbtective]

# Ensure all models and sources have descriptions
[[tool.dbtective.manifest_tests]]
name = "models_must_have_description"
type = "has_description"
severity = "error"
applies_to = ["models", "sources"]
description = "All models and sources must have a description."

# Enforce snake_case naming for all objects
[[tool.dbtective.manifest_tests]]
name = "naming_convention"
type = "name_convention"
description = "All objects must follow the snake_case naming convention."
pattern = "snake_case"
severity = "error"

# Warn if staging models lack descriptions
[[tool.dbtective.manifest_tests]]
name = "staging_description_warning"
type = "has_description"
severity = "warning"
applies_to = ["models"]
includes = ["models/staging/**"]
description = "Staging models should have descriptions."

# Ensure mart models have descriptions (excluding deprecated)
[[tool.dbtective.manifest_tests]]
name = "marts_must_have_description"
type = "has_description"
severity = "error"
applies_to = ["models"]
includes = ["models/marts/**"]
excludes = ["models/marts/deprecated/**"]
description = "All mart models must have descriptions."
```

{{< /tab >}}

{{< /tabs >}}

## Config File Detection

dbtective automatically detects and loads your configuration file. If you have multiple config files in your project directory, dbtective will use the following priority order:

1. **`dbtective.yml` or `dbtective.yaml`** (highest priority)
2. **`dbtective.toml`**
3. **`pyproject.toml`** (lowest priority)

**What happens with multiple configs:**

- If multiple config files are found, dbtective will use the highest priority one
- A warning will be displayed showing which files were found and which one was chosen
- You can override auto-detection by explicitly specifying a config file with `--config-file`

## Rule Configuration

| Property | Required | Description |
|----------|----------|-------------|
| `type` | **Yes** | The type of check to perform see [individual check documentation](/docs/checks) |
| `name` | No | Custom name to show for the rule. Defaults to the rule type if not specified |
| `severity` | No | `error` (fails check, default) or `warning` (reports but doesn't fail) |
| `description` | No | Human-readable description of what the rule checks |
| `applies_to` | No | List of dbt object types to check (e.g., `["models", "sources"]`). See [individual check documentation](/docs/checks) for valid targets |
| `includes` | No | File path patterns to include. Supports glob syntax (e.g., `models/staging/**`) |
| `excludes` | No | File path patterns to exclude. Supports glob syntax (e.g., `models/deprecated/**`) |
| `custom_fiels` | Sometimes | Custom fields for checks. See [individual check documentation](/docs/checks) |
