use std::fs::File;
use std::io::ErrorKind as FileErrorKind;

pub fn main() {
    // For unrecoverable, will clean memory and display an error message
    // panic!("Panic message");

    // For recoverable errors: Results
    #[derive(Debug)]
    enum UselessErrorKind {
        NumberTooHigh,
        NumberTooLow,
    }

    fn process_number(x: i32) -> Result<i32, UselessErrorKind> {
        match x {
            x if x < 10 => Err(UselessErrorKind::NumberTooLow),
            x if x > 10 && x < 100 => Ok(x * 25),
            _ => Err(UselessErrorKind::NumberTooHigh),
        }
    }

    println!("{:?}", process_number(8));
    println!("{:?}", process_number(70));
    println!("{:?}", process_number(140));

    // Open a file

    let f = File::open("not-found.txt");
    println!("{:?}", f);

    // Match on result

    match f {
        Ok(file) => println!("File found: \n\n{:?}", file),
        Err(error) => match error.kind() {
            FileErrorKind::NotFound => eprintln!("File not found: {:?}", error.kind()),
            _ => eprintln!("Unknown error: {:?}", error),
        },
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

    // propagating errors with a match

    fn propagate_error_with_match() -> Result<i32, UselessErrorKind> {
        let z = match process_number(9) {
            Ok(x) => x,
            Err(x) => return Err(x),
        };
        return Ok(z * 50);
    }

    // propagating errors wit a ?

    fn propagate_error_with_interrogation_mark() -> Result<i32, UselessErrorKind> {
        let f = process_number(9);
        let z = process_number(9)?;
        return Ok(z * 50);
    }
}
