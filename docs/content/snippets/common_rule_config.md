<details>
<summary>Common Rule Config</summary>

- **name**: Human-readable name of the rule.
- **severity**: `"error"` (fail) or `"warning"` (warn only).
  - *(optional, defaults to `"error"` if not specified)*
- **description**: Human-readable explanation of the rule.
- **includes**: List of patterns to explicitly include for this rule.<br>
  &nbsp;&nbsp;Paths are relative to the `original_file_path` from the manifest.<br>
  &nbsp;&nbsp;**Pattern syntax:**
  - `*` matches any characters except `/` (within a single directory)
  - `**` matches any characters including `/` (across directories)
  - `^` at the start anchors to the beginning of the path
  - `$` at the end anchors to the end of the path
  - Without anchors, pattern matches if it appears anywhere in the path (contains)<br>
  &nbsp;&nbsp;**Examples:**
  &nbsp;&nbsp;&nbsp;&nbsp;`^models/staging/` - paths starting with `models/staging/`
  &nbsp;&nbsp;&nbsp;&nbsp;`orders` - paths containing `orders` anywhere
  &nbsp;&nbsp;&nbsp;&nbsp;`.sql$` - paths ending with `.sql`
  &nbsp;&nbsp;&nbsp;&nbsp;`^models/*.sql$` - SQL files directly in `models/` folder
  &nbsp;&nbsp;&nbsp;&nbsp;`^models/**/*.sql$` - SQL files in any subfolder of `models/`
- **excludes**: List of patterns to explicitly exclude from this rule.<br>
  &nbsp;&nbsp;Uses the same pattern syntax as `includes`.<br>
  &nbsp;&nbsp;**Examples:**
  &nbsp;&nbsp;&nbsp;&nbsp;`^models/legacy/` - exclude legacy models folder
  &nbsp;&nbsp;&nbsp;&nbsp;`_deprecated` - exclude paths containing `_deprecated`
  &nbsp;&nbsp;&nbsp;&nbsp;`^tests/` - exclude test files
- **model_materializations**: Filter models by materialization type. Only applies when `applies_to` includes `models`.<br>
  &nbsp;&nbsp;*(optional, if not specified all materializations are included)*<br>
  &nbsp;&nbsp;**Built-in types:** `table`, `view`, `incremental`, `ephemeral`, `materialized_view`. Custom materializations are also supported.<br>
  &nbsp;&nbsp;**Example:** `["table", "incremental"]`

<hr>

</details>
