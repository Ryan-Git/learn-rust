extern crate learn_rust;

mod common;

#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, learn_rust::add_two(2));
}