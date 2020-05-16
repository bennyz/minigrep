use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Couldn't parse the arguments {}", err);
        process::exit(1)
    });

    println!("Running query '{}'", config.query);
    println!("In file: '{}'", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Failed to read content! {}", e);
        process::exit(1);
    }
}
