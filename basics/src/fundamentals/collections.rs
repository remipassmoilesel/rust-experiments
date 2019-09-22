pub fn main() {
    // Immutable empty vector, immutable vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Simple mutation
    let mut v = Vec::new();
    v.push(5);

    // Scope
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here

    // Elements access

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // 6 here cause panic !
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    match v.get(6) {
        Some(third) => println!("The element 6 is {}", third),
        None => println!("There is no element 6."),
    }

    // Immutability issue, we keep a reference from element 1 and we try to mutate
    // collection, fail !
    //
    //      let mut v = vec![1, 2, 3, 4, 5];
    //      let first = &v[0];
    //      v.push(6);
    //      println!("The first element is: {}", first);
    //

    // Iterations

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Mutations

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // we dereference i here with *
    }

    // Multiple types in one vector

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Slice of vector

    let v = vec![100, 32, 57];
    println!("{:?}", &v[1..]);
    println!("{:?}", &v[1..2]);
    println!("{:?}", &v[..2]);
}
