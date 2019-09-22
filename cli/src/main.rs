extern crate clap;

use std::process::exit;

use clap::{App, Arg, SubCommand};

fn error_for_invalid_command() {
    println!("Invalid command, try: $ memo --help");
    exit(1)
}

fn main() {
    let matches = App::new("Memo !")
        .version("0.0.1")
        .author("Rémi Passmoilesel<r.passmoilesel@protonmail.com")
        .about("Store CLI commands !")
        .subcommand(SubCommand::with_name("add").about("add a memo"))
        .subcommand(SubCommand::with_name("search").about("search a memo"))
        .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_m)) => println!("add {:?}", sub_m),
        ("search", Some(sub_m)) => println!("search {:?}", sub_m),
        _ => error_for_invalid_command(),
    }
}
