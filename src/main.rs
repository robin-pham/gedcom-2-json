use gedcom_parser::Config;
use std::env;
use std::process;
// extern crate regex;
// extern crate serde;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = gedcom_parser::run(config) {
        println!("Parsing error: {}", e);
        process::exit(1);
    }
}
