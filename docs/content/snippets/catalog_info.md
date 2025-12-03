<details>
  <summary>Why differentiate between <code>manifest</code> and <code>catalog</code> checks?</summary>

  <p>
    Catalog checks use both the <code>manifest.json</code> and <code>catalog.json</code> artifacts. Catalog checks require an <b>active database connection</b> to be generated using <a href="https://docs.getdbt.com/reference/commands/cmd-docs" target="_blank"><code>dbt docs generate</code></a>. These files can become out of sync during development (for example, when running <code>dbtective</code> in pre-commit hooks), especially if files are moved or renamed and only one of the commands generating <code>manifest.json</code> is run. For more information, see the <a href="https://docs.getdbt.com/reference/artifacts/manifest-json" target="_blank">dbt documentation on manifest.json</a>.
  </p>
  <p>
    To ensure your catalog is up to date, delete it from the dbt target folder and regenerate it using <code>dbt docs generate</code>. Future updates to dbtective will include an option to automate this process with a specific flag. It's also possible to disable `catalog` based checks using the `--only-manifest` flag.
  </p>
</details>
