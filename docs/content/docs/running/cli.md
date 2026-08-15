---
title: CLI Reference
description: Command-line interface reference for dbtective
weight: 3
---

## Installation

See the [README](https://github.com/feliblo/dbtective#installation) for installation instructions.

## Global Options

| Option            | Description                   |
| ----------------- | ----------------------------- |
| `--verbose`, `-v` | Enable verbose logging output |
| `--help`, `-h`    | Display help information      |
| `--version`, `-V` | Display version               |

## Commands

### `run`

Run dbtective analysis on your dbt project.

**Usage:** `dbtective run [OPTIONS]`

**Important:**

- Before running manifest-based rules, run `dbt compile`, `dbt build`, `dbt run` or any of the [documented commands](https://docs.getdbt.com/reference/artifacts/manifest-json) to ensure `manifest.json` is up to date. Alternatively, use `--auto-parse` to let dbtective handle this automatically (recommended for pre-commit/prek).
- Before running catalog-based rules, run `dbt docs generate` to ensure `catalog.json` is available.
- On dbt v2, use `dbt build --write-index`. See [Artifact formats](../artifact-formats).

#### Options

| Option                     | Short | Default                | Description                                                                                            |
| -------------------------- | ----- | ---------------------- | ------------------------------------------------------------------------------------------------------ |
| `--entry-point <PATH>`     |       | `.`                    | Path to dbt project root                                                                               |
| `--config-file <PATH>`     | `-c`  | Auto-detected          | Path to dbtective configuration from the entry-point (overrides auto-detection)                        |
| `--manifest-file <PATH>`   | `-m`  | `target/manifest.json` | Path to dbt manifest.json                                                                              |
| `--catalog-file <PATH>`    | `-g`  | `target/catalog.json`  | Path to dbt catalog.json                                                                               |
| `--index-dir <PATH>`       |       | `target/index`         | Path to the dbt v2 Parquet index                                                                       |
| `--artifact-format <FMT>`  |       | `auto`                 | Which artifacts to read: `auto`, `json`, or `parquet`                                                  |
| `--only-manifest`          |       | `false`                | Run only manifest rules (recommended for local & pre-commit/prek)                                      |
| `--disable-hyperlinks`     |       | `false`                | Disable file hyperlinks in the output                                                                  |
| `--hide-warnings`          |       | `false`                | Hide warnings from output (only show errors)                                                           |
| `--hide-catalog-tip`       |       | `false`                | Hide the catalog mismatch tip shown when catalog tests fail                                            |
| `--output-format <FORMAT>` |       | `table`                | Output format: `table`, `json`, `csv`, or `ndjson`                                                     |
| `--auto-parse`             |       | `false`                | Run the configured `auto_parse_command` before reading the manifest (recommended for pre-commit hooks) |
| `--output-file <PATH>`     |       |                        | Write output to a file instead of stdout (does nothing in combination with `table` output format).     |

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

# Output results as JSON
dbtective run --output-format json

# Output results as CSV to a file
dbtective run --output-format csv --output-file results.csv

# Output results as newline-delimited JSON (NDJSON) for streaming
dbtective run --output-format ndjson

# Auto-parse the manifest before running (recommended for pre-commit hooks)
dbtective run --auto-parse --only-manifest

# Pipe JSON output for downstream processing
dbtective run --output-format json | jq '.metadata'
```

#### Auto-Parse

The `--auto-parse` flag runs the configured `auto_parse_command` before reading the manifest, ensuring `manifest.json` is always up to date. This is recommended for pre-commit hooks where the manifest may be stale.

To configure the command, add an `auto_parse_command` under the `config` section of your dbtective config:

```yaml
# dbtective.yml
config:
  auto_parse_command: "dbt parse" # or "uv run dbt parse", "poetry run dbt parse"
```

`dbtective init` automatically detects uv (`uv.lock`) or poetry (`poetry.lock`) and suggests the appropriate command.

**Note:** This adds a performance penalty as dbt needs to parse your project first. Only use `--auto-parse` in pre-commit hooks — in CI, generate the manifest as a separate step instead.

#### Structured Output Formats

When using `--output-format`, dbtective produces machine-readable output instead of the terminal table. The exit code behavior is unchanged: exit `0` if all rules pass (or only warnings), exit `1` if any errors.

**JSON** (`--output-format json`) produces a nested structure with metadata, summary, and results:

```json
{
  "metadata": {
    "dbtective_version": "0.2.2",
    "timestamp": "2026-02-16T14:30:00.123+00:00",
    "project_name": "my_dbt_project",
    "manifest_path": "target/manifest.json",
    "catalog_path": "target/catalog.json",
    "execution_time_seconds": 0.042
  },
  "summary": {
    "total": 2,
    "errors": 1,
    "warnings": 1,
    "pass": false
  },
  "results": [
    {
      "severity": "error",
      "object_type": "Model",
      "rule_name": "has_description",
      "message": "Model 'orders' is missing a description",
      "relative_path": "models/orders.sql"
    }
  ]
}
```

**CSV** (`--output-format csv`) and **NDJSON** (`--output-format ndjson`) produce flat rows with metadata and summary repeated on every row, making them suitable for loading into databases or streaming pipelines:

```
dbtective_version,timestamp,project_name,manifest_path,catalog_path,execution_time_seconds,total,total_errors,total_warnings,pass,severity,object_type,rule_name,message,relative_path
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
6. **Auto-parse command** — which command to use for `--auto-parse` (e.g. `dbt parse`, `uv run dbt parse`, `poetry run dbt parse`). dbtective auto-detects uv and poetry by looking for their lock files.

The generated config file is ready to use. You can edit it afterwards to fine-tune any rule options.

#### Options

| Option              | Short | Default | Description                                     |
| ------------------- | ----- | ------- | ----------------------------------------------- |
| `--location <PATH>` | `-l`  | `.`     | Directory where the config file will be created |

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
