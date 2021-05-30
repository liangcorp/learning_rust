extern crate minigrep;
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
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
