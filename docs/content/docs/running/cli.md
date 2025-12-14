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

- Before running manifest-based checks, run `dbt compile`, `dbt build`, `dbt run` or any of the [documented commands](https://docs.getdbt.com/reference/artifacts/manifest-json) to ensure `manifest.json` is up to date.
- Before running catalog-based checks, run `dbt docs generate` to ensure `catalog.json` is available.

#### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--entry-point <PATH>` | | `.` | Path to dbt project root |
| `--config-file <PATH>` | `-c` | Auto-detected | Path to dbtective configuration from the entry-point (overrides auto-detection) |
| `--manifest-file <PATH>` | `-m` | `target/manifest.json` | Path to dbt manifest.json |
| `--catalog-file <PATH>` | `-g` | `target/catalog.json` | Path to dbt catalog.json |
| `--only-manifest` | | `true` | Run only manifest checks |
| `--disable-hyperlinks` | | `false` | Disable file hyperlinks in the output |

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

# Run only manifest checks
dbtective run --only-manifest

# Disable hyperlinks in output table
dbtective run --disable-hyperlinks
```

### `init`

Initialize a new dbtective configuration file in your dbt project.

**Usage:** `dbtective init [OPTIONS]`

#### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--location <PATH>` | `-l` | `.` | Directory where the config file will be created |
| `--format <FORMAT>` | `-f` | `yml` | Config file format: `yml`, `yaml`, `toml`, or `pyproject` |

#### Examples

```bash
# Create dbtective.yml in current directory (default)
dbtective init

# Create dbtective.toml instead
dbtective init --format toml

# Add [tool.dbtective] section to pyproject.toml
dbtective init --format pyproject

# Create config in a specific directory
dbtective init --location ./my_dbt_project
```


## Getting Help

- Command help: `dbtective --help` or `dbtective run --help`
- Documentation: [https://feliblo.github.io/dbtective/](https://feliblo.github.io/dbtective/)
- Issues: [https://github.com/feliblo/dbtective/issues](https://github.com/feliblo/dbtective/issues)
