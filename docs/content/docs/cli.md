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
| `--config-file <PATH>` | `-c` | `dbtective.yml` | Path to dbtective configuration |
| `--manifest-file <PATH>` | `-m` | `target/manifest.json` | Path to dbt manifest.json |
| `--catalog-file <PATH>` | `-g` | `target/catalog.json` | Path to dbt catalog.json |
| `--only-manifest` | | `true` | Run only manifest checks |
| `--only-catalog` | | `false` | Run only catalog checks |
| `--pyproject-file <PATH>` | `-p` | `pyproject.toml` | Path to pyproject.toml (reserved for future use) |

### `init`

Initialize a new dbtective project.

**Usage:** `dbtective init [OPTIONS]`

**Status:** Placeholder for future functionality to generate a starter `dbtective.yml` configuration.

## Exit Codes

| Code | Description |
|------|-------------|
| `0` | Success - all checks passed or only warnings found |
| `1` | Failure - one or more checks failed with `severity: "error"` |

## Examples

### Basic Usage

```bash
# Run with defaults (current directory, dbtective.yml, target/manifest.json)
dbtective run

# Run with verbose output
dbtective run --verbose

# Run on a specific dbt project
dbtective run --entry-point ./dbt_project
```

### Custom Configuration

```bash
# Use custom config file
dbtective run --config-file ./configs/custom.yml

# Use custom manifest location
dbtective run --manifest-file custom/path/manifest.json

# Combine multiple options
dbtective run --entry-point ./my_project --config-file config.yml --verbose
```

### Catalog Checks

```bash
# Run catalog checks (requires dbt docs generate)
dbtective run --only-catalog

# Run both manifest and catalog checks
dbtective run --only-manifest=false --only-catalog=false
```

### CI/CD Integration

```bash
# In CI/CD pipeline - exit with error code on failures
dbt compile
dbtective run || exit 1
```

## Troubleshooting

**Manifest not found:**
```bash
dbt compile
dbtective run --manifest-file path/to/manifest.json
```

**Configuration file not found:**
```bash
ls -la dbtective.yml
dbtective run --entry-point path/to/dbt/project
dbtective run --config-file path/to/dbtective.yml
```

## Getting Help

- Command help: `dbtective --help` or `dbtective run --help`
- Documentation: [https://feliblo.github.io/dbtective/](https://feliblo.github.io/dbtective/)
- Issues: [https://github.com/feliblo/dbtective/issues](https://github.com/feliblo/dbtective/issues)