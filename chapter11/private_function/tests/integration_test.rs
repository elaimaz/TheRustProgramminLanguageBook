use private_function;

mod commom;

#[test]
fn it_adds_two() {
    commom::setup();
    assert_eq!(4, private_function::add_two(2));
}

