pub fn main() {

    // MOVE

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // s1 is not valid anymore, because value moved to s2

    // COPY

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // CLONE

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // PASS OWNERSHIP
    fn take_ownership(value: String) {
        println!("{}", value)
    }

    let x = String::from("Test string");
    take_ownership(x);

    // println!("{}", x); // Error: x moved

    fn concat(a: &String, b: &String) -> String {
        format!("{}{}", &a, &b)
    }

    let result = concat(&String::from("a"), &String::from("b"));
    println!("{}", result);

    // We cannot change value of a reference
    //fn change(some_string: &String) {
    //    some_string.push_str(", world");
    //}

    fn mutate(x: &mut String) {
        x.push_str(" mutated !");
    }

    let mut a = String::from("Part 1");
    mutate(&mut a); // only one mutable reference per scope is allowed
    {
        let second_mutable_ref = &mut a;
    }
    println!("{}", a);

}