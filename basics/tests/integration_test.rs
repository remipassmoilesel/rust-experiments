extern crate basics;

use basics::*;
use basics::testing::calculator::Calculator;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(Calculator::add(&2, &3), Ok(5));
}