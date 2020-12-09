extern crate adder;

mod common;

#[test]
fn it_add_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2))
}
#[test]
fn it_add_two2() {
    assert_eq!(4, adder::add_two(2))
}
