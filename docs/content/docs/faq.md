---
title: FAQ
weight: 4
toc: false
---
{{% details title="What is dbtective?" closed="true" %}}

`dbtective` is a fast, lightweight linter for dbt projects written in Rust. It helps enforce best practices and coding standards across your dbt codebase by validating your project against configurable rules.

{{% /details %}}

{{% details title="How does dbtective compare to similar projects?" closed="true" %}}

Similar projects (as far as I know) include:

- [dbt-score](https://github.com/PicnicSupermarket/dbt-score) - Scores entire dbt projects based on best practices
- [dbt-bouncer](https://github.com/godatadriven/dbt-bouncer) - A configurable linter for dbt projects

`dbtective` is inspired by `dbt-bouncer` but aims to improve on it in several key areas:

| Feature | dbtective |
|---------|-----------|
| **Performance** | Written in Rust for smaller binaries and faster execution (in all environments) |
| **Quick start** | `init` command generates a configuration template that works for most users |
| **Configuration** | Batteries-included sensible defaults with optional fine-grained customization |
| **Cross-object rules** | Single rules apply across multiple object types (e.g., one `has_description` rule for models, sources, macros etc.) |
| **Documentation** | Searchable rules with examples for fixing violations |
| **User experience** | Formatted output with clickable hyperlinks to problematic files |

{{% /details %}}

{{% details title="How do I get started?" closed="true" %}}

1. Install `dbtective`
2. Run `dbtective init` in your dbt project root to generate a configuration file
3. Run `dbtective` to lint your project

See the [Getting Started](/docs) guide for detailed instructions.

{{% /details %}}

{{% details title="Can I use dbtective in CI/CD?" closed="true" %}}

Yes! `dbtective` is designed to work in CI/CD pipelines. It returns a non-zero exit code when rule violations are found, making it easy to fail builds on linting errors. For example take a look at [GitHub Actions](/docs/running/github-actions)

{{% /details %}}

{{% details title="How do I disable or configure a specific rule?" closed="true" %}}

You can configure rules in your `dbtective.toml` (or equivalent) configuration file. Each rule can be enabled, disabled, or customized with specific parameters. See the [Rules](/docs/checks) documentation for available options.

{{% /details %}}
