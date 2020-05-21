use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Cargo argument logic
    let args: Vec<String> = env::args().collect();
    // config Result unwrapping with closure to handle Err
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run the src/lib.rs to open the config and run the program
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
