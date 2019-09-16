use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    println!("{:?}", &args[1..]);
}