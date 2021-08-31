use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments: {}", _err);
        process::exit(1);
    });

    if let Err(_e) = minigrep::run(config) {
        eprintln!("Application error: {}", _e);

        process::exit(1);
    }
}
