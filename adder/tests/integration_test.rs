use adder;

mod common;

#[test]
fn four_adds_two() {
    common::common_setup();
    assert_eq!(4, adder::add_two(2));
}
