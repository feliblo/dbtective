<details>
<summary>Common Rule Config</summary>

- **name**: Human-readable name of the rule.
- **severity**: `"error"` (fail) or `"warning"` (warn only).
  - *(optional, defaults to `"error"` if not specified)*
- **description**: Human-readable explanation of the rule.
- **includes**: List of patterns to explicitly include for this rule. Patterns match against `original_file_path` from the manifest.<br>
  &nbsp;&nbsp;See [Includes & Excludes](/docs/includes_excludes) for full pattern syntax, examples, and cross-platform details.<br>
  &nbsp;&nbsp;**Quick examples:**
  &nbsp;&nbsp;&nbsp;&nbsp;`models/staging` - paths containing `models/staging`
  &nbsp;&nbsp;&nbsp;&nbsp;`orders` - paths containing `orders` anywhere
  &nbsp;&nbsp;&nbsp;&nbsp;`^models/staging/` - paths starting with `models/staging/`
  &nbsp;&nbsp;&nbsp;&nbsp;`.sql$` - paths ending with `.sql`
  &nbsp;&nbsp;&nbsp;&nbsp;`models/staging/*.sql` - SQL files in `models/staging/` (not subdirs)
  &nbsp;&nbsp;&nbsp;&nbsp;`models/**/*.sql` - SQL files in any subfolder of `models/`
- **excludes**: List of patterns to explicitly exclude from this rule. Uses the same pattern syntax as `includes`.<br>
  &nbsp;&nbsp;**Quick examples:**
  &nbsp;&nbsp;&nbsp;&nbsp;`models/legacy` - exclude legacy models directory
  &nbsp;&nbsp;&nbsp;&nbsp;`_deprecated` - exclude paths containing `_deprecated`
  &nbsp;&nbsp;&nbsp;&nbsp;`mart_orders` - exclude a specific file by name
- **model_materializations**: Filter models by materialization type. Only applies when `applies_to` includes `models`.<br>
  &nbsp;&nbsp;*(optional, if not specified all materializations are included)*<br>
  &nbsp;&nbsp;**Built-in types:** `table`, `view`, `incremental`, `ephemeral`, `materialized_view`. Custom materializations are also supported.<br>
  &nbsp;&nbsp;**Example:** `["table", "incremental"]`

<hr>

</details>
