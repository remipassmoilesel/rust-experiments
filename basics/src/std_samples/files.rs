use std::fs::File;
use std::io::ErrorKind as FileErrorKind;

pub fn main() {
    let f = File::open("not-found.txt");
    println!("{:?}", f);

    match f {
        Ok(file) => println!("File found: \n\n{:?}", file),
        Err(error) => match error.kind() {
            FileErrorKind::NotFound => eprintln!("File not found: {:?}", error.kind()),
            _ => eprintln!("Unknown error: {:?}", error)
        }
    }
}