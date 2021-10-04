use std::thread;
use std::time::Duration;

fn main() {
    let main_thread = thread::current();

    println!(
        "[{:?} {:?}] spanning a thread now.",
        main_thread.name().unwrap(),
        main_thread.id()
    );

    let join_handle = thread::spawn(|| {
        for i in 1..4 {
            let curr_thread = thread::current();
            println!(
                "[{:?} {:?}] step #{}.",
                curr_thread.name().unwrap_or("unnamed"),
                curr_thread.id(),
                i
            );
            thread::sleep(Duration::new(1, 0));
        }
    });

    println!(
        "[{:?} {:?}] is joining the spawned thread.",
        main_thread.name().unwrap(),
        main_thread.id().to_owned(),
    );
    // Waiting for the spawned thread to finish, then end our main function (and thread).
    join_handle.join().unwrap();
}
