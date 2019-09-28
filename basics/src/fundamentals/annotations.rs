pub fn main() {
    disable_warns();
}

fn disable_warns() {
    let mut i: i8 = 25;

    // disable warn and compile check on i8 value
    #[allow(overflowing_literals, unused_comparisons)]
    while i < 300 {
        i = i + 1;
        println!("{}", i)
    }

    #[allow(dead_code)]
    fn never_used() {}
}
