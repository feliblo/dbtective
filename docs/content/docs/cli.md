---
title: CLI Reference
description: Command-line interface reference for dbtective
weight: 3
---

# CLI Reference
## Installation

Before using the CLI, make sure dbtective is installed. See the [README](https://github.com/feliblo/dbtective#installation) for installation instructions.

## Global Options

These options are available for all commands:

### `--verbose` / `-v`

Enable verbose logging output for debugging purposes.

```bash
dbtective --verbose run
dbtective -v run
```

When enabled, dbtective will output debug-level logs showing detailed information about what it's doing.

### `--help` / `-h`

Display help information for dbtective or a specific command.

```bash
dbtective --help
dbtective run --help
```

### `--version` / `-V`

Display the version of dbtective.

```bash
dbtective --version
```

## Commands

### `run`

Run dbtective analysis on your dbt project.

**Usage:**

```bash
dbtective run [OPTIONS]
```

**Options:**

#### `--entry-point <PATH>`

Path to your dbt project root directory.

- **Default:** `.` (current directory)
- **Example:** `dbtective run --entry-point "my_dbt_project"`

```bash
dbtective run --entry-point /path/to/dbt/project
```

#### `--config-file` / `-c <PATH>`

Path to the dbtective configuration file.

- **Default:** `dbtective.yml`
- **Example:** `dbtective run --config-file custom-config.yml`

```bash
dbtective run -c path/to/dbtective.yml
```

#### `--manifest-file` / `-m <PATH>`

Path to the dbt manifest.json file.

- **Default:** `target/manifest.json`
- **Example:** `dbtective run --manifest-file custom/manifest.json`

```bash
dbtective run -m target/manifest.json
```

**Note:** Make sure to run `dbt compile` or another command like `dbt build` or `dbt run` before running dbtective to ensure the manifest.json file is up to date.

#### `--pyproject-file` / `-p <PATH>`

Not used currently.
Path to the pyproject.toml file (for future use).

- **Default:** `pyproject.toml`
- **Example:** `dbtective run --pyproject-file custom-pyproject.toml`

```bash
dbtective run -p pyproject.toml
```

Run with default settings (current directory, default config):
```bash
dbtective run
```

Run with verbose output:
```bash
dbtective run --verbose
```

Run on a specific dbt project:
```bash
dbtective run --entry-point ./dbt_project
```

Run with custom configuration file:
```bash
dbtective run --config-file ./configs/dbtective.yml
```

### `init`

Initialize a new dbtective project (planned feature).

**Usage:**

```bash
dbtective init [OPTIONS]
```

**Status:** This command is currently a placeholder for future functionality. It will help generate a starter `dbtective.yml` configuration file.

## Exit Codes

dbtective uses the following exit codes:

- **0** - Success: All checks passed or only warnings were found
- **1** - Failure: One or more checks failed with `severity: "error"`

These exit codes make it easy to integrate dbtective into CI/CD pipelines:

```bash
# In CI/CD pipeline
dbtective run || exit 1
```

## Troubleshooting

### Manifest not found

If dbtective can't find the manifest.json:

```bash
# Make sure you've compiled your dbt project
dbt compile

# Or specify the manifest location explicitly
dbtective run --manifest-file path/to/manifest.json
```

### Configuration file not found

If dbtective can't find the configuration file:

```bash
# Check that dbtective.yml exists in your project root
ls -la dbtective.yml

# Or specify the entrypoint explicitly
dbtective run --entry-point path/to/dbt/project

# Or specify the config location explicitly
dbtective run --config-file path/to/dbtective.yml
```

## Getting Help

- **Command help:** Run `dbtective --help` or `dbtective run --help`
- **Documentation:** Visit [https://feliblo.github.io/dbtective/](https://feliblo.github.io/dbtective/)
- **Issues:** Report bugs or request features at [https://github.com/feliblo/dbtective/issues](https://github.com/feliblo/dbtective/issues)