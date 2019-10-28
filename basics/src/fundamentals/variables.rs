pub fn main() {
    let _i = 0;
    let mut j = 2;
    j += 2;
    assert_eq!(j, 4);

    let _k: i32 = 0;
    let _l = 10i32;

    // no warning here because of _
    let _unused_variable = 3;

    let _tab = &[0, 1, 2];

    const MAX_POINTS: u32 = 100_000;
    assert_eq!(MAX_POINTS, 100_000);

    // Shadowing is allowed
    let x = 5;
    let x = x + 1;
    let _x = x * 2;

    // overflows break build unless you annotate it
    // rust warn us about i8 too if we want to compare it to a too great value (300) so we use allow(unused_comparisons)
    let mut i: i8 = 0;
    #[allow(overflowing_literals, unused_comparisons)]
    while i < 300 {
        i = i + 1;
        println!("{}", i)
    }

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    let _x = "Test string"; // literal string are immutable
    let _x = String::from("Test string");

    // Slices
    let s = String::from("hello");

    let len = s.len();

    let _slice = &s[3..len];
    let _slice = &s[3..];
}
