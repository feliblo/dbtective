---
title: CLI Reference
description: Command-line interface reference for dbtective
weight: 3
---

## Installation

See the [README](https://github.com/feliblo/dbtective#installation) for installation instructions.

## Global Options

| Option | Description |
|--------|-------------|
| `--verbose`, `-v` | Enable verbose logging output |
| `--help`, `-h` | Display help information |
| `--version`, `-V` | Display version |

## Commands

### `run`

Run dbtective analysis on your dbt project.

**Usage:** `dbtective run [OPTIONS]`

**Important:**

- Before running manifest-based rules, run `dbt compile`, `dbt build`, `dbt run` or any of the [documented commands](https://docs.getdbt.com/reference/artifacts/manifest-json) to ensure `manifest.json` is up to date.
- Before running catalog-based rules, run `dbt docs generate` to ensure `catalog.json` is available.

#### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--entry-point <PATH>` | | `.` | Path to dbt project root |
| `--config-file <PATH>` | `-c` | Auto-detected | Path to dbtective configuration from the entry-point (overrides auto-detection) |
| `--manifest-file <PATH>` | `-m` | `target/manifest.json` | Path to dbt manifest.json |
| `--catalog-file <PATH>` | `-g` | `target/catalog.json` | Path to dbt catalog.json |
| `--only-manifest` | | `true` | Run only manifest rules |
| `--disable-hyperlinks` | | `false` | Disable file hyperlinks in the output |
| `--hide-warnings` | | `false` | Hide warnings from output (only show errors) |

#### Config File Auto-Detection

By default, dbtective automatically searches for configuration files in the following priority order:

1. `dbtective.yml` or `dbtective.yaml` (highest priority)
2. `dbtective.toml`
3. `pyproject.toml` (lowest priority)

If multiple config files exist, dbtective will use the highest priority one and display a warning. You can override this behavior by explicitly specifying `--config-file`.

#### Examples

```bash
# Run with defaults (auto-detects config, uses target/manifest.json)
dbtective run

# Run with a specific config file
dbtective run --config-file ./configs/dbtective.toml

# Run with verbose output
dbtective run --verbose

# Run on a specific dbt project
dbtective run --entry-point ./dbt_project

# Run only manifest rules
dbtective run --only-manifest

# Disable hyperlinks in output table
dbtective run --disable-hyperlinks

# Hide warnings, only show errors (useful for CI)
dbtective run --hide-warnings
```

### `init`

Generate a new dbtective configuration file by answering a few simple questions.

**Usage:** `dbtective init [OPTIONS]`

When you run `dbtective init` without flags it starts an interactive questionnaire. You will be asked:

1. **Config format** — where to write the output: a standalone `dbtective.yml`/`dbtective.toml`, or a `[tool.dbtective]` section inside an existing `pyproject.toml`.
2. **Naming convention** — the pattern your project follows (`snake_case`, `kebab-case`, `camelCase`, or `PascalCase`). This is applied to both model and column naming rules.
3. **Data model structure** — None, Common (`staging`/`marts`/`intermediate`), or Medallion (`bronze`/`silver`/`gold`). Choosing one automatically adds an `allowed_subfolders` rule with the matching folder list.
4. **Strictness level** — Basic, Standard, or Strict. Each level pre-selects a sensible set of rules; you can adjust them in the next step.
5. **Which rules to enable** — two multi-select prompts let you add or remove individual manifest and catalog rules on top of what the strictness level chose.

The generated config file is ready to use. You can edit it afterwards to fine-tune any rule options.

#### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--location <PATH>` | `-l` | `.` | Directory where the config file will be created |
| `--format <FORMAT>` | `-f` | `yml` | Config file format: `yml`, `yaml`, `toml`, or `pyproject`. Skips the format question when provided. |

#### Examples

```bash
# Run the interactive questionnaire (default)
dbtective init

# Skip the format question — write dbtective.toml directly
dbtective init --format toml

# Add [tool.dbtective] to an existing pyproject.toml
dbtective init --format pyproject

# Generate config in a specific directory
dbtective init --location ./my_dbt_project
```

## Getting Help

- Command help: `dbtective --help` or `dbtective run --help`
- Documentation: [https://feliblo.github.io/dbtective/](https://feliblo.github.io/dbtective/)
- Issues: [https://github.com/feliblo/dbtective/issues](https://github.com/feliblo/dbtective/issues)
