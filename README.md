# 🕵️ dbtective

dbtective is a Rust-powered 'detective' for `dbt metadata` best practices. As your dbt project grows, keeping metadata consistent and high-quality can become a real challenge.

**dbtective** makes it easy to spot and fix common issues, examples:

- **Missing descriptions:** Does every model and seed have a description?
- **Column types:** Are all columns explicitly typed?
- **Ownership:** Do all sources have an owner?
- **Naming conventions:** Are all marts following your team's naming standards?

We detect and enforce these rules in your `pre-commit` and `CI/CD` pipeline, so fast you will barely notice🕵️

## Quickstart
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

