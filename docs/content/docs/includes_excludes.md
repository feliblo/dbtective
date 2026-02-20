---
title: Includes & Excludes
description: How to filter which dbt objects a rule applies to using path patterns, object names, or tags
weight: 3
---

Every rule supports `includes` and `excludes` to control which dbt objects it runs on. Patterns can match against file paths, object names, or tags.

## Quick reference

| I want to...                             | Pattern                                |
| ---------------------------------------- | -------------------------------------- |
| Target a specific file                   | `"mart_orders"` or `"mart_orders.sql"` |
| Target a directory                       | `"models/staging"`                     |
| Target files directly in a directory     | `"models/staging/*.sql"`               |
| Target files at any depth in a directory | `"models/staging/**"`                  |
| Target by file extension                 | `".sql$"`                              |
| Target paths starting with               | `"^models/staging/"`                   |
| Exact path match                         | `"^models/staging/stg_orders.sql$"`    |
| Target by object name                    | `"name:my_source"`                     |
| Target by name pattern                   | `"name:stg_*"`                         |
| Target by tag                            | `"tag:pii"`                            |
| Target by tag pattern                    | `"tag:pii_*"`                          |

{{< callout type="tip" >}}
Path patterns match against the `original_file_path` field in your `manifest.json`.

If your dbt objects live in subfolders, paths may include a prefix like `dbt/models/...` instead of `models/...`. Check the actual `original_file_path` values in your manifest (located in the `target` folder of dbt) to see the exact paths your patterns will match against.
{{< /callout >}}

## How it works

Patterns are matched against different fields depending on their prefix:

| Prefix   | Matches against                              | Example            |
| -------- | -------------------------------------------- | ------------------ |
| (none)   | `original_file_path` (file path)             | `"models/staging"` |
| `name:`  | Object name (model name, source name, etc.)  | `"name:stg_orders"`|
| `tag:`   | Object tags                                  | `"tag:pii"`        |

### File paths

By default (no prefix), patterns match against the **full `original_file_path`** of each dbt object. This is the path dbt records in `manifest.json`, for example:

```
models/staging/stg_orders.sql
models/marts/finance/fct_revenue.sql
seeds/raw_data.csv
snapshots/orders_snapshot.sql
models/sources.yml                      ← sources use their YAML file path
```

If your dbt project lives in a subdirectory (e.g. a monorepo), `original_file_path` may include a prefix like `dbt/`. Backslash paths (e.g. `models\raw\file.sql`) are automatically normalized to forward slashes before matching, so **always write patterns with forward slashes**.

### Object names (`name:`)

The `name:` prefix matches against the object's name — the same name you see in dbt (model name, source name, macro name, etc.). This is especially useful for **sources**, where multiple sources share the same YAML file path and can't be distinguished by path alone.

| Pattern              | Object name    | Match? |
| -------------------- | -------------- | ------ |
| `name:my_source`     | `my_source`    | Yes    |
| `name:my_source`     | `other_source` | No     |
| `name:stg_*`         | `stg_orders`   | Yes    |
| `name:stg_*`         | `fct_orders`   | No     |

### Object tags (`tag:`)

The `tag:` prefix matches against the object's tags. If the object has **any** tag that matches the pattern, it's considered a match. Glob patterns (`*`, `**`) work with tag patterns too.

| Pattern        | Object tags             | Match? |
| -------------- | ----------------------- | ------ |
| `tag:pii`      | `["pii", "finance"]`    | Yes    |
| `tag:pii`      | `["finance"]`           | No     |
| `tag:pii_*`    | `["pii_email"]`         | Yes    |
| `tag:pii_*`    | `["finance"]`           | No     |

{{< callout type="info" >}}
Tag filtering works for object types that support tags in dbt: **models**, **sources**, and **exposures**. Objects without tags (macros, semantic models, unit tests) will never match a `tag:` pattern.
{{< /callout >}}

## Pattern types (path patterns)

There are four ways to write a path pattern, from simplest to most specific:

### 1. Substring match (no special characters)

A plain string matches if it appears **anywhere** in the path.

| Pattern           | Path                              | Match? | Why                                                |
| ----------------- | --------------------------------- | ------ | -------------------------------------------------- |
| `orders`          | `models/staging/stg_orders.sql`   | Yes    | `orders` appears in the filename                   |
| `orders`          | `models/marts/dim_orders.sql`     | Yes    | `orders` appears in the filename                   |
| `orders`          | `models/marts/dim_customers.sql`  | No     | `orders` not found anywhere                        |
| `staging`         | `models/staging/stg_orders.sql`   | Yes    | `staging` appears in the directory                 |
| `staging`         | `models/marts/dim_orders.sql`     | No     | `staging` not found                                |
| `models/marts`    | `models/marts/dim_orders.sql`     | Yes    | `models/marts` appears in the path                 |
| `models/marts`    | `dbt/models/marts/dim_orders.sql` | Yes    | still appears (subdirectory prefix doesn't matter) |
| `mart_orders.sql` | `models/marts/mart_orders.sql`    | Yes    | exact filename match                               |

This is the most common and simplest way to filter. Use it for:

- Targeting a specific file by name: `"mart_orders"`
- Targeting a directory: `"models/staging"`
- Targeting a keyword: `"_deprecated"`

### 2. Glob patterns (`*` and `**`)

| Symbol | Meaning                                                            |
| ------ | ------------------------------------------------------------------ |
| `*`    | Matches any characters **except** `/` (stays within one directory) |
| `**`   | Matches any characters **including** `/` (crosses directories)     |

| Pattern                | Path                                | Match? | Why                                   |
| ---------------------- | ----------------------------------- | ------ | ------------------------------------- |
| `models/staging/*.sql` | `models/staging/stg_orders.sql`     | Yes    | `*` matches `stg_orders`              |
| `models/staging/*.sql` | `models/staging/sub/stg_orders.sql` | No     | `*` does not cross `/`                |
| `models/**/*.sql`      | `models/staging/stg_orders.sql`     | Yes    | `**` crosses directories              |
| `models/**/*.sql`      | `models/staging/sub/deep.sql`       | Yes    | `**` matches `staging/sub`            |
| `models/**/*.sql`      | `models/orders.sql`                 | No     | `**/` requires at least one directory |

{{< callout type="info" >}}
`**/*` requires at least one subdirectory. To match files at **any** depth (including directly in the folder), use `models/**` instead of `models/**/*`.
{{< /callout >}}

### 3. Anchors (`^` and `$`)

By default, patterns match anywhere in the path (substring/contains). Add anchors to constrain where the pattern must appear:

| Anchor  | Meaning                                         |
| ------- | ----------------------------------------------- |
| `^`     | Pattern must match at the **start** of the path |
| `$`     | Pattern must match at the **end** of the path   |
| `^...$` | Pattern must match the **entire** path (exact)  |

| Pattern                           | Path                                | Match? | Why                                    |
| --------------------------------- | ----------------------------------- | ------ | -------------------------------------- |
| `^models/staging/`                | `models/staging/stg_orders.sql`     | Yes    | path starts with `models/staging/`     |
| `^models/staging/`                | `dbt/models/staging/stg_orders.sql` | No     | path starts with `dbt/`, not `models/` |
| `.sql$`                           | `models/staging/stg_orders.sql`     | Yes    | path ends with `.sql`                  |
| `.sql$`                           | `models/staging/schema.yml`         | No     | path ends with `.yml`                  |
| `^models/staging/stg_orders.sql$` | `models/staging/stg_orders.sql`     | Yes    | exact match                            |
| `^models/staging/stg_orders.sql$` | `dbt/models/staging/stg_orders.sql` | No     | prefix prevents match                  |

{{< callout type="warning" >}}
If your dbt project lives in a subfolder (e.g. a monorepo), `original_file_path` may have a prefix like `dbt/`. Using `^` anchors won't match these paths. Prefer **substring patterns** (without `^`) for maximum compatibility across project structures.
{{< /callout >}}

### 4. Combined (anchors + globs)

Anchors and globs can be used together:

| Pattern                  | Meaning                                     |
| ------------------------ | ------------------------------------------- |
| `^models/staging/*.sql$` | SQL files **directly** in `models/staging/` |
| `^models/**/*.sql$`      | SQL files in **any subfolder** of `models/` |
| `^models/**`             | **Everything** under `models/` at any depth |
| `^seeds/*.csv$`          | CSV files directly in `seeds/`              |

## Evaluation order

When both `includes` and `excludes` are specified, they are evaluated in this order:

1. **Exact exclude** - if the pattern exactly matches (path, name, or tag), the object is excluded
2. **Exact include** - if the pattern exactly matches, the object is included
3. **Pattern exclude** - if any exclude glob/substring matches, the object is excluded
4. **Pattern include** - if any include glob/substring matches, the object is included. If includes are specified but none match, the object is **excluded**
5. **Default** - if no includes or excludes are specified, the object is included

The key takeaway: **excludes always win over includes** at the same specificity level, and **exact matches beat pattern matches**. This applies across all pattern types — path, `name:`, and `tag:` patterns follow the same precedence rules.

## Examples

### Only run a rule on staging models

```yaml
manifest_tests:
  - name: "staging_must_have_description"
    type: "has_description"
    severity: "error"
    applies_to: ["models"]
    includes:
      - "models/staging"
```

### Exclude raw/landing zone from a catalog rule

```yaml
catalog_tests:
  - name: "columns_need_types"
    type: "columns_have_data_type"
    severity: "error"
    applies_to: ["models"]
    excludes:
      - "models/raw"
```

### Run on marts, but skip a specific model

```yaml
manifest_tests:
  - name: "marts_description"
    type: "has_description"
    severity: "error"
    applies_to: ["models"]
    includes:
      - "models/marts"
    excludes:
      - "legacy_orders"
```

### Exclude a specific source by name

```yaml
manifest_tests:
  - name: "sources_must_have_description"
    type: "has_description"
    severity: "error"
    applies_to: ["sources"]
    excludes:
      - "name:my_legacy_source"
```

### Only run on sources matching a name pattern

```yaml
manifest_tests:
  - name: "stg_sources_have_freshness"
    type: "has_freshness"
    severity: "warning"
    applies_to: ["sources"]
    includes:
      - "name:stg_*"
```

### Exclude objects with a specific tag

```yaml
manifest_tests:
  - name: "models_need_description"
    type: "has_description"
    severity: "error"
    applies_to: ["models"]
    excludes:
      - "tag:deprecated"
```

### Only run on objects tagged as PII

```yaml
manifest_tests:
  - name: "pii_models_need_owner"
    type: "has_metadata_keys"
    severity: "error"
    applies_to: ["models"]
    required_keys: ["owner"]
    includes:
      - "tag:pii"
```

### Mix path, name, and tag patterns

```yaml
manifest_tests:
  - name: "marts_description"
    type: "has_description"
    severity: "error"
    applies_to: ["models", "sources"]
    includes:
      - "models/marts"
      - "name:important_source"
    excludes:
      - "tag:deprecated"
```

### Exclude multiple directories

```yaml
catalog_tests:
  - name: "columns_snake_case"
    type: "columns_name_convention"
    pattern: "snake_case"
    severity: "error"
    applies_to: ["models", "seeds"]
    excludes:
      - "models/raw"
      - "models/staging"
      - "seeds/legacy"
```

### Exclude by file extension

```yaml
manifest_tests:
  - name: "sql_files_only"
    type: "has_description"
    severity: "error"
    applies_to: ["models"]
    excludes:
      - ".yml$"
```

## Path normalization

dbtective normalizes all paths to forward slashes before matching. Your patterns should always use forward slashes (`/`).

If your dbt project is in a subfolder, `original_file_path` will include that prefix. Substring patterns handle this automatically:

| `original_file_path` in manifest | Normalized                  | Matches `models/raw`? |
| -------------------------------- | --------------------------- | --------------------- |
| `models/raw/orders.sql`          | `models/raw/orders.sql`     | Yes                   |
| `dbt/models/raw/orders.sql`      | `dbt/models/raw/orders.sql` | Yes                   |
| `models\raw\orders.sql`          | `models/raw/orders.sql`     | Yes                   |
