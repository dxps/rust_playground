use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::sync::{Arc, Mutex};

pub fn write_loop(outfile: &str, quit: Arc<Mutex<bool>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };
    loop {
        // TODO: Receive vector from stats thread.
        let buffer = Vec::<u8>::new(); // just to compile
        {
            // Check if we need to quit. This is done in its own short scope,
            // so that received `quit` ref will get locked for a short time.
            let quit = quit.lock().unwrap();
            if *quit {
                break;
            }
        }
        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // Stop the program cleanly.
                return Ok(());
            }
            return Err(e);
        }
    }
    Ok(())
}
