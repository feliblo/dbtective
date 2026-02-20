---
title: Only Manifest Mode
description: When to use --only-manifest and how catalog rules fall back to manifest data
weight: 5
---

dbtective has two types of rules: **manifest rules** and **catalog rules**. Manifest rules only need `manifest.json`, while catalog rules need both `manifest.json` and `catalog.json`.

The `--only-manifest` flag tells dbtective to skip loading `catalog.json`. This is the recommended mode for local development and pre-commit hooks.

## When to Use `--only-manifest`

### Pre-commit & Prek (recommended)

Use `--only-manifest` for all local hooks. The `catalog.json` goes stale quickly during local development because it is **only** updated by `dbt docs generate` (which takes a long time), while `manifest.json` is updated by most dbt commands (`dbt run`, `dbt build`, `dbt compile`, etc.). Running catalog rules against a stale catalog produces misleading failures.

Combine with `--auto-parse` to ensure the manifest is always up to date before running rules. See [Pre-commit & Prek](../running/precommit) for setup details.

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/feliblo/dbtective
    rev: v0.2.10
    hooks:
      - id: dbtective-run
        entry: dbtective run
        args: [--auto-parse, --only-manifest, --hide-warnings]
```

### Do not use `--only-manifest` in CI/CD (use a catalog here)

CI/CD is the **only place** where we recommend running both catalog and manifest rules. Generate a fresh catalog and manifest before running dbtective:

```yaml
# GitHub Actions example
- name: Generate dbt artifacts
  run: |
    dbt compile
    dbt docs generate

- name: Run dbtective
  uses: feliblo/dbtective@v0.2.10
  with:
    entry-point: "."
    # No --only-manifest flag: runs all tests including catalog
```

This ensures all rules run against freshly generated artifacts that match the current state of your project. See the [GitHub Actions](../running/github-actions) docs for full setup details.

## Manifest Fallback for Catalog Rules

When `--only-manifest` is set **and** your config contains `catalog_tests`, dbtective does not silently skip all catalog rules. Instead, eligible catalog rules automatically fall back to running against manifest data only. Rules that cannot produce meaningful results from manifest data alone are skipped.

Check individual rule documentation for the <span class="rule-category-badge badge-manifest-fallback">Fallback</span> badge to see which rules support fallback.

### How it works

Fallback rules reuse the same rule logic but read only from `manifest.json` instead of `catalog.json`. This means:

- Results are based on what you declared in your YAML schema files, not what exists in the actual database.
- Objects or attributes that exist in the database but are not declared in your YAML files will not be seen by fallback mode.
- Results may differ from running with a full catalog.

When fallback mode produces failures, a warning is printed below the results table listing which rules ran in fallback and which were skipped.

### `data_types` filter in fallback mode

The `data_types` filter uses the `data_type` field from your YAML schema files in fallback mode, not the physical database type from `catalog.json`. Results may differ if your YAML types don't exactly match catalog types.

## Summary

| Environment               | Flags                            | Catalog Rules                                |
| ------------------------- | -------------------------------- | -------------------------------------------- |
| Local / pre-commit / prek | `--auto-parse --only-manifest`   | Eligible rules fall back to manifest data    |
| CI/CD                     | _(none)_                         | Full catalog rules with fresh `catalog.json` |
