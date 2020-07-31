pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += num_read;
    if !silent {
        eprint!("\r[total bytes: {}]", total_bytes);
        if last {
            eprintln!();
        }
    }
}
