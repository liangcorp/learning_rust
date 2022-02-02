extern crate minigrep;
use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem in Parsing: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    minigrep::run(config).unwrap_or_else(|err| {
        println!("Application Error: {}", err);
        process::exit(1);
    });
}
