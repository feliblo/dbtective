# 🕵️ dbtective

dbtective is a Rust-powered 'detective' for `dbt metadata` best practices. As your dbt project grows, keeping metadata consistent and high-quality can become a real challenge.

Explore the [full documentation](https://feliblo.github.io/dbtective/).

> [!WARNING]
> dbtective is currently in very early stages.
> Issues and commits are welcome, but don't rely on us yet!

**dbtective** makes it easy to spot and fix common issues, examples:

- **Missing descriptions:** Does every model and seed have a description?
- **Column types:** Are all columns explicitly typed?
- **Ownership:** Do all sources have an owner?
- **Naming conventions:** Are all marts following your team's naming standards?

We detect and enforce these rules in your `pre-commit` and `CI/CD` pipeline, so fast you will barely notice🕵️

## Installation

### Shell installer (macOS/Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/feliblo/dbtective/releases/latest/download/dbtective-installer.sh | sh
```

### PowerShell installer (Windows)

```powershell
irm https://github.com/feliblo/dbtective/releases/latest/download/dbtective-installer.ps1 | iex
```

### Cargo

```bash
cargo install dbtective
```

### Download binaries

Pre-built binaries for Linux, macOS, and Windows are available on the [releases page](https://github.com/feliblo/dbtective/releases).

## Quickstart

All possible checks can be found in the [checks documentation](https://feliblo.github.io/dbtective/docs/). Information about configuring `dbtective` is shown at the [config documentation](https://feliblo.github.io/dbtective/docs/config)

### Example

todo!()

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
