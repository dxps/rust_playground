use clap::{App, Arg};
use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

/// 16 kb for the chunk of the bytes
const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let matches = App::new("pipeviewer")
        .version("0.0.1")
        .arg(Arg::with_name("infile").help("Read from a file instead of stdin"))
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true),
        )
        .arg(Arg::with_name("silent").short("s").long("silent"))
        .get_matches();

    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    dbg!(infile, outfile, silent);

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("total_bytes: {}", total_bytes);
        }
        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    Ok(())
}
