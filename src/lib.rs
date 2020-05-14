use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("With text: {}", content);

    // This is like a void function, but it can also return errors!
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        return Ok(Config { query, filename });
    }
}