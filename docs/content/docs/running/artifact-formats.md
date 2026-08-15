---
title: Artifact formats
description: Reading dbt v1 JSON artifacts or the dbt v2 Parquet index
weight: 6
---

dbtective reads two artifact formats:

| Format | Files | Produced by |
| --- | --- | --- |
| JSON | `target/manifest.json`, `target/catalog.json` | dbt v1, and dbt v2 |
| Parquet index | `target/index/*.parquet` | dbt v2 with `--write-index` |

Rules behave identically either way — the format only changes how metadata is read.

## Generating the Parquet index

```bash
dbt build --write-index --static-analysis strict
```

`--static-analysis strict` is what populates warehouse column types, so catalog
rules such as `columns_have_data_type` work without a separate `catalog.json`.
Without it the index still parses, and those rules fall back to
[manifest data](../manifest-only).

`--write-index` also works with `parse`, `compile` and `run`, but `parse` writes
no column types at all.

## Which format is used

By default dbtective picks automatically:

1. Only one format present → that one is used.
2. Both present → the Parquet index wins, unless `manifest.json` was written by a
   strictly newer dbt version.

dbt v2 writes both formats, so the index is normally preferred. Run with
`--verbose` to see which was chosen.

Force a format with `--artifact-format`:

```bash
dbtective run --artifact-format parquet
dbtective run --artifact-format json
```

Use `--index-dir` if the index is not at `target/index`.

## Consistency with the JSON artifacts

The same project produces the same findings either way. dbtective's own test
project reports identical results from both formats.

That parity relies on the JSON artifacts still being present, because **as of
15/08/2026 `dbt-core 2.0.0-beta.1` does not write everything to the Parquet index
that it writes to the JSON artifacts**:

| Not in the index | Read from |
| --- | --- |
| `raw_code`, `loader`, declared columns, the macro list, user-defined functions | `manifest.json` |
| Full warehouse columns — the index records 31 typed columns of 616 in our test project, and 2 for a snapshot where the catalog has 8 | `catalog.json` |

dbt v2 writes both files alongside the index, so this is normally automatic.
Only the missing keys are deserialized from `manifest.json`, so the cost is small.
Note the column gap is not caused by failed builds — it is the same on a build
with no errors.

If the JSON artifacts are absent, dbtective still reads the index on its own, but
rules that depend on the fields above cannot fire. Generate the catalog with
`dbt compile --write-catalog` (or `dbt build --write-index --write-catalog`);
`dbt docs generate` no longer exists in v2.
