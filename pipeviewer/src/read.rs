use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use std::sync::{Arc, Mutex};

/// This function reads from the provided ref to the input file.
/// It just signals if an error has occured (that's why the `std::io::Result`),
/// otherwise, it does not return a value (see `()` unit).
pub fn read_loop(infile: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        // TODO: Send this buffer to the stats thread.
        Vec::from(&buffer[..num_read]);
    }
    // TODO: Send an empty buffer to the stats thread.
    // Let the caller we're done.
    let mut quit = quit.lock().unwrap();
    *quit = true;
    Ok(())
}
