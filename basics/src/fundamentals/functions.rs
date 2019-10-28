pub fn main() {
    fn get_bigger(nb1: i32, nb2: i32) -> i32 {
        match nb1 {
            x if x > nb2 => nb1,
            _ => nb2,
        }
    }

    assert_eq!(get_bigger(1, 2), 2);

    fn multiple_results() -> (i32, i32) {
        (1, 2)
    }

    assert_eq!(multiple_results(), (1, 2));
}
