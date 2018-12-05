extern crate adder_for_test;

mod common;

#[test]
fn equal() {
    common::setup();
    assert_eq!(4, 4)
}
