use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}
