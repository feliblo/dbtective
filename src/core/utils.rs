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

pub fn print_catalog_warning(fallback_rules: &[String], skipped_rules: &[String]) {
    let is_fallback = !fallback_rules.is_empty() || !skipped_rules.is_empty();

    if is_fallback {
        println!(
            "\n{}{}",
            "Catalog tests ran in manifest-only mode. ".yellow().bold(),
            "Results may not represent the actual database state.".yellow()
        );

        if !fallback_rules.is_empty() {
            println!(
                "  {} {}",
                "Test types that ran with manifest fallback:".dimmed(),
                fallback_rules.join(", ").dimmed()
            );
        }

        if !skipped_rules.is_empty() {
            println!(
                "  {} {}",
                "Skipped types (requires catalog.json):".dimmed(),
                skipped_rules.join(", ").dimmed()
            );
        }

        println!(
            "  {}",
            "Provide a catalog.json (remove --only-manifest) for full accuracy."
                .cyan()
                .dimmed()
        );
    } else {
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
    }

    println!(
        "  {}",
        "See: https://feliblo.github.io/dbtective/docs/running/manifest-only".dimmed()
    );
}
