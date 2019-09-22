pub fn main() {
    fn get_bigger(nb1: i32, nb2: i32) -> i32 {
        match nb1 {
            x if x > nb2 => nb1,
            _ => nb2,
        }
    }

    let anonym_function = |i: i32| -> i32 { i * 2 };

    println!("{}", anonym_function(4));

    fn multiple_results() -> (i32, i32) {
        (1, 2)
    }
}
