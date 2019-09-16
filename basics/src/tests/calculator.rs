pub struct Calculator {}

impl Calculator {
    pub fn multiply(x: &i32, y: &i32) -> Result<i32,()> {
        Ok(x * y)
    }

    pub fn add(x: &i32, y: &i32) -> Result<i32,()> {
        Ok(x + y)
    }

    pub fn minus(x: &i32, y: &i32) -> Result<i32,()> {
        Ok(x - y)
    }

    pub fn divide(x: &i32, y: &i32) -> Result<i32,()> {
        Ok(x / y)
    }
}
