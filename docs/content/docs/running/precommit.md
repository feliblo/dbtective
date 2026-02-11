---
title: Pre-commit & Prek
description: Using dbtective with pre-commit hooks and prek
weight: 4
---

This guide covers how to integrate dbtective with pre-commit hooks using [pre-commit](https://pre-commit.com/) or [prek](https://github.com/j178/prek).

For details on `--only-manifest` mode, manifest fallback for catalog rules, and our recommendations for local vs CI/CD usage, see [Only Manifest Mode](../running/manifest-only).

## Pre-commit Setup

We recommend running with `--only-manifest` and `--hide-warnings` for pre-commit hooks:

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/feliblo/dbtective
    rev: v0.2.1
    hooks:
      - id: dbtective-run
        entry: dbtective run
        args: [--only-manifest, --hide-warnings]
```

This avoids stale catalog issues while still catching metadata problems. Eligible catalog rules will automatically [fall back to manifest data](../running/manifest-only#manifest-fallback-for-catalog-rules).
