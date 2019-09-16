use std::fs::File;
use std::io::ErrorKind as FileErrorKind;

pub fn main() {

    // Open a file

    let f = File::open("not-found.txt");
    println!("{:?}", f);

    match f {
        Ok(file) => println!("File found: \n\n{:?}", file),
        Err(error) => match error.kind() {
            FileErrorKind::NotFound => eprintln!("File not found: {:?}", error.kind()),
            _ => eprintln!("Unknown error: {:?}", error)
        }
    }

    // More concise way to do the same thing

    let file = "Cargo.toml";
    let file = File::open(file).unwrap_or_else(|error| {
        if error.kind() == FileErrorKind::NotFound {
            File::create(file).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("File: {:?}", file);
}