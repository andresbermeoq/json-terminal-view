use std::env;
use std::process;

use json_viewer::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Failed In the Arguments Parser: {err}");
        process::exit(1);
    });

    println!("Searching for {0}", config.query);
    if let Err(e) = json_viewer::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    };
}
