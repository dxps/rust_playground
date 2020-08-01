use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use std::sync::mpsc::Sender;

/// This function reads from the provided ref to the input file.
/// It just signals if an error has occured (that's why the `std::io::Result`),
/// otherwise, it does not return a value (see `()` unit).
pub fn read_loop(infile: &str, stats_tx: Sender<Vec<u8>>) -> Result<()> {
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
        // Send this buffer to the stats thread.
        if stats_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break; // exit out cleanly
        }
    }
    // Send an empty buffer to the stats thread.
    let _ = stats_tx.send(Vec::new());
    Ok(())
}
