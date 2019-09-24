
use std::error::Error;


use config::Config;
use file_reader::read_file;

mod config;
mod file_reader;
mod search;

pub fn minigrep(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let command_config = Config::new(args)?;
    let _file_content = read_file(&command_config.file_path)?;

    println!("Searching: {}", &command_config.query);
    println!("In file: {}", &command_config.file_path);

    Ok(())
}
