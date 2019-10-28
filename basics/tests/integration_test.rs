// Current project must be load as an external crate
// as `tests` directory have a special build configuration.
extern crate basics;

use basics::testing::calculator::Calculator;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(Calculator::add(&2, &3), Ok(5));
}
