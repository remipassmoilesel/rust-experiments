extern crate ncurses;
extern crate regex;

use ncurses::*;
use regex::Regex;
use std::char;
use std::process::Command;

pub fn main() {
    let ch = getch();

    if ch == KEY_F2 {}
}
