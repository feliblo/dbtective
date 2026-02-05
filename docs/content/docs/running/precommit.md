---
title: Pre-commit & Prek
description: Using dbtective with pre-commit hooks and prek
weight: 4
---

This guide covers how to integrate dbtective with pre-commit hooks (using [pre-commit](https://pre-commit.com/) or [prek](https://github.com/feliblo/prek)) and best practices for avoiding common issues.

## The Catalog Mismatch Problem

When using dbtective locally with pre-commit hooks, you may encounter **catalog/manifest mismatches**. This happens because:

1. The `manifest.json` is updated automatically by most dbt commands (`dbt run`, `dbt build`, `dbt compile`, etc.)
2. The `catalog.json` is **only** updated when you run `dbt docs generate`
3. If you modify models and run dbt commands without regenerating the catalog, your `catalog.json` becomes stale

This causes catalog tests to fail with misleading errors.

## Recommended Setup

### Use `--only-manifest` for Local Development & pre-commit hooks

For pre-commit/prek hooks, we recommend running **only manifest tests** locally. We also recommend using `--hide-warnings` to keep the output clean and only show errors that block commits:

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/feliblo/dbtective
    rev: v0.1.31
    hooks:
      - id: dbtective-run
        entry: dbtective run
        args: [--only-manifest, --hide-warnings]
```

This avoids the catalog mismatch problem entirely while still catching most metadata issues.

### Run Full Tests in CI/CD

In your CI/CD pipeline, generate a fresh catalog before running dbtective with all tests:

```yaml
# GitHub Actions example
- name: Generate dbt artifacts
  run: |
    dbt compile
    dbt docs generate

- name: Run dbtective
  uses: feliblo/dbtective@v0.1.31
  with:
    entry-point: "."
    # No --only-manifest flag: runs all tests including catalog
```

This ensures catalog tests run against a freshly generated `catalog.json`.
