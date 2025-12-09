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
