<details>
<summary>Common Rule Config</summary>

<ul>
  <li><b>name</b>: Human-readable name of the rule.</li>
  <li><b>severity</b>: <code>"error"</code> (fail) or <code>"warning"</code> (warn only).
    <ul><li><i>(optional, defaults to <code>"error"</code> if not specified)</i></li></ul>
  </li>
  <li><b>description</b>: Human-readable explanation of the rule.</li>
  <li><b>includes</b>: List of patterns to explicitly include for this rule. See <a href="/docs/includes_excludes">Includes & Excludes</a> for pattern syntax and examples.</li>
  <li><b>excludes</b>: List of patterns to explicitly exclude from this rule. See <a href="/docs/includes_excludes">Includes & Excludes</a> for pattern syntax and examples.</li>
  <li><b>model_materializations</b>: Filter models by materialization type. Only applies when <code>applies_to</code> includes <code>models</code>.
    <ul>
      <li><i>(optional, if not specified all materializations are included)</i></li>
      <li>Built-in types: <code>table</code>, <code>view</code>, <code>incremental</code>, <code>ephemeral</code>, <code>materialized_view</code>. Custom materializations are also supported.</li>
      <li>Example: <code>["table", "incremental"]</code></li>
    </ul>
  </li>
</ul>

<hr>

</details>
