use owo_colors::OwoColorize;
use std::process::exit;

pub fn unwrap_or_exit<T>(result: anyhow::Result<T>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{}", err.to_string().red());
            exit(1);
        }
    }
}

pub fn print_catalog_warning() {
    println!(
        "\n{}{}",
        "One or more catalog tests failed. ".yellow().bold(),
        "This may be due to a stale catalog (local development, pre-commit).".yellow()
    );

    println!(
        "  {}",
        "Use --only-manifest everywhere except for CI/CD (recommended), or regenerate with `dbt docs generate`"
            .cyan()
            .dimmed()
    );

    println!(
        "  {}",
        "See: https://feliblo.github.io/dbtective/docs/running/precommit".dimmed()
    );
}
