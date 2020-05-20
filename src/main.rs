use std::env;
use std::fs;
use std::process;

fn main() {
    // Cargo argument logic
    let args: Vec<String> = env::args().collect();
    // config Result unwrapping with closure to handle Err
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // File operation
    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading file");
    
    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments supplied");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { query, filename })
    }
}
