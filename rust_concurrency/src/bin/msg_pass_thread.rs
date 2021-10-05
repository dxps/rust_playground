// This example show the MPSC (multiple producer, single consumer) pattern
// in action: a spawned thread takes (moves) the `tx` part of the channel
// into the closure, and the main is just listening on the `rx` (receiving) part.

use std::time::Duration;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let msg = String::from("I'm done");
        // simulating some work that would take 1 second
        std::thread::sleep(Duration::new(1, 0));
        // it should normally work, otherwise it panics
        tx.send(msg).unwrap();
    });

    // waiting for the thread's feedback (result) before ending
    let thread_msg = rx.recv().unwrap();
    println!(">>> Got '{}' from thread.", thread_msg)
}
