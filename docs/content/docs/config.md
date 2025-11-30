---
title: Configuration
description: Learn how to configure dbtective for your dbt project
weight: 2
---

# Configuration

## Complete Example

dbtective uses a YAML configuration file (`dbtective.yml`) to define the rules and checks that will be applied to your dbt project. This file should be placed in the root of your dbt project.

Since everyone hates reading documentation they don't need. Let's start with a complete example of a `dbtective.yml` configuration file:

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


## Rule Configuration

Each rule in the `manifest_tests` array can have the following properties:

### Required Properties

#### `type`
**Required** - Specifies the type of check to perform.

Available check types:
- `has_description` - Ensures objects have descriptions
- `name_convention` - Enforces naming conventions

### Optional Properties

#### `name`
**Optional** - A custom name for the rule. If not specified, the rule type will be used as the name.

```yaml
manifest_tests:
  - name: "models_must_have_description"
    type: "has_description"
```

#### `severity`
**Optional** - The severity level when the rule fails. Defaults to `error`.

Available values:
- `error` - Causes dbtective to exit with code 1 (default)
- `warning` - Reports the issue but doesn't fail the check

```yaml
manifest_tests:
  - type: "has_description"
    severity: "warning"
```

#### `description`
**Optional** - A human-readable description of what the rule checks.

```yaml
manifest_tests:
  - type: "has_description"
    description: "All models and sources must have a description."
```

#### `applies_to`
**Optional** - Specifies which dbt object types the rule should apply to. If not specified, default targets for each rule type will be used.

Available targets:
- `models` - dbt models
- `seeds` - dbt seeds
- `sources` - dbt sources
- `tests` - dbt tests
- `snapshots` - dbt snapshots
- `analyses` - dbt analyses
- `macros` - dbt macros
- `exposures` - dbt exposures
- `metrics` - dbt metrics
- `semantic_models` - dbt semantic models
- `saved_queries` - dbt saved queries
- `hook_nodes` - dbt hook nodes
- `sql_operations` - dbt SQL operations

```yaml
manifest_tests:
  - type: "has_description"
    applies_to: ["models", "sources"]
```

**Note:** Not all check types support all targets. Each rule type has specific valid targets. dbtective will validate your configuration and report an error if you use an invalid target for a rule type.

#### `includes`
**Optional** - A list of file path patterns to include. Only objects matching these patterns will be checked. Patterns can use glob syntax.

```yaml
manifest_tests:
  - type: "has_description"
    includes:
      - "models/staging/**"
      - "models/marts/**"
```

#### `excludes`
**Optional** - A list of file path patterns to exclude. Objects matching these patterns will be skipped. Patterns can use glob syntax.

```yaml
manifest_tests:
  - type: "has_description"
    excludes:
      - "models/staging/deprecated/**"
      - "models/temp/**"
```
