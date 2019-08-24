//
// A set of integration tests.
//

use adder;

#[test]
fn it_1st_test() {
    assert_eq!(7, adder::my_logic_function1(5), "Expected to return 7");
}