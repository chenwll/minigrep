// in main.rs
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let content = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // cargo run -- test sample.txt
    // 在sample.txt寻找test字符
    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config {
            query,
            file_path,
        }
    }
}