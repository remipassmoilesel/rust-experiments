
pub fn main(){

    // For unrecoverable, will clean memory and display an error message
    // panic!("Panic message");

    // For recoverable errors: Results
    #[derive(Debug)]
    enum ErrorType {
        NumberTooHigh,
        NumberTooLow
    }

    fn do_something(x: &i32) -> Result<i32, ErrorType>{
        match x {
            x if x < &10 => Err(ErrorType::NumberTooLow),
            x if x > &10 && x < &100 => Ok(x * 25),
            _ => Err(ErrorType::NumberTooHigh),
        }
    }

    println!("{:?}", do_something(&8));
    println!("{:?}", do_something(&70));
    println!("{:?}", do_something(&140));
}