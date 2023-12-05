// in main.rs
use minigrep::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // cargo run -- test sample.txt
    // 在sample.txt寻找test字符
    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process.exit(1);
    }
}

