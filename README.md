# 🕵️ dbtective

dbtective is a Rust-powered 'detective' for `dbt metadata` best practices. As your dbt project grows, keeping metadata consistent and high-quality can become a real challenge.

Explore the [full documentation](https://feliblo.github.io/dbtective/docs) or the [possible checks](https://feliblo.github.io/dbtective/docs/checks).

> [!WARNING]
> dbtective is currently in very early stages.
> Issues and commits are welcome, but don't rely on us yet!

**dbtective** makes it easy to spot and fix common issues, examples:

- **Missing descriptions:** Does every model and seed have a description?
- **Column types:** Are all columns explicitly typed?
- **Ownership:** Do all sources have an owner?
- **Naming conventions:** Are all marts following your team's naming standards?

We detect and enforce these rules in your `cli`, `prek`/`pre-commit` and `CI/CD` pipeline, so fast you will barely notice🕵️.

## Installation

<details>
<summary>Pip (pypi)</summary>

```bash
pip install dbtective
```

</details>

<details>
<summary> uv </summary>

Install as a dev dependency:

```bash
uv add dbtective --dev
```

</details>

<details>
<summary>Shell installer (macOS/Linux)</summary>

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/feliblo/dbtective/releases/latest/download/dbtective-installer.sh | sh
```

</details>

<details>
<summary>PowerShell installer (Windows)</summary>

```powershell
irm https://github.com/feliblo/dbtective/releases/latest/download/dbtective-installer.ps1 | iex
```

</details>

<details>
<summary>Homebrew</summary>

```bash
brew install feliblo/tap/dbtective
```

</details>

<details>
<summary>Binary download</summary>

Pre-built binaries for Linux, macOS, and Windows are available on the [releases page](https://github.com/feliblo/dbtective/releases).
</details>

## Quickstart

All possible checks can be found in the [checks documentation](https://feliblo.github.io/dbtective/docs/). Information about configuring `dbtective` is shown at the [config documentation](https://feliblo.github.io/dbtective/docs/config)

### Example

1. Create a `dbtective.yml` config file in the root of your dbt project. For example:

```yaml
manifest_tests:
  - name: "models_must_have_description"
    type: "has_description"
    severity: "error"
    applies_to: ["models", "sources"]
    description: " models and sources must have a description."

  - name: "naming_convention"
    type: "name_convention"
    description: "Everything must follow the naming convention."
    pattern: "snake_case"

 - name: "all_marts_must_be_tagged"
    type: "has_tags"
    severity: "warning"
    applies_to: ["models"]
    tags:
      - "mart"
    description: "All marts must be tagged with 'mart'"
```

2. Run `dbtective` in the root of your current directory or specify an entry point if your dbt_project is not located in the root/current drectory.

```bash
dbtective run
dbtective run --entry-point "my_dbt_project"
```

## Contributing

We welcome contributions! Whether you're fixing bugs, adding features, or improving documentation, your help makes dbtective better for everyone.

**Quick start:**
Install [just](https://github.com/casey/just) command line runner & take a look at the commands in the justfile.

To build and run on the example project (`./dbt_project` using config `./dbt_project/dbtective.yml`) use:

```bash
just run
just run-verbose
```

For detailed contributing guidelines, development setup, and coding standards, please see [CONTRIBUTING.md](./CONTRIBUTING.md).
