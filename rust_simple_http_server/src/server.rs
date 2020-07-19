// Every file in Rust is treated as a module.

use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = match TcpListener::bind(&self.addr) {
            Ok(listener) => listener,
            Err(error) => {
                println!(
                    "Error: Cannot listen on '{}'. Details: {}",
                    self.addr, error
                );
                return;
            }
        };

        loop {
            match listener.accept() {
                Ok((stream, addr)) => {}
                Err(err) => {
                    println!("Error: Cannot establish connection. Details: {}", err);
                }
            }
        }
    }
}
