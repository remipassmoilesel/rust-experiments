pub fn main() {
    // Loops

    let mut i: i32 = 0;
    loop {
        i += 1;
        if i > 2 {
            break;
        }
    }

    let var = loop {
        break i * 2;
    };
    assert_eq!(var, 6);

    // While

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // For

    let _range = 0..10;

    for i in 0..10 {
        // Every object that implements IntoIterator
        println!("i vaut : {}", i);
    }

    let v = vec![1, 4, 5, 10, 6];

    for value in v {
        println!("{}", value);
    }

    for (i, j) in (5..10).enumerate() {
        println!("i = {} et j = {}", i, j);
    }

    let v = vec!["a", "b", "c", "d"];

    for (i, value) in v.iter().enumerate() {
        println!("i = {} et value = \"{}\"", i, value);
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
