---
title: Checks
type: docs
prev: docs/docs-main
sidebar:
  open: true
---

Here you can find an overview of all possible checks. Use the filter below to find the checks you need, or search using `ctrl + K`.

<div id="checksContent">

## Manifest Checks

</br>
<div style="margin-bottom: 20px;">
  <input type="text" id="checkFilter" placeholder="Filter checks by name, keywords, or category..." style="width: 100%; padding: 10px; border: 1px solid #ccc; border-radius: 4px; font-size: 14px;" />
</div>

<table class="checks-table">
  <thead>
    <tr>
      <th>Check Name</th>
      <th>Category</th>
      <th>Description</th>
      <th>Keywords</th>
    </tr>
  </thead>
  <tbody>
    <tr class="check-item" data-keywords="description documentation yaml schema describe docs comments metadata" data-category="manifest">
      <td><a href="/docs/checks/description" class="check-name">has_description</a></td>
      <td><span class="check-category-badge badge-manifest">Manifest</span></td>
      <td>Check if a description is populated. Ensures objects have documentation in their schema (e.g. YAML) files.</td>
      <td style="font-size: 12px; color: #666;">description, documentation, yaml, schema</td>
    </tr>
    <tr class="check-item" data-keywords="naming pattern regex standards conventions prefixes suffixes name format" data-category="manifest">
      <td><a href="/docs/checks/naming_conventions" class="check-name">name_convention</a></td>
      <td><span class="check-category-badge badge-manifest">Manifest</span></td>
      <td>Check if object names follow casing (e.g.<code>snake_case</code>) or custom regex patterns. Enforces naming standards using configurable patterns.</td>    <td style="font-size: 12px; color: #666;">naming, pattern, regex, conventions, prefixes</td>
    </tr>
    <tr class="check-item" data-keywords="tags metadata categorization organization labels tagging" data-category="manifest">
      <td><a href="/docs/checks/tags" class="check-name">has_tags</a></td>
      <td><span class="check-category-badge badge-manifest">Manifest</span></td>
      <td>Check if objects have the required tags. Ensure proper categorization for selective execution.</td>
      <td style="font-size: 12px; color: #666;">tags, metadata, categorization, organization</td>
    </tr>
    <tr class="check-item" data-keywords="orphaned unused references dependencies lineage data assets cleanup" data-category="manifest">
      <td><a href="/docs/checks/is_not_orphaned" class="check-name">is_not_orphaned</a></td>
      <td><span class="check-category-badge badge-manifest">Manifest</span></td>
      <td>Check if objects are referenced by other objects. Identifies orphaned data assets that may be unused or underutilized.</td>
      <td style="font-size: 12px; color: #666;">orphaned, unused, references, dependencies, lineage</td>
    </tr>
    <tr class="check-item" data-keywords="tests uniqueness unique validation custom" data-category="manifest">
      <td><a href="/docs/checks/has_unique_test" class="check-name">has_unique_test</a></td>
      <td><span class="check-category-badge badge-manifest">Manifest</span></td>
      <td>Check if dbt objects have at least one uniqueness test attached. Supports standard and custom uniqueness tests.</td>
      <td style="font-size: 12px; color: #666;">tests, uniqueness, unique, validation, custom</td>
    </tr>
  </tbody>
</table>

## Catalog Checks

<details>
  <summary><strong>Why differentiate between <code>manifest</code> and <code>catalog</code>?</strong></summary>

  <p>
    These checks use both the <code>manifest.json</code> and <code>catalog.json</code> artifacts. These files can become out of sync during development (for example, when running <code>dbtective</code> in pre-commit hooks), especially if files are moved or renamed and only one of the commands generating <code>manifest.json</code> is run. For more information, see the <a href="https://docs.getdbt.com/reference/artifacts/manifest-json" target="_blank">dbt documentation on manifest.json</a>.
  </p>
  <p>
    To ensure your catalog is up to date, delete it from the dbt target folder and regenerate it using <code>dbt docs generate</code>. Future updates to dbtective will include an option to automate this process with a specific flag.
  </p>
</details>

<table class="checks-table">
  <thead>
    <tr>
      <th>Check Name</th>
      <th>Category</th>
      <th>Description</th>
      <th>Keywords</th>
    </tr>
  </thead>
  <tbody>
    <tr class="check-item" data-keywords="columns catalog database schema yml yaml documentation undocumented missing docs column-level" data-category="catalog">
      <td><a href="/docs/checks/documentation" class="check-name">columns_all_documented</a></td>
      <td><span class="check-category-badge badge-catalog">Catalog</span> </td>
      <td>Check if all SQL columns are documented in e.g. their yml file. Validates that database columns match documentation.</td>
      <td style="font-size: 12px; color: #666;">columns, catalog, database, schema, undocumented</td>
    </tr>
  </tbody>
</table>

</div>

<script>
document.addEventListener('DOMContentLoaded', function() {
  const filterInput = document.getElementById('checkFilter');
  const checksContent = document.getElementById('checksContent');

  if (filterInput && checksContent) {
    filterInput.addEventListener('input', function(e) {
      const filterValue = e.target.value.toLowerCase().trim();
      const checkItems = checksContent.querySelectorAll('.check-item');
      const tables = checksContent.querySelectorAll('.checks-table');

      if (filterValue === '') {
        // Show all
        checkItems.forEach(item => item.style.display = '');
        tables.forEach(table => table.style.display = '');
        checksContent.querySelectorAll('h2').forEach(heading => heading.style.display = '');
        return;
      }

      // Filter rows
      checkItems.forEach(item => {
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
        const visibleRows = table.querySelectorAll('tbody .check-item:not([style*="display: none"])');
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
