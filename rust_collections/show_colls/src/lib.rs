use std::collections::HashMap;
use std::collections::LinkedList;

pub fn show_vector() {
    println!("\n---------- vector > getting elements ----------\n");

    // using vec! macro, otherwise use Vector::new().
    // these are imported automatically (part of prelude)
    let mut v = vec![1, 2, 3];
    v.push(4);
    // using the indexing syntax, with '&' to return an immutable reference
    println!("4th element is {}", &v[3]);
    // using the get method, returning an Option<&T>
    match v.get(4) {
        Some(element) => println!("5th element is {}", element),
        None => println!("5th element is non-existent"),
    }
}

pub fn show_hashmap() {
    println!("\n---------- hash maps & ownership ----------\n");

    let mut hm = HashMap::new();
    let user1_uid = "uid-123"; // this is an &str, so no movement happens
    let _user2_uid = String::from("uid-123"); // but this String would be moved
                                              // so we won't be able to use it later
    hm.insert(user1_uid, String::from("user-1"));
    hm.insert("uid-456", String::from("user-2"));
    println!("Added uid: {:?} to hm = {:?}", user1_uid, hm);
}

pub fn show_linkedlist() {
    println!("\n---------- linked list ----------\n");

    let mut ll = LinkedList::new();

    ll.push_back(1);
    ll.push_back(2);
    ll.push_back(3);

    for item in ll {
        print!("{} ", item)
    }
    println!();
}
