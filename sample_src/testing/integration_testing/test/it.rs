
extern create integration_testing;

#[test]
fn it_works() {
    assert_eq!(integration_testing::fun1(), 4); // error, fun1 is private
    assert_eq!(integration_testing::fun2(), 4); // Works fine.
}