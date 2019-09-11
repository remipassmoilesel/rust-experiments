pub fn main() {
    let mut i: i32 = 0;
    loop {
        i += 1;
        if i > 2 {
            break;
        }
    }

    let range = 0..10;

    for i in 0..10 { // Every object that implements IntoIterator
        println!("i vaut : {}", i);
    }

    let v = vec!(1, 4, 5, 10, 6);

    for value in v {
        println!("{}", value);
    }

    for (i, j) in (5..10).enumerate() {
        println!("i = {} et j = {}", i, j);
    }

    let v = vec!("a", "b", "c", "d");

    for (i, value) in v.iter().enumerate() {
        println!("i = {} et value = \"{}\"", i, value);
    }
}