use std::env;

fn banner() {
    println!(
        "
    ┏┳┓┏━╸┏┳┓┏━┓
    ┃┃┃┣╸ ┃┃┃┃ ┃
    ╹ ╹┗━╸╹ ╹┗━┛
"
    )
}

fn parse_arguments(args: Vec<String>) {
    println!("{:?}", args);
}

pub fn main() {
    banner();
    parse_arguments(env::args().collect());
}
