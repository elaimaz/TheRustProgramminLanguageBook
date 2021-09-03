use std::env;
use std::process;

use improved_io::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments: {}", _err);
        process::exit(1);
    });

    if let Err(_e) = improved_io::run(config) {
        eprintln!("Application error: {}", _e);

        process::exit(1);
    }
}
