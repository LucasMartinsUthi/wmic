use std::env;
use std::process;

use wmic::{Config};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = wmic::run(config) {
        println!("Problem TLS: {}", e);
        process::exit(1);
    };
}


