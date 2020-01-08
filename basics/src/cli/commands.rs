extern crate ncurses;
extern crate url;

use std::fmt::Error;
use std::process::{Command, Stdio};

use regex::Regex;
use url::{ParseError, Url};

pub fn main() {
    // simple_command("https://github.com/remipassmoilesel/linux-utils".parse().unwrap());
    output_streaming("https://github.com/remipassmoilesel/linux-utils".parse().unwrap());
}

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn simple_command(url: Url) -> Result<(), ()> {
    let output = Command::new("git").arg("clone").arg(url.to_string()).output().unwrap();
    println!("{:?}", output);
    println!("{:?}", output.stdout);
    println!("{:?}", output.stderr);

    if !output.status.success() {
        println!("Command executed with failing error code");
        return Err(());
    }

    Ok(())
}

fn output_streaming(url: Url) {
    let mut cmd =
        Command::new("git")
            .arg("clone")
            .arg(url.to_string())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();

    let status = cmd.wait();
    println!("Exited with status {:?}", status);
}

fn git_log() {
    let output = Command::new("git").arg("log").output().unwrap();

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
}
