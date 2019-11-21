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
