use std::thread;
use std::time::Duration;

fn main() {
    println!(
        "[{:?}] spanning a thread now.",
        thread::current().name().unwrap()
    );

    let join_handle = thread::spawn(|| {
        for i in 1..4 {
            println!("[{:?}] step #{}", thread::current().id(), i);
            thread::sleep(Duration::new(1, 0));
        }
    });

    // Waiting for the spawned thread to finish, then end our main function (and thread).
    join_handle.join().unwrap();
}
