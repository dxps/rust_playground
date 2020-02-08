// ----------------------------------------------------------------------------
// It shows how a vector (homogeneous type structure) can contain different
// data types based on a common trait. `&dyn` is a language feature allowing
// a dynamic (at runtime, versus static at compile time) dispatch.
// ----------------------------------------------------------------------------

use std::fmt::Display;

fn show_all(items: Vec<&dyn Display>) {
    for item in items {
        println!(">>> {}", item)
    }
}

fn main() {
    let items = vec![&12 as &dyn Display, &"Hi" as &dyn Display];
    println!();
    show_all(items);
    println!();
}
