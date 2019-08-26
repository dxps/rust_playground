use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
}

struct Config {
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename} // this Config instance contains its own values
}
