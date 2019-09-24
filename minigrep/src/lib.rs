use std::error::Error;

use config::Config;
use file_reader::read_file;
use search::search;

mod config;
mod file_reader;
mod search;

pub fn minigrep(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let command_config = Config::new(args)?;
    let file_content = read_file(&command_config.file_path)?;

    println!("Searching: {}", &command_config.query);
    println!("In file: {}\n", &command_config.file_path);

    let search_result = search(&command_config.query, &file_content);

    match search_result {
        ref x if x.len() > 0 => x.iter().for_each(|line| println!("L{}: {}", line.number, line.content)),
        _ => println!("Nothing found !")
    }

    Ok(())
}
