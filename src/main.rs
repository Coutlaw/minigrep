use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // config Result unwrapping with closure to handle Err
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run the src/lib.rs to open the config and run the program
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
