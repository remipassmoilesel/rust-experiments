use std::fs;
use std::io::ErrorKind;

pub fn read_file(filepath: &String) -> Result<String, String> {
    let result = fs::read_to_string(filepath).map_err(|err| err.kind());
    match result {
        Ok(s) => Ok(s),
        Err(ErrorKind::NotFound) => Err(format!("File not found: {}", filepath)),
        _ => Err(String::from("Unknown error")),
    }
}
