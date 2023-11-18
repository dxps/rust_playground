use tokio::signal;

#[tokio::main]
pub async fn main() {
    println!("Server started. The process id is {}.", std::process::id());
    println!("Waiting for the shutdown signal ...");

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("Shutdown signal received.")
        }
        Err(err) => {
            eprintln!("Unable to listen for the shutdown signal: {err}");
            // The server should shutdown in this case.
        }
    }

    println!("Exit now.");
}
