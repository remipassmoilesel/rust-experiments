use std::fs::File;
use std::io::ErrorKind as FileErrorKind;

pub fn main(){

    // For unrecoverable, will clean memory and display an error message
    // panic!("Panic message");

    // For recoverable errors: Results
    #[derive(Debug)]
    enum UselessErrorKind {
        NumberTooHigh,
        NumberTooLow
    }

    fn do_something(x: &i32) -> Result<i32, UselessErrorKind>{
        match x {
            x if x < &10 => Err(UselessErrorKind::NumberTooLow),
            x if x > &10 && x < &100 => Ok(x * 25),
            _ => Err(UselessErrorKind::NumberTooHigh),
        }
    }

    println!("{:?}", do_something(&8));
    println!("{:?}", do_something(&70));
    println!("{:?}", do_something(&140));

    // Open a file

    let f = File::open("not-found.txt");
    println!("{:?}", f);

    // Match on result

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

    // Unwrap: get value or panic!

    let f = File::open("Cargo.toml").unwrap();
    let f = File::open("Cargo.toml").expect("Panic message"); // same as unwrap but with message

}