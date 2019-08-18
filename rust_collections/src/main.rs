use std::collections::HashMap;

fn main() {
    println!("\n---------- vector > getting elements ----------\n");

    let mut v = vec![1, 2, 3]; // using vec! macro, otherwise use Vector::new()
    v.push(4);
    // using the indexing syntax, with '&' to return an immutable reference
    println!("4th element is {}", &v[3]);
    // using the get method, returning an Option<&T>
    match v.get(4) {
        Some(element) => println!("5th element is {}", element),
        None => println!("5th element is non-existent"),
    }

    println!("\n---------- hash maps & ownership ----------\n");

    let mut hm = HashMap::new();
    let user1_uid = "uid-123"; // this is an &str, so no movement happens
    let _user2_uid = String::from("uid-123"); // but this String would be moved
                                              // so we won't be able to use it later
    hm.insert(user1_uid, String::from("user-1"));
    hm.insert("uid-456", String::from("user-2"));
    println!("Added uid: {:?} to hm = {:?}", user1_uid, hm);
}
