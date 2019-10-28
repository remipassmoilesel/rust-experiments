pub fn main() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}", tup.0);

    // destructuration
    let (_x, _y, _z) = tup;

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // create an array with 5 integer

    let _first = a[0];
    let _second = a[1];
}
