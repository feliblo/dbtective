---
title: Rules
type: docs
weight: 2
prev: docs/docs-main
sidebar:
  open: true
---

Here you can find an overview of all possible rules. Use the filter below to find the rules you need, or search using <kbd>Ctrl</kbd> or <kbd>Cmd</kbd> + <kbd>K</kbd>.

<br>
<br>

{{% details title="What are Rule Categories?" closed="true" %}}

Each rule has a default **category** that classifies what aspect of your dbt project it checks. Categories are included in structured output formats (JSON, CSV, NDJSON) but are not shown in the CLI table.

The default categories are:
<span class="rule-category-badge badge-documentation">Documentation</span>
<span class="rule-category-badge badge-naming">Naming</span>
<span class="rule-category-badge badge-testing">Testing</span>
<span class="rule-category-badge badge-governance">Governance</span>
<span class="rule-category-badge badge-structure">Structure</span>
<span class="rule-category-badge badge-performance">Performance</span>

You can override the default category for any rule by setting `category: "your_custom_category"` in your rule configuration.

{{% /details %}}

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
    <tr class="rule-item" data-keywords="description documentation yaml schema describe docs comments metadata" data-category="documentation">
      <td><a href="description" class="rule-name">has_description</a></td>
      <td><span class="rule-category-badge badge-documentation">Documentation</span></td>
      <td>Check if a description is populated. Ensures objects have a description in their schema (e.g. YAML) files.</td>
      <td style="font-size: 12px; color: #666;">description, documentation, yaml, schema</td>
    </tr>
    <tr class="rule-item" data-keywords="naming pattern regex standards conventions prefixes suffixes name format" data-category="naming">
      <td><a href="naming_conventions" class="rule-name">name_convention</a></td>
      <td><span class="rule-category-badge badge-naming">Naming</span></td>
      <td>Check if object names follow casing (e.g.<code>snake_case</code>) or custom regex patterns. Enforces naming standards using configurable patterns.</td>    <td style="font-size: 12px; color: #666;">naming, pattern, regex, conventions, prefixes</td>
    </tr>
    <tr class="rule-item" data-keywords="tags metadata categorization organization labels tagging" data-category="governance">
      <td><a href="tags" class="rule-name">has_tags</a></td>
      <td><span class="rule-category-badge badge-governance">Governance</span></td>
      <td>Check if objects have the required tags. Ensure proper categorization for selective execution.</td>
      <td style="font-size: 12px; color: #666;">tags, metadata, categorization, organization</td>
    </tr>
    <tr class="rule-item" data-keywords="orphaned unused references dependencies lineage data assets cleanup" data-category="structure">
      <td><a href="is_not_orphaned" class="rule-name">is_not_orphaned</a></td>
      <td><span class="rule-category-badge badge-structure">Structure</span></td>
      <td>Check if objects are referenced by other objects. Identifies orphaned data assets that may be unused or underutilized.</td>
      <td style="font-size: 12px; color: #666;">orphaned, unused, references, dependencies, lineage</td>
    </tr>
    <tr class="rule-item" data-keywords="tests uniqueness unique validation custom" data-category="testing">
      <td><a href="tests" class="rule-name">has_unique_test</a></td>
      <td><span class="rule-category-badge badge-testing">Testing</span></td>
      <td>Check if dbt objects have at least one uniqueness test attached. Supports standard and custom uniqueness tests.</td>
      <td style="font-size: 12px; color: #666;">tests, uniqueness, unique, validation, custom</td>
    </tr>
      <tr class="rule-item" data-keywords="tests metadata keys" data-category="governance">
      <td><a href="has_metadata_keys" class="rule-name">has_metadata_keys</a></td>
      <td><span class="rule-category-badge badge-governance">Governance</span></td>
      <td>Check if dbt objects has the provided keys in the metadata</td>
      <td style="font-size: 12px; color: #666;">tests, uniqueness, unique, validation, custom</td>
    </tr>
    <tr class="rule-item" data-keywords="references upstream dependencies ref source hardcoded sql" data-category="governance">
      <td><a href="has_refs" class="rule-name">has_refs</a></td>
      <td><span class="rule-category-badge badge-governance">Governance</span></td>
      <td>Check if dbt objects have at least one upstream reference using <code>ref()</code> or <code>source()</code>. Identifies objects that may be using hardcoded SQL instead of leveraging dbt's dependency management.</td>
      <td style="font-size: 12px; color: #666;">references, upstream, dependencies, ref, source</td>
    </tr>
    <tr class="rule-item" data-keywords="code ref source references hardcoded sql lineage dependency raw_code" data-category="governance">
      <td><a href="code#rule-code_contains_refs" class="rule-name">code_contains_refs</a></td>
      <td><span class="rule-category-badge badge-governance">Governance</span></td>
      <td>Check if SQL code contains <code>ref()</code> or <code>source()</code> function calls. Strips comments before checking. Case-insensitive.</td>
      <td style="font-size: 12px; color: #666;">code, ref, source, references, hardcoded, sql</td>
    </tr>
    <tr class="rule-item" data-keywords="code lines length size complexity maintainability modularity readability" data-category="performance">
      <td><a href="code#rule-max_code_lines" class="rule-name">max_code_lines</a></td>
      <td><span class="rule-category-badge badge-performance">Performance</span></td>
      <td>Enforce a maximum line count for code. </td>
      <td style="font-size: 12px; color: #666;">code, lines, length, size, complexity</td>
    </tr>
    <tr class="rule-item" data-keywords="forbidden code patterns jinja macros sql banned prohibited disallowed raw_code" data-category="performance">
      <td><a href="code#rule-has_forbidden_code" class="rule-name">has_forbidden_code</a></td>
      <td><span class="rule-category-badge badge-performance">Performance</span></td>
      <td>Check if code contains forbidden patterns, such direct selects <code>SELECT *</code></td>
      <td style="font-size: 12px; color: #666;">forbidden, code, patterns, jinja, macros</td>
    </tr>
    <tr class="rule-item" data-keywords="joins code complexity sql join count limit threshold raw_code" data-category="performance">
      <td><a href="code#rule-max_joins" class="rule-name">max_joins</a></td>
      <td><span class="rule-category-badge badge-performance">Performance</span></td>
      <td>Enforce a maximum number of JOINs in SQL code. Strips comments before counting. Helps reduce code complexity.</td>
      <td style="font-size: 12px; color: #666;">joins, code, complexity, sql, limit</td>
    </tr>
    <tr class="rule-item" data-keywords="folder structure organization path subfolders hierarchy medallion bronze silver gold source" data-category="naming">
      <td><a href="allowed_subfolders" class="rule-name">allowed_subfolders</a></td>
      <td><span class="rule-category-badge badge-naming">Naming</span></td>
      <td>Enforce that objects are organized within specific allowed subfolders. Ensures consistent project structure and proper categorization by source or domain.</td>
      <td style="font-size: 12px; color: #666;">folder, structure, organization, path, subfolders</td>
    </tr>
    <tr class="rule-item" data-keywords="loader source etl elt pipeline ingestion fivetran stitch airflow documentation" data-category="documentation">
      <td><a href="has_loader" class="rule-name">sources_have_loader</a></td>
      <td><span class="rule-category-badge badge-documentation">Documentation</span></td>
      <td>Check if sources have a loader defined. Ensures sources document which tool loads data into the warehouse.</td>
      <td style="font-size: 12px; color: #666;">loader, source, etl, pipeline, ingestion</td>
    </tr>
    <tr class="rule-item" data-keywords="freshness source staleness warn error monitoring data quality loaded_at" data-category="governance">
      <td><a href="has_freshness" class="rule-name">sources_have_freshness</a></td>
      <td><span class="rule-category-badge badge-governance">Governance</span></td>
      <td>Check if sources have freshness configured. Ensures sources define acceptable data staleness thresholds.</td>
      <td style="font-size: 12px; color: #666;">freshness, source, staleness, monitoring, data quality</td>
    </tr>
    <tr class="rule-item" data-keywords="upstream fan-in ref source consuming complexity limit threshold lineage dag" data-category="structure">
      <td><a href="fan-in-fan-out#rule-max_upstream_dependencies" class="rule-name">max_upstream_dependencies</a></td>
      <td><span class="rule-category-badge badge-structure">Structure</span></td>
      <td>Limit <b>fan-in</b>. How many other models a model can select from via ref() and source().</td>
      <td style="font-size: 12px; color: #666;">upstream, fan-in, ref, source, lineage</td>
    </tr>
    <tr class="rule-item" data-keywords="downstream fan-out bottleneck blast radius limit threshold lineage dag consumers" data-category="structure">
      <td><a href="fan-in-fan-out#rule-max_downstream_dependencies" class="rule-name">max_downstream_dependencies</a></td>
      <td><span class="rule-category-badge badge-structure">Structure</span></td>
      <td>Limit <b>fan-out</b>. How many other models can ref() a single model.</td>
      <td style="font-size: 12px; color: #666;">downstream, fan-out, bottleneck, blast radius, lineage</td>
    </tr>
  </tbody>
</table>

## Catalog Rules

{{% details title="Why differentiate between manifest and catalog?" closed="true" %}}

These rules use both the `manifest.json` and `catalog.json` artifacts. These files can become out of sync during development (for example, when running `dbtective` in pre-commit hooks), especially if files are moved or renamed and only one of the commands generating `manifest.json` is run. For more information, see the [dbt documentation on manifest.json](https://docs.getdbt.com/reference/artifacts/manifest-json).

To ensure your catalog is up to date, delete it from the dbt target folder and regenerate it using `dbt docs generate`. Future updates to dbtective will include an option to automate this process with a specific flag.

{{% /details %}}

{{% details title="What is Fallback?" closed="true" %}}

When running with `--only-manifest`, some catalog rules can still run using only manifest data. These rules are marked with the <span class="rule-category-badge badge-manifest-fallback">Fallback</span> badge. Rules without this badge require data from `catalog.json` and are skipped.

See [Only Manifest Mode](../running/manifest-only) for full details.

{{% /details %}}

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
    <tr class="rule-item" data-keywords="columns catalog database schema yml yaml documentation undocumented missing docs column-level" data-category="documentation">
      <td><a href="columns#rule-columns_all_documented" class="rule-name">columns_all_documented</a></td>
      <td><span class="rule-category-badge badge-documentation">Documentation</span> <span class="rule-category-badge badge-catalog">Catalog</span></td>
      <td>Check if all SQL columns are documented in e.g. their yml file. Validates that database columns match documentation.</td>
      <td style="font-size: 12px; color: #666;">columns, catalog, database, schema, undocumented</td>
    </tr>
    <tr class="rule-item" data-keywords="columns descriptions documentation catalog database schema yml yaml column-level missing docs fallback manifest" data-category="documentation">
      <td><a href="columns#rule-columns_have_description" class="rule-name">columns_have_description</a></td>
      <td><span class="rule-category-badge badge-documentation">Documentation</span> <span class="rule-category-badge badge-catalog">Catalog</span> <span class="rule-category-badge badge-manifest-fallback">Fallback</span></td>
      <td>Check if all documented columns have non-empty descriptions. Ensures column-level documentation is complete</td>
      <td style="font-size: 12px; color: #666;">columns, descriptions, documentation, catalog, database</td>
    </tr>
    <tr class="rule-item" data-keywords="columns naming pattern regex standards conventions prefixes suffixes name format fallback manifest" data-category="naming">
      <td><a href="columns#rule-columns_name_convention" class="rule-name">columns_name_convention</a></td>
      <td><span class="rule-category-badge badge-naming">Naming</span> <span class="rule-category-badge badge-catalog">Catalog</span> <span class="rule-category-badge badge-manifest-fallback">Fallback</span></td>
      <td>Check if column names follow casing (e.g.<code>snake_case</code>) or custom regex patterns. Enforces naming standards using configurable patterns.</td>
      <td style="font-size: 12px; color: #666;">columns, naming, pattern, regex, conventions</td>
    </tr>
    <tr class="rule-item" data-keywords="columns canonical naming standards conventions consistency aliases synonyms fallback manifest" data-category="naming">
      <td><a href="columns#rule-columns_canonical_name" class="rule-name">columns_canonical_name</a></td>
      <td><span class="rule-category-badge badge-naming">Naming</span> <span class="rule-category-badge badge-catalog">Catalog</span> <span class="rule-category-badge badge-manifest-fallback">Fallback</span></td>
      <td>Enforce canonical column naming by flagging invalid name patterns. Supports exceptions for allowed variations.</td>
      <td style="font-size: 12px; color: #666;">columns, canonical, naming, standards, consistency</td>
    </tr>
    <tr class="rule-item" data-keywords="columns data types type coverage catalog database schema yml yaml column-level fallback manifest" data-category="documentation">
      <td><a href="columns#rule-columns_have_data_type" class="rule-name">columns_have_data_type</a></td>
      <td><span class="rule-category-badge badge-documentation">Documentation</span> <span class="rule-category-badge badge-catalog">Catalog</span> <span class="rule-category-badge badge-manifest-fallback">Fallback</span></td>
      <td>Check if columns have data types defined. Supports coverage threshold (all or percentage).</td>
      <td style="font-size: 12px; color: #666;">columns, data types, coverage, catalog, database</td>
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

  // Column sorting
  const tables = document.querySelectorAll('.rules-table');
  tables.forEach(table => {
    const headers = table.querySelectorAll('th');
    headers.forEach((header, index) => {
      header.style.cursor = 'pointer';
      header.style.userSelect = 'none';
      header.title = 'Click to sort';
      header.addEventListener('click', function() {
        const tbody = table.querySelector('tbody');
        const rows = Array.from(tbody.querySelectorAll('tr'));
        const ascending = header.dataset.sortDir !== 'asc';
        header.dataset.sortDir = ascending ? 'asc' : 'desc';

        // Reset sort indicators on other headers
        headers.forEach(h => {
          if (h !== header) {
            h.dataset.sortDir = '';
            h.textContent = h.textContent.replace(/ [▲▼]$/, '');
          }
        });

        // Update sort indicator
        header.textContent = header.textContent.replace(/ [▲▼]$/, '') + (ascending ? ' ▲' : ' ▼');

        rows.sort((a, b) => {
          const aText = a.cells[index]?.textContent.trim().toLowerCase() || '';
          const bText = b.cells[index]?.textContent.trim().toLowerCase() || '';
          return ascending ? aText.localeCompare(bText) : bText.localeCompare(aText);
        });

        rows.forEach(row => tbody.appendChild(row));
      });
    });
  });
});
</script>
