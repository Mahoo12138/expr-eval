#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic(expected="error!")]
fn it_error() {
    panic!("error!");
}