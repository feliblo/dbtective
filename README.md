# 🕵️ dbtective

On the case for your dbt best practices.

**dbt-tective** is a **Rust-powered** linter and "detective" for **dbt metadata best practictes**

> ( •*•)>⌐■-■ dbtective  
> (⌐■*■) Case solved!

## What is dbttective?

[dbt](https://www.getdbt.com/) (Data Build Tool) is a powerful framework for building, testing, and documenting data models in your data warehouse. As teams scale, dbt projects accumulate a wealth of metadata: documentation, tests, ownership, access controls, column types, constraints, and more. Managing the consistency and quality of this metadata at scale can become overwhelming.

**dbt-tective** helps teams uncover inconsistencies, enforce best practices, and maintain high-quality metadata across their dbt projects. By programmatically defining and enforcing rules, dbtective makes it easier to keep your dbt project organized, documented, and production-ready.

## Features

- [ ] **Lint dbt metadata**: Analyze your models, sources, and snapshots for documentation, ownership, tests, and other best practices.
- [ ] **Customizable rules**: Define your own rules or use built-in ones to match your team's standards.
- [ ] **Scalable & Fast**: Built in Rust for blazing speed and reliability, even on large dbt projects.
- [ ] **Actionable output**: Get clear, actionable feedback for every dbt resource.
- [ ] **Gamified scoring**: Assigns "maturity scores" to your dbt entities, making metadata improvements trackable and fun.
- [ ] **CI/CD ready**: Integrate with your continuous integration workflows to prevent metadata drift.

## Why dbtective?

As dbt projects grow, ensuring high-quality, consistent metadata becomes a detective job of its own. dbtective shines a light on the mysteries in your dbt project, helping your team spot issues before they become problems.

## Documentation

Full documentation is coming soon!

For now, usage examples, rule configuration, and contributing guidelines will be available in the [docs directory](./docs/) and on the project's website.

## Contributing

We welcome contributions! Whether you're fixing bugs, adding features, or improving documentation, your help makes dbtective better for everyone.

**Quick start:**

```bash
# Debug mode (includes debug logs and timing)
cargo run

# Release mode (optimized, clean output)
cargo run --release

# Run with arguments
cargo run -- --help
```

For detailed contributing guidelines, development setup, and coding standards, please see [CONTRIBUTING.md](./CONTRIBUTING.md).
