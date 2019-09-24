use std::env;

use argument_parser::ArgumentParser;
use argument_parser::CliCommand;

mod argument_parser;

fn banner() {
    println!(
        "
┏┳┓┏━╸┏┳┓┏━┓
┃┃┃┣╸ ┃┃┃┃ ┃
╹ ╹┗━╸╹ ╹┗━┛
"
    )
}

pub fn main() {
    banner();
    let command = ArgumentParser::parse(env::args().collect());
    match command {
        CliCommand::AddMemo { memo, description } => println!("add: {} {}", memo, description),
        CliCommand::SearchMemo { query } => println!("search: {}", query),
        CliCommand::InvalidCommand { message } => println!("Invalid command: {}", message),
    }
}
