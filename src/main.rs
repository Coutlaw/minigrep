use std::env;
use std::fs;

fn main() {
    // Cargo argument logic
    let args: Vec<String> = env::args().collect();

    let query: &str = &args[1];
    let filename: &str = &args[2];

    println!("Searching for: {}", query);
    println!("In file: {}", filename);

    // File operation
    let contents = fs::read_to_string(filename).expect("Something went wrong reading file");
    
    println!("With text:\n{}", contents);
}
