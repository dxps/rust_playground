// use crate::stats::timer::Timer;
use super::timer::Timer;
use crossbeam::channel::Receiver;
use crossterm::{
    cursor, execute,
    style::{self, Color, PrintStyledContent},
    terminal::{Clear, ClearType},
};
use std::io::{self, Result, Stderr, Write};
use std::time::Instant;

/// `stats_loop` function measures and output the statistics based on the number of bytes
/// that it receives through the provided `stats_rx` channel.
///
/// ## Example
/// Here is an example of how to use it.
///
/// ```rust
/// use std::thread;
/// use crossbeam::unbounded;
/// use pipeviewer::stats::stats_loop;
/// let silent = true;
/// let (stats_tx, stats_rx) = unbounded();
/// let stats_handle = thread::spawn(move || stats_loop(silent, stats_rx));
/// stats_tx.send(5);
/// stats_tx.send(0);
/// stats_handle.join();
/// ```
pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    let mut timer = Timer::new();
    let mut stderr = io::stderr();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();
        total_bytes += num_bytes;

        if !silent && timer.ready {
            timer.ready = false;
            output_progress(&mut stderr, total_bytes, start.elapsed().as_secs().as_time(), rate_per_second);
        }

        if num_bytes == 0 {
            break;
        }
    }
    if !silent {
        eprintln!();
    }
    Ok(())
}

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let elapsed = style::style(elapsed).with(Color::Green);
    let bytes = style::style(format!(" {}", bytes)).with(Color::Red);
    let rate = style::style(format!(" [{:.0}B/s]", rate)).with(Color::Blue);
    let _ = execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(elapsed),
        PrintStyledContent(bytes),
        PrintStyledContent(rate),
    );
    let _ = stderr.flush();
}

/// The `TimeOutput` trait adds a `.as_time()` method to `u64`.
trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::TimeOutput;

    #[test]
    fn as_time_format() {
        let pairs = vec![(5_u64, "0:00:05"), (80_u64, "0:01:20"), (3607_u64, "1:00:07")];
        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output);
        }
    }
}
