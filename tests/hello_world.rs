use rust_exercism::hello_world;

#[test]
fn test_hello_world() {
    assert_eq!("Hello, World!", hello_world::hello());
}
