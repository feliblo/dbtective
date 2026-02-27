---
title: Init
description: Understanding the layering strategy and data modeling methodology choices during dbtective init
weight: 2
---

When you run `dbtective init`, you'll be asked some questions about your dbt project structure. These choices determine which rules and folder conventions are generated in your configuration file.
{{< callout type="info" >}}
These are just **initialization defaults**. You can always customize, add, or remove any rules after init. The generated config is a starting point, not a constraint!
{{< /callout >}}

## Layering Strategy

The first question asks how your dbt project organizes pipeline stages.

<details>
<summary><strong>Medallion (bronze, silver, gold)</strong></summary>

| Layer      | Purpose                                                                     |
| ---------- | --------------------------------------------------------------------------- |
| **bronze** | Raw ingestion, 1:1 with source. Minimal transformation (casting, renaming). |
| **silver** | Cleansed, deduplicated, conformed. Business logic applied.                  |
| **gold**   | Consumption-ready. Aggregated, modeled for end users.                       |

**Generated rules:**

- `allowed_subfolders` enforcing `["bronze", "silver", "gold"]`
- `has_contract_enforced` scoped to `models/silver` and `models/gold`
- `code_forbidden_patterns` preventing direct schema references like `bronze.`, `silver.`, `gold.`
- `is_not_orphaned` ensuring mart models are referenced by exposures

</details>

<details>
<summary><strong>Common (staging, warehouse, marts)</strong></summary>

| Layer         | Purpose                                                                      |
| ------------- | ---------------------------------------------------------------------------- |
| **staging**   | 1:1 with source. Light transformations (renaming, casting, basic filtering). |
| **warehouse** | Business logic, joins across staging models. Not exposed to end users.       |
| **marts**     | Consumption-ready models. Exposed via BI tools, APIs, etc.                   |

**Generated rules:**

- `allowed_subfolders` enforcing `["staging", "warehouse", "marts"]`
- `has_contract_enforced` scoped to `models/marts` and `models/warehouse`
- `code_forbidden_patterns` preventing direct schema references like `staging.`, `warehouse.`, `marts.`
- `is_not_orphaned` ensuring mart models are referenced by exposures

</details>

<details>
<summary><strong>None</strong></summary>

No folder structure enforcement is applied. Use this if your project doesn't follow a standard layering pattern or if you want to define your own folder rules manually.

</details>

---

## Data Modeling Methodology

The second question asks what data modeling methodology you follow. This determines sub-folder structures within your layers and naming convention rules.

<details>
<summary><strong>Kimball (star schema — dimensions & facts)</strong></summary>

Kimball dimensional modeling organizes data into facts (measurable events) and dimensions (descriptive context). The generated folder structure depends on the layering strategy.

**With Medallion** — dims, facts, and marts all live under the final layer (gold):

```
gold/
├── dimensions/     # dim_customer, dim_product, dim_date
├── facts/          # fact_orders, fact_page_views
└── marts/          # Consumption-ready aggregations, exposed via BI tools
```

**With Common** — dims and facts live in the middle layer (warehouse), marts is the final layer:

```
warehouse/
├── dimensions/     # dim_customer, dim_product, dim_date
└── facts/          # fact_orders, fact_page_views
marts/              # Consumption-ready models, exposed via BI tools
```

**Generated rules:**

- `allowed_subfolders`: `["dimensions", "facts", "marts"]` on the final layer (Medallion) or `["dimensions", "facts"]` on the middle layer (Common)
- `name_convention` with naming-convention-aware prefix scoped to dimensions folder (e.g. `^dim_` for snake_case)
- `name_convention` with naming-convention-aware prefix scoped to facts folder (e.g. `^fact_` for snake_case)
- `is_not_orphaned` scoped to the marts folder (Medallion: `gold/marts`, Common: `marts`)

</details>

<details>
<summary><strong>Inmon (normalized enterprise DWH → departmental marts)</strong></summary>

Inmon methodology uses a normalized (3NF) enterprise data warehouse with departmental data marts derived from it.

**Folder structure generated** (example with Medallion):

```
silver/
├── core/           # Core entity tables (customers, products, orders)
├── reference/      # Reference/lookup tables
└── bridge/         # Bridge/association tables for M:N relationships
gold/
├── finance/        # Departmental data marts (customize these)
├── marketing/
└── operations/
```

**Generated rules:**

- `allowed_subfolders` on middle layer: `["core", "reference", "bridge"]`
- `allowed_subfolders` on final layer: `["finance", "marketing", "operations"]` (customize to your departments)
- `has_contract_enforced` on the middle layer (enterprise model)
- `has_unique_test` on the middle layer (primary key enforcement)

</details>

<details>
<summary><strong>Data Vault (hub-link-satellite)</strong></summary>

Data Vault is the most structured methodology. It organizes data into hubs (business keys), links (relationships), and satellites (descriptive context).

**Folder structure generated** (example with Medallion):

```
silver/                     # Raw Vault
├── hubs/                   # hub_customer, hub_product
├── links/                  # lnk_order_customer
├── satellites/             # sat_customer_details
└── t_links/                # t_lnk_order_line
gold/                       # Business Vault + Information Marts
├── business_vault/         # pit_, bridge_, bsat_ tables
└── information_mart/       # dim_, fact_ tables for consumption
```

**Generated rules:**

- `allowed_subfolders` on middle layer: `["hubs", "links", "satellites", "t_links"]`
- `allowed_subfolders` on final layer: `["business_vault", "information_mart"]`
- `name_convention` rules with naming-convention-aware prefixes: `hub`, `lnk`, `sat`, `t_lnk` scoped to respective folders
- `name_convention` rules with naming-convention-aware prefixes: `dim`, `fact` in the information mart
- `has_contract_enforced` on vault tables
- `has_unique_test` on hubs and links (hash key uniqueness)
- `is_not_orphaned` scoped to the information mart

</details>

<details>
<summary><strong>None</strong></summary>

No methodology-specific rules are generated. Use this if your project doesn't follow a specific data modeling methodology or if you want to define your own rules manually.

</details>

---

## Combination Matrix

The layering strategy and methodology work together. The layering strategy determines the **top-level folder names**, while the methodology determines the **sub-folder structure within those layers**.

| Layering + Methodology  | Middle Layer Sub-folders                 | Final Layer Sub-folders              | Naming Prefixes                                   |
| ----------------------- | ---------------------------------------- | ------------------------------------ | ------------------------------------------------- |
| Any + **None**          | (none)                                   | (none)                               | (none)                                            |
| Medallion + **Kimball** | (none)                                   | `dimensions`, `facts`, `marts`       | `dim_`, `fact_`                                   |
| Common + **Kimball**    | `dimensions`, `facts`                    | (none)                               | `dim_`, `fact_`                                   |
| Any + **Inmon**         | `core`, `reference`, `bridge`            | department names                     | (none)                                            |
| Any + **Data Vault**    | `hubs`, `links`, `satellites`, `t_links` | `business_vault`, `information_mart` | `hub_`, `lnk_`, `sat_`, `t_lnk_`, `dim_`, `fact_` |

{{< callout type="tip" >}}
After running `dbtective init`, review the generated config file and customize any rules to match your project's specific needs. The department names in Inmon (`finance`, `marketing`, `operations`) are examples — replace them with your actual department or domain names.
{{< /callout >}}

{{< callout type="info" >}}
**Naming convention aware**: The methodology naming prefixes (`dim_`, `fact_`, `hub_`, `lnk_`, etc.) are automatically adapted to your chosen naming convention. For example, with PascalCase they become `^Dim[A-Z]`, `^Fact[A-Z]`, `^Hub[A-Z]`, etc. With kebab-case they become `^dim-`, `^fact-`, `^hub-`, etc.
{{< /callout >}}
