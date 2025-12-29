---
title: Rules
type: docs
weight: 2
prev: docs/docs-main
sidebar:
  open: true
---

Here you can find an overview of all possible rules. Use the filter below to find the rules you need, or search using <kbd>Ctrl</kbd> or <kbd>Cmd</kbd> + <kbd>K</kbd>.

<div id="rulesContent">

## Manifest Rules

</br>
<div style="margin-bottom: 20px;">
  <input type="text" id="ruleFilter" placeholder="Filter rules by name, keywords, or category..." style="width: 100%; padding: 10px; border: 1px solid #ccc; border-radius: 4px; font-size: 14px;" />
</div>

<table class="rules-table">
  <thead>
    <tr>
      <th>Rule Name</th>
      <th>Category</th>
      <th>Description</th>
      <th>Keywords</th>
    </tr>
  </thead>
  <tbody>
    <tr class="rule-item" data-keywords="description documentation yaml schema describe docs comments metadata" data-category="manifest">
      <td><a href="description" class="rule-name">has_description</a></td>
      <td><span class="rule-category-badge badge-manifest">Manifest</span></td>
      <td>Check if a description is populated. Ensures objects have a description in their schema (e.g. YAML) files.</td>
      <td style="font-size: 12px; color: #666;">description, documentation, yaml, schema</td>
    </tr>
    <tr class="rule-item" data-keywords="naming pattern regex standards conventions prefixes suffixes name format" data-category="manifest">
      <td><a href="naming_conventions" class="rule-name">name_convention</a></td>
      <td><span class="rule-category-badge badge-manifest">Manifest</span></td>
      <td>Check if object names follow casing (e.g.<code>snake_case</code>) or custom regex patterns. Enforces naming standards using configurable patterns.</td>    <td style="font-size: 12px; color: #666;">naming, pattern, regex, conventions, prefixes</td>
    </tr>
    <tr class="rule-item" data-keywords="tags metadata categorization organization labels tagging" data-category="manifest">
      <td><a href="tags" class="rule-name">has_tags</a></td>
      <td><span class="rule-category-badge badge-manifest">Manifest</span></td>
      <td>Check if objects have the required tags. Ensure proper categorization for selective execution.</td>
      <td style="font-size: 12px; color: #666;">tags, metadata, categorization, organization</td>
    </tr>
    <tr class="rule-item" data-keywords="orphaned unused references dependencies lineage data assets cleanup" data-category="manifest">
      <td><a href="is_not_orphaned" class="rule-name">is_not_orphaned</a></td>
      <td><span class="rule-category-badge badge-manifest">Manifest</span></td>
      <td>Check if objects are referenced by other objects. Identifies orphaned data assets that may be unused or underutilized.</td>
      <td style="font-size: 12px; color: #666;">orphaned, unused, references, dependencies, lineage</td>
    </tr>
    <tr class="rule-item" data-keywords="tests uniqueness unique validation custom" data-category="manifest">
      <td><a href="tests" class="rule-name">has_unique_test</a></td>
      <td><span class="rule-category-badge badge-manifest">Manifest</span></td>
      <td>Check if dbt objects have at least one uniqueness test attached. Supports standard and custom uniqueness tests.</td>
      <td style="font-size: 12px; color: #666;">tests, uniqueness, unique, validation, custom</td>
    </tr>
      <tr class="rule-item" data-keywords="tests metadata keys" data-category="manifest">
      <td><a href="has_metadata_keys" class="rule-name">has_metadata_keys</a></td>
      <td><span class="rule-category-badge badge-manifest">Manifest</span></td>
      <td>Check if dbt objects has the provided keys in the metadata</td>
      <td style="font-size: 12px; color: #666;">tests, uniqueness, unique, validation, custom</td>
    </tr>
    <tr class="rule-item" data-keywords="references upstream dependencies ref source hardcoded sql" data-category="manifest">
      <td><a href="has_refs" class="rule-name">has_refs</a></td>
      <td><span class="rule-category-badge badge-manifest">Manifest</span></td>
      <td>Check if dbt objects have at least one upstream reference using <code>ref()</code> or <code>source()</code>. Identifies objects that may be using hardcoded SQL instead of leveraging dbt's dependency management.</td>
      <td style="font-size: 12px; color: #666;">references, upstream, dependencies, ref, source</td>
    </tr>
    <tr class="rule-item" data-keywords="code lines length size complexity maintainability modularity readability" data-category="manifest">
      <td><a href="max_code_lines" class="rule-name">max_code_lines</a></td>
      <td><span class="rule-category-badge badge-manifest">Manifest</span></td>
      <td>Enforce a maximum line count for code. </td>
      <td style="font-size: 12px; color: #666;">code, lines, length, size, complexity</td>
  </tbody>
</table>

## Catalog Rules

<details>
  <summary><strong>Why differentiate between <code>manifest</code> and <code>catalog</code>?</strong></summary>

  <p>
    These rules use both the <code>manifest.json</code> and <code>catalog.json</code> artifacts. These files can become out of sync during development (for example, when running <code>dbtective</code> in pre-commit hooks), especially if files are moved or renamed and only one of the commands generating <code>manifest.json</code> is run. For more information, see the <a href="https://docs.getdbt.com/reference/artifacts/manifest-json" target="_blank">dbt documentation on manifest.json</a>.
  </p>
  <p>
    To ensure your catalog is up to date, delete it from the dbt target folder and regenerate it using <code>dbt docs generate</code>. Future updates to dbtective will include an option to automate this process with a specific flag.
  </p>
</details>

<table class="rules-table">
  <thead>
    <tr>
      <th>Rule Name</th>
      <th>Category</th>
      <th>Description</th>
      <th>Keywords</th>
    </tr>
  </thead>
  <tbody>
    <tr class="rule-item" data-keywords="columns catalog database schema yml yaml documentation undocumented missing docs column-level" data-category="catalog">
      <td><a href="columns" class="rule-name">columns_all_documented</a></td>
      <td><span class="rule-category-badge badge-catalog">Catalog</span> </td>
      <td>Check if all SQL columns are documented in e.g. their yml file. Validates that database columns match documentation.</td>
      <td style="font-size: 12px; color: #666;">columns, catalog, database, schema, undocumented</td>
    </tr>
    <tr class="rule-item" data-keywords="columns descriptions documentation catalog database schema yml yaml column-level missing docs" data-category="catalog">
      <td><a href="columns" class="rule-name">columns_have_description</a></td>
      <td><span class="rule-category-badge badge-catalog">Catalog</span> </td>
      <td>Check if all documented columns have non-empty descriptions. Ensures column-level documentation is complete</td>
      <td style="font-size: 12px; color: #666;">columns, descriptions, documentation, catalog, database</td>
    </tr>
    <tr class="rule-item" data-keywords="columns naming pattern regex standards conventions prefixes suffixes name format" data-category="catalog">
      <td><a href="columns" class="rule-name">columns_name_convention</a></td>
      <td><span class="rule-category-badge badge-catalog">Catalog</span> </td>
      <td>Check if column names follow casing (e.g.<code>snake_case</code>) or custom regex patterns. Enforces naming standards using configurable patterns.</td>
      <td style="font-size: 12px; color: #666;">columns, naming, pattern, regex, conventions</td>
    </tr>
    <tr class="rule-item" data-keywords="columns canonical naming standards conventions consistency aliases synonyms" data-category="catalog">
      <td><a href="columns" class="rule-name">columns_canonical_name</a></td>
      <td><span class="rule-category-badge badge-catalog">Catalog</span> </td>
      <td>Enforce canonical column naming by flagging invalid name patterns. Supports exceptions for allowed variations.</td>
      <td style="font-size: 12px; color: #666;">columns, canonical, naming, standards, consistency</td>
    </tr>
  </tbody>
</table>

</div>

<script>
document.addEventListener('DOMContentLoaded', function() {
  const filterInput = document.getElementById('ruleFilter');
  const rulesContent = document.getElementById('rulesContent');

  if (filterInput && rulesContent) {
    filterInput.addEventListener('input', function(e) {
      const filterValue = e.target.value.toLowerCase().trim();
      const ruleItems = rulesContent.querySelectorAll('.rule-item');
      const tables = rulesContent.querySelectorAll('.rules-table');

      if (filterValue === '') {
        // Show all
        ruleItems .forEach(item => item.style.display = '');
        tables.forEach(table => table.style.display = '');
        rulesContent.querySelectorAll('h2').forEach(heading => heading.style.display = '');
        return;
      }

      // Filter rows
      ruleItems.forEach(item => {
        const keywords = item.getAttribute('data-keywords') || '';
        const text = item.textContent.toLowerCase();

        if (text.includes(filterValue) || keywords.includes(filterValue)) {
          item.style.display = '';
        } else {
          item.style.display = 'none';
        }
      });

      // Hide tables/sections with no visible rows
      tables.forEach(table => {
        const visibleRows = table.querySelectorAll('tbody .rule-item:not([style*="display: none"])');
        if (visibleRows.length === 0) {
          table.style.display = 'none';
          // Hide the heading before this table
          let prevElement = table.previousElementSibling;
          while (prevElement) {
            if (prevElement.tagName === 'H2') {
              prevElement.style.display = 'none';
              break;
            }
            prevElement = prevElement.previousElementSibling;
          }
        } else {
          table.style.display = '';
          // Show the heading before this table
          let prevElement = table.previousElementSibling;
          while (prevElement) {
            if (prevElement.tagName === 'H2') {
              prevElement.style.display = '';
              break;
            }
            prevElement = prevElement.previousElementSibling;
          }
        }
      });
    });
  }
});
</script>
