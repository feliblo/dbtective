---
title: Quickstart
weight: 1
toc: false
---


All possible rules can be found in the [rules documentation](/docs/rules). Information about customizing `dbtective` is shown at the [config documentation](/docs/config)

1. Generate your config file by answering a few simple questions:

    ```bash
    dbtective init
    ```

    This walks you through picking a config format, naming convention, data model, strictness level, and which rules to enable — then generates the config for you. See the [CLI reference](/docs/running/cli#init) for details.

2. (Optional) Generate the dbt manifest and catalog files if you haven't done so already. Most dbt commands automatically generate the `manifest.json`, but if you want to ensure both files are up to date, run:

    ```bash
    dbt compile
    dbt docs generate
    ```

3. Run `dbtective` in the root of your current directory or specify an entry point if your dbt_project is not located in the root/current directory.

    ```bash
    dbtective run
    dbtective run --entry-point "my_dbt_project"
    ```

4. Review the output and fix any issues found.

5. (Optional) Integrate `dbtective` into your CI/CD pipeline or pre-commit hooks to automate rules on every commit and/or pull request.
