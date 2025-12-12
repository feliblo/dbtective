---
title: has_tags
type: docs
prev: docs/checks
sidebar:
  open: true
---


### Check: `has_tags`

<br>
<details open>
<summary>has_tags details</summary>
<br>
This check ensures that dbt objects (model, seed, source, macro, etc.) contain tags specified in the configuration. It can be configured to "all", "any" or "one_of" (maximum one) tags from a specified list.

---

**Configuration**

- **type**: Must be `has_tags`.
- **applies_to**: *(optional)* List of dbt object types to check.
  - Default: `["models", "seeds", "snapshots", "analyses", "sources", "exposures"]`
  - Options: `models`, `seeds`, `snapshots`, `analyses`, `sources`, `exposures`
- **tags**: List of tags to check for.
- **criteria**: Criteria for tag presence.
  - Options:
    - `all`: All specified tags must be present.
    - `any`: At least one of the specified tags must be present.
    - `one_of`: Exactly one of the specified tags must be present.
  - Default: `all`

{{< include-markdown "content/snippets/common_check_config.md" >}}

**Example Config**

{{< tabs items="dbtective.yml,dbtective.toml,pyproject.toml" >}}

{{< tab >}}

```yaml
manifest_tests:
  - name: "everything_has_tags"
    type: "has_tags"
    required_tags: ["tag1", "tag2", "tag3"]
    criteria: "all"  # options: "all", "any", "one_of"
    description: "Everything must have tags."
    # severity: "warning"  (optional)
    # applies_to: ['models', 'seeds']  (optional)
    # includes: ["path/to/include/*"]
    # excludes: ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[manifest_tests]]
name = "everything_has_tags"
type = "has_tags"
required_tags = ["tag1", "tag2", "tag3"]
criteria = "all"  # options: "all", "any", "one_of"
description = "Everything must have tags."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< tab >}}

```toml
[[tool.dbtective.manifest_tests]]
name = "everything_has_tags"
type = "has_tags"
required_tags = ["tag1", "tag2", "tag3"]
criteria = "all"  # options: "all", "any", "one_of"
description = "Everything must have tags."
# severity = "warning"  # (optional)
# applies_to = ["models", "seeds"]  # (optional)
# includes = ["path/to/include/*"]
# excludes = ["path/to/exclude/*"]
```

{{< /tab >}}

{{< /tabs >}}

<details closed>
<summary>Relevant dbt code</summary>

```yaml
models:
  - name: model_with_tags
    # Either in config block
    config:
        tags:
          - tag1
          - tag2
    # Or directly as a property
    tags: ["tag1", "tag2"]
  - name: model_without_tags
```

</details>

</details>
