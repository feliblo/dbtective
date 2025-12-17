# 🕵️ dbtective

dbtective is a Rust-powered 'detective' for `dbt metadata` best practices. As your dbt project grows, keeping metadata consistent and high-quality can become a real challenge.

![CLI demo](/docs/static/demo.gif)

Explore the [full documentation](https://feliblo.github.io/dbtective/docs) or the [possible rules](https://feliblo.github.io/dbtective/docs/rules).

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
<summary>Homebrew</summary>

```bash
brew install feliblo/tap/dbtective
```

</details>

<details>
<summary>GitHub Actions</summary>

Run dbtective as part of your CI/CD pipeline. See the [GitHub Actions documentation](https://feliblo.github.io/dbtective/docs/github-actions) for more details.

```yaml
- uses: feliblo/dbtective@v0.1.28
  with:
    config-file: "dbtective.yml"
    entry-point: "."
    only-manifest: "true"
    verbose: "false"
```

</details>

<details>
<summary>prek/pre-commit</summary>

Prerequisite: `dbtective` is installed via one of the methods above.

We (currently) recommend using the `--only-manifest` flag with prek/pre-commit to avoid issues caused by `catalog.json` mismatches. For more details, see the explanation in the [rules documentation](https://feliblo.github.io/dbtective/docs/).

Add the following to your `.pre-commit-config.yaml`.

```yaml
repos:
  - repo: https://github.com/feliblo/dbtective
    rev: v0.1.28
    hooks:
      - id: dbtective-run
        args: [--only-manifest]
```

And run

```bash
prek install
prek run --all-files
# or with pre-commit
pre-commit install
pre-commit run --all-files

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
<summary>Binary download</summary>

Pre-built binaries for Linux, macOS, and Windows are available on the [releases page](https://github.com/feliblo/dbtective/releases).
</details>

## Quickstart

All possible rules can be found in the [rules documentation](https://feliblo.github.io/dbtective/docs/). Information about customizing `dbtective` is shown at the [config documentation](https://feliblo.github.io/dbtective/docs/config)

1. Create a `dbtective.yml` config file in the root of your dbt project by running:

More about the `init` (it also supports `pyproject.toml` or `dbtective.toml`) command is available in the [init documentation](https://feliblo.github.io/dbtective/docs/cli#dbtective-init).

```bash
dbtective init
```

Inspect the contents of the generated `dbtective.yml` file and modify it to fit your project's needs.

2. (Optional) Generate the dbt manifest and catalog files if you haven't done so already. Most dbt commands automatically generate the `manifest.json`, but if you want to ensure both files are up to date, run:

```bash
dbt compile
dbt docs generate
```

3. Run `dbtective` in the root of your current directory or specify an entry point if your dbt_project is not located in the root/current drectory.

```bash
dbtective run
dbtective run --entry-point "my_dbt_project"
```

4. Review the output and fix any issues found.

5. (Optional) Integrate `dbtective` into your CI/CD pipeline or pre-commit hooks to automate rules on every commit and/or pull request.

## Contributing

We welcome contributions! Whether you're fixing bugs, adding features, or improving documentation, your help makes dbtective better for everyone.

For detailed contributing guidelines, development setup, and coding standards, please see the [contributing documentation](https://feliblo.github.io/dbtective/docs/contributing).

# Acknowledgements

This project is heavily inspired dbt [dbt-bouncer](https://github.com/godatadriven/dbt-bouncer). It tries to improve certain aspects of the amazing work by [pgoslatara](https://github.com/pgoslatara), while giving me an opportunity to improve my Rust. More about the aspects we try to improve is available in our  [FAQ](https://feliblo.github.io/dbtective/docs/faq).
