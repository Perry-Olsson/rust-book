use std::{env, process};
use minigrep::{Config, EnvVars};

fn main() {
    let config = Config::build(env::args(), EnvVars::new()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

