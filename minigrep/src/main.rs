extern crate minigrep;

use std::env;
use std::process::exit;

fn main() {
    minigrep::minigrep(env::args().collect()).unwrap_or_else(|err| {
        eprintln!("ERROR: {}", err);
        exit(1);
    })
}
