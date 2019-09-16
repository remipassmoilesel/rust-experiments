pub struct Calculator {}

impl Calculator {
    pub fn multiply(x: &i32, y: &i32) -> i32 {
        x * y
    }

    pub fn add(x: &i32, y: &i32) -> i32 {
        x + y
    }

    pub fn minus(x: &i32, y: &i32) -> i32 {
        x - y
    }

    pub fn divide(x: &i32, y: &i32) -> i32 {
        x / y
    }
}
