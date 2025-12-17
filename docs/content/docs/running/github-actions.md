---
title: GitHub Actions
description: Run dbtective in your CI/CD pipeline with GitHub Actions
weight: 4
---


Run dbtective as part of your CI/CD pipeline using the official GitHub Action.

### Basic Setup

Add `dbtective` to your workflow file (e.g., `.github/workflows/ci.yml`).If you don't have access to your warehouse in the CI environment, you can set `only_manifest: true` to skip catalog based rules. We recommend pinning both a major and minor version number.

```yaml
    - name: Run dbtective
      uses: feliblo/dbtective@v0.1.28
      with:
        config-file: "dbtective.yml"
        entry-point: "."
        verbose: "false"
```

Complete example:

```yaml
name: CI pipeline

on:
  pull_request:
    branches:
      - main

jobs:
  dbtective:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Generate or fetch dbt artifacts
        run: |
          pip install dbt-core dbt-[adapter]  # e.g., dbt-postgres, dbt-bigquery, etc.
          dbt compile # to generate manifest.json
          dbt docs generate # to generate catalog.json

      - name: Run dbtective
        uses: feliblo/dbtective@v0.1.28
        with:
          config-file: "dbtective.yml"
          entry-point: "."
          verbose: "false"
```

### Configuration Options

| Input | Default | Description |
|-------|---------|-------------|
| `config-file` | `dbtective.yml` | Location of the YML config file |
| `entry-point` | `.` | Path to dbt project root directory |
| `manifest-file` | `target/manifest.json` | Path to dbt manifest file |
| `verbose` | `false` | Run dbtective in verbose mode |
| `version` | `latest` | Version of dbtective to install (e.g., 'v0.1.10' or 'latest') |

### Exit Codes

The action will:

- Exit with code `0` if all rules pass or only warnings are found
- Exit with code `1` if any rules fail with `severity: "error"`

This allows you to fail the CI pipeline when dbtective detects issues.

## Getting Help

- Action source: [action.yml](https://github.com/feliblo/dbtective/blob/main/action.yml)
- Documentation: [https://feliblo.github.io/dbtective/](https://feliblo.github.io/dbtective/)
- Issues: [https://github.com/feliblo/dbtective/issues](https://github.com/feliblo/dbtective/issues)
