pub(crate) fn handle_config_error(err: serde_yaml::Error) -> anyhow::Error {
    eprintln!("Configuration parsing error: {}", err);
    std::process::exit(1);
}
