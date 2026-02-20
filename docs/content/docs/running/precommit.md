---
title: Pre-commit & Prek
description: Using dbtective with pre-commit hooks and prek
weight: 4
---

This guide covers how to integrate dbtective with pre-commit hooks using [pre-commit](https://pre-commit.com/) or [prek](https://github.com/j178/prek).

For details on `--only-manifest` mode, manifest fallback for catalog rules, and our recommendations for local vs CI/CD usage, see [Only Manifest Mode](../running/manifest-only).

## Pre-commit Setup

We recommend running with `--auto-parse`, `--only-manifest`, and `--hide-warnings` for pre-commit hooks:

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/feliblo/dbtective
    rev: v0.2.8
    hooks:
      - id: dbtective-run
        entry: dbtective run
        args: [--auto-parse, --only-manifest, --hide-warnings]
```

`--auto-parse` runs your configured `auto_parse_command` (e.g. `dbt parse`, `uv run dbt parse`) before reading the manifest, so it is always up to date. Configure it in your dbtective config:

```yaml
# dbtective.yml
config:
  auto_parse_command: "dbt parse"
```

`dbtective init` detects uv and poetry automatically and suggests the right command.

This avoids stale catalog and manifest issues while still catching metadata problems. Eligible catalog rules will automatically [fall back to manifest data](../running/manifest-only#manifest-fallback-for-catalog-rules).
