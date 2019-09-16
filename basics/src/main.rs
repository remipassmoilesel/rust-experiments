
extern crate basics;

/**

    Use of lib.rs + main.rs like this (main.rs as a wrapper around lib.rs)
    allow to use code from this crate in integration tests.

*/
pub fn main() {
    basics::main();
}