
#[derive(Debug, Clone)]
struct Example1 {
    value: String
}

#[derive(Debug, Clone)]
struct Example2 {
    value: String
}

impl Example2 {
    pub fn from(ex1: Example1) -> Example2 {
        Example2 { value: ex1.value }
    }
}

pub fn main() {
    iter_to_ref()
}

pub fn iter_to_ref() {
    let strings = vec!("1".to_string(), "2".to_string(), "3".to_string(), "4".to_string());
    let s1: Vec<&str> = strings.iter().map(|s| s.as_str()).collect();

    let ex1: Vec<Example2> = vec![1; 20].iter()
        .map(|i| Example1 { value: format!("{}", i) })
        .map(|i| Example2::from(i))
        .collect();

    println!("{:#?}", ex1);

    let ex1: Vec<Example2> = vec![1; 20].iter()
        .map(|i| Example1 { value: format!("{}", i) })
        .map(|i| Example2::from(i))
        .collect();

    let ex1: Vec<&Example2> = ex1.iter()
        .map(|i| i)
        .collect();

    println!("{:#?}", ex1);
}