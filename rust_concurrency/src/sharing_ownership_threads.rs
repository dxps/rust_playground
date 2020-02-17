// This example shows how to use `Arc` (atomically reference counted), a smart pointer
// that is thread-safe, and it allows multiple threads to _own_ a thing (`Mutex` in this case).
// The `Mutex` is used to control the increasing of a counter, and avoid any data races.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(">>> Result: {}", counter.lock().unwrap());
}
