use anyhow::{Error, Result};

fn main() {
    if let Err(err) = do_something() {
        // Note: The Debug format (`{:?}`) includes the backtrace (aka showing the "Caused by:" section).
        println!("Got error: {:?}", err)
    }
}

fn do_something() -> Result<()> {
    //
    println!("done something");
    let err = Error::msg("some failure").context("someContext");
    Err(err)
}
