use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = parse_config(&args);

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("With text:\n {}", contents)
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].to_string();
    let file_path = args[2].to_string();
    Config { query, file_path }
}
