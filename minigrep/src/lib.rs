use std::env;

use config::Config;
use file_reader::read_file;
use std::process::exit;

mod config;
mod file_reader;

pub fn minigrep() {
    let command_config = Config::new(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("Usage error: {}", err);
        exit(1);
    });
    let file_content = read_file(&command_config.file_path).unwrap_or_else(|err| {
        eprintln!("File error: {}", err);
        exit(1);
    });
    println!("Searching: {}", &command_config.query);
    println!("In file: {}", &command_config.file_path);
}
