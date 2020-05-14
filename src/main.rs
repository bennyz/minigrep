use std::env;
use std::fs;
use std::{process, error::Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Couldn't parse the arguments {}", err);
        process::exit(1)
    });

    println!("Running query {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = run(config) {
        println!("Failed to read content! {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("With text: {}", content);

    // This is like a void function, but it can also return errors!
    Ok(())
}
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config { query, filename });
    }
}