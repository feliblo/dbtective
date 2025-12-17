<details>
<summary>Common Rule Config</summary>

- **name**: Human-readable name of the rule.
- **severity**: `"error"` (fail) or `"warning"` (warn only).
  - *(optional, defaults to `"error"` if not specified)*
- **description**: Human-readable explanation of the rule.
- **includes**: List of file paths or wildcard patterns to explicitly include for this rule.<br>
  &nbsp;&nbsp;Paths are relative to the `entrypoint`.<br>
  &nbsp;&nbsp;**Examples:**
  &nbsp;&nbsp;&nbsp;&nbsp;`models/my_model.sql`
  &nbsp;&nbsp;&nbsp;&nbsp;`models/**/*.sql`
  &nbsp;&nbsp;&nbsp;&nbsp;`seeds/*_data.csv`
- **excludes**: List of file paths or wildcard patterns to explicitly exclude from this rule.<br>
  &nbsp;&nbsp;Paths are relative to the `entrypoint`.<br>
  &nbsp;&nbsp;**Examples:**
  &nbsp;&nbsp;&nbsp;&nbsp;`models/legacy_model.sql`
  &nbsp;&nbsp;&nbsp;&nbsp;`models/archive/*`
  &nbsp;&nbsp;&nbsp;&nbsp;`seeds/old_data.csv`

<hr>

</details>
