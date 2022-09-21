use std::{env, process};

use minigrep::Config;

fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    println!("Searching `{}` in {}", config.query, config.filename);

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application Error: {}", err);
        process::exit(1);
    }
}
