extern crate ncurses;
extern crate url;

use std::fmt::Error;
use std::process::Command;
use regex::Regex;
use url::{Url, ParseError};

fn main() {
    git_clone("https://github.com/remipassmoilesel/linux-utils".parse().unwrap());
}

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn git_clone(url: Url) -> Result<(), ()> {
    let output = Command::new("git").arg("clone").arg(url.to_string()).output().unwrap();
    let output = Command::new("git").arg("log").arg(url.to_string()).output().unwrap();

    if !output.status.success() {
        println!("Command executed with failing error code");
        return Err(());
    }

    let pattern = Regex::new(r"(?x)
                               ([0-9a-fA-F]+) # commit hash
                               (.*)           # The commit message").unwrap();

    String::from_utf8(output.stdout).unwrap()
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
            Commit {
                hash: cap[1].to_string(),
                message: cap[2].trim().to_string(),
            }
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}
