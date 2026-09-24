# Documentation guide

The docs are a Hugo site on the [Hextra](https://imfing.github.io/hextra/) theme, published to https://feliblo.github.io/dbtective/ by `.github/workflows/pages.yml` on release.
Only `docs/content/` is rendered. Read the root [AGENTS.md](../AGENTS.md) first.

Preview locally with `just setup-docs` (once) and then `just docs` (needs Hugo extended + Go). If you can't run Hugo, check your shortcodes and HTML carefully against an existing page.

## Every rule change updates two places

1. **The rule page**: `docs/content/docs/rules/<rule>.md`, or a new `### Rule:` section in the family page.
2. **The rules table**: a `<tr>` in `docs/content/docs/rules/_index.md`, under **Manifest Rules** or **Catalog Rules**.

This check lists rules that are missing from the table or have no page. It should print nothing:

```bash
diff <(for f in src/core/config/manifest_rule.rs src/core/config/catalog_rule.rs; do
         awk '/^pub enum (Manifest|Catalog)SpecificRuleConfig/{on=1;next} on&&/^}/{on=0} on&&/^    [A-Z][A-Za-z]* \{/{print $1}' "$f"
       done | sed -E 's/([a-z0-9])([A-Z])/\1_\2/g' | tr '[:upper:]' '[:lower:]' | sort -u) \
     <(grep -o 'class="rule-name">[a-z_]*' docs/content/docs/rules/_index.md | sed 's/.*>//' | sort -u)
# same again with the pages:
#   <(grep -rho '### Rule: `[a-z_]*`' docs/content/docs/rules | sed 's/.*`\(.*\)`/\1/' | sort -u)
# and unbalanced <details> tags (today this prints the three known-broken pages named under "Rule page"):
for f in docs/content/docs/rules/*.md; do [ "$(grep -c '<details' $f)" = "$(grep -c '</details>' $f)" ] || echo "$f"; done
```

### Rule page

Copy an existing page and keep its structure (front matter, `### Rule:` heading, badge, `<details open>` wrapper, **Configuration** list, the `common_rule_config.md` include, three config tabs, and a closed "Relevant dbt code" `<details>`):
- no options: [rules/has_loader.md](content/docs/rules/has_loader.md)
- with options: [rules/node_dependency.md](content/docs/rules/node_dependency.md)

Don't copy `allowed_subfolders.md`, `has_contract_enforced.md` or `is_not_orphaned.md`: they're missing the closing `</details>` of the outer wrapper.

Rules for the page:
- **Default/Options must copy the code exactly**: `default_applies_to_for_*_rule` and `applies_to_options_for_*_rule` in `src/core/config/{manifest,catalog}_rule.rs`, and the option defaults in `check_config_options.rs`.
- Provide **all three tabs** (`dbtective.yml`, `dbtective.toml`, `pyproject.toml`), and keep the examples identical in meaning. Catalog rules use `catalog_tests` / `[[catalog_tests]]` / `[[tool.dbtective.catalog_tests]]`.
- Badges (classes from `docs/assets/css/custom.css`; `docs/public/` is gitignored build output): `badge-manifest` "Manifest Rule"; `badge-catalog` "Catalog Rule" followed by `{{< include-markdown "content/snippets/catalog_info.md" >}}`; `badge-manifest-fallback` "Fallback" for catalog rules where `supports_manifest_fallback()` is `true`.
- Always include `content/snippets/common_rule_config.md`. Don't re-document `name`, `severity`, `description`, `category`, `includes`, `excludes` or `model_materializations`.
- Link related rules, e.g. `[has_refs](../has_refs)`.

**Family page** (several rules on one page, e.g. `code.md`, `columns.md`, `fan-in-fan-out.md`, `materialization_lineage.md`, `test_configuration.md`): add a new `### Rule: \`<rule_name>\`` section separated by `<hr style="border: 1px solid #666; margin: 2em 0;">`, and **bump the count in the front-matter title** (`title: code (5)` → `code (6)`). Hugo turns the heading into the anchor `#rule-<rule_name>`, which the table links to.

### Rules table row (`rules/_index.md`)

```html
<tr class="rule-item" data-keywords="space separated search terms synonyms" data-category="governance">
  <td><a href="<page>" class="rule-name"><rule_name></a></td>
  <td><span class="rule-category-badge badge-governance">Governance</span></td>
  <td>One-sentence description.</td>
  <td style="font-size: 12px; color: #666;">5, comma, separated, visible, keywords</td>
</tr>
```

- `href`: the page file name without `.md` (`has_loader`), or `<family>#rule-<rule_name>` for family pages (`code#rule-code_max_joins`).
- `data-category` and the badge must match `default_category()` in code (lowercase class `badge-<category>`, capitalised label).
- Catalog rows add `<span class="rule-category-badge badge-catalog">Catalog</span>` and, when the rule supports fallback, `<span class="rule-category-badge badge-manifest-fallback">Fallback</span>` in the category cell.
- `data-keywords` feed the filter box, so make them relevant to *this* rule.
- Put the row next to rules of the same category. Don't touch the `<script>` at the bottom.

## Other pages to check when behaviour changes

| Change                                                   | Update                                                                  |
| -------------------------------------------------------- | ----------------------------------------------------------------------- |
| New or renamed CLI flag                                  | `running/cli.md`; `action.yml` inputs + `running/github-actions.md` if the Action should expose it |
| Config file format / top-level keys / `config:` settings | `config.md`                                                             |
| includes/excludes syntax                                 | `includes_excludes.md`                                                  |
| `dbtective init` output (presets, layering, methodology) | `running/init.md`                                                       |
| Catalog fallback behaviour                               | `running/manifest-only.md` + Fallback badges                            |
| JSON vs Parquet handling                                 | `running/artifact-formats.md`                                           |
| Install / quickstart / supported dbt versions            | `README.md` + `quickstart.md` (`CONTRIBUTING.md` mirrors `contributing/_index.md`) |

## Don'ts

- Don't change version numbers (`rev: vX.Y.Z`, `feliblo/dbtective@vX.Y.Z`). `scripts/update_*.sh` rewrite them on release.
- Don't edit `CHANGELOG.md`.
- Don't hand-write front-matter `weight` on rule pages; the sidebar orders them itself. Other sections use `weight` deliberately, so keep the existing values.
- Don't write docs for behaviour that doesn't exist. Check the code, especially defaults.
