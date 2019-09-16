pub fn main() {
    let i = 0;
    let mut j = 2;
    j += 2;

    let k: i32 = 0;
    let l = 10i32;

    // no warning here because of _
    let _unused_variable = 3;

    let tab = &[0, 1, 2];

    const MAX_POINTS: u32 = 100_000;

    // Shadowing is allowed
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    // overflows break build unless you annotate it
    let mut i: i8 = 0;
    #[allow(overflowing_literals)]
    while i < 300 {
        i = i + 1;
        println!("{}", i)
    }

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let x = "Test string"; // literal string are immutable
    let x = String::from("Test string");

    // Slices
    let s = String::from("hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

}