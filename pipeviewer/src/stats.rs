use std::io::Result;
use std::sync::{Arc, Mutex};

pub fn stats_loop(silent: bool, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        // TODO: Receive the vector of bytes
        let buffer = Vec::<u8>::new(); // just for compilation
        total_bytes += buffer.len();
        if !silent {
            eprint!("\r[total bytes: {}]", total_bytes);
        }
        // TODO: Send vector to write loop.

        // Check if this needs to quit.
        let quit = quit.lock().unwrap();
        if *quit {
            break;
        }
    }
    eprintln!();
    Ok(())
}
