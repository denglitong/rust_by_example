extern crate testing;

use testing::add;

mod another;
mod common;
mod some_mod;

#[test]
fn test_add() {
    common::setup();
    some_mod::name();
    another::name();
    another::yet_another::name();
    assert_eq!(add(3, 2), 5);
}
