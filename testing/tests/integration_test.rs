use testing;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, testing::add(2, 2));
}