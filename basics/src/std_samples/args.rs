use std::env;

pub fn main() {
    // Command line arguments

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("{:?}", &args[1..]);
}
