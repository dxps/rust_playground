//
// Example of using `AveragedCollection` that is defined in `lib.rs`.
//

use rust_oop::avg_coll::AveragedCollection;

fn main() {
    let mut coll = AveragedCollection::new();
    coll.add(2).add(8).add(11);
    println!(">>> Collection's average is {}", coll.get_average());
    let item = coll.remove();
    if item.is_some() {
        println!(">>> Removed item {} from collection.", item.unwrap());
        println!(">>> Collection's average is {}", coll.get_average());
    }
}
