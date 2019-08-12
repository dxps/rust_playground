fn main() {
    println!("\n---------- Vector > getting elements ----------\n");

    let mut v = vec![1, 2, 3]; // using vec! macro, otherwise use Vector::new()
    v.push(4);
    // using the indexing syntax, with '&' to return an immutable reference
    println!("4th element is {}", &v[3]);
    // using the get method, returning an Option<&T>
    match v.get(4) {
        Some(element) => println!("5th element is {}", element),
        None => println!("5th element is non-existent"),
    }
}
