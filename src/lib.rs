use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    
    let result = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in result {
        println!("{}", line);
    }

    // This is like a void function, but it can also return errors!
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // skip the filename
        let query = match args.next() {
            Some(v) => v,
            None => return Err("No query provided"),
        };

        let filename = match args.next() {
            Some(v) => v,
            None => return Err("No filename provided"),
        };

        let case_sensitive  = env::var("CASE_INSENSITIVE").is_err();
        return Ok(Config { 
            query,
            filename,
            case_sensitive, 
        });
    }
}

pub fn search<'a>(query: &'a str, content:  &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect::<Vec<&str>>()
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // query is now a String, not a slice
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, proDuctive.
Pick three.";

        assert_eq!(vec!["safe, fast, proDuctive."], search_case_insensitive(query, content));
    }
}