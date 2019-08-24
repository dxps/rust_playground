/// A function part of my logic that returns 7.
pub fn my_logic_function1(a: i32) -> i32 {
    println!("Input: {}", a);
    my_logic_internal_function();
    7
}

fn my_logic_internal_function() -> bool {
    // nothing explicit here ...
    true
}

#[cfg(test)]
mod tests {

    // as a submodule, I need to import
    // the elements from the parent module
    use super::*;

    #[test]
    fn ut_my_logic_function1() {
        assert_eq!(7, my_logic_function1(5), "Expected to return 7");
    }

    #[test]
    // we can also test a private function
    fn ut_my_logic_internal_function() {
        assert!(my_logic_internal_function(), "Expected to return true");
    }

}
