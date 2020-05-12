use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let query = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
    
    let content = fs::read_to_string(filename)
        .expect("Failed to read the file");
    println!("With content: {}", content);
}