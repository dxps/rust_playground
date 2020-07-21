use crate::http::Request;
use std::convert::TryFrom;
use std::io::Read;
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
                println!("Error: Cannot listen on '{}'. Details: {}", self.addr, error);
                return;
            }
        };

        loop {
            match listener.accept() {
                Ok((mut stream, client_addr)) => {
                    // get all the data sent by the client
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received request from {}: '{}'.", client_addr, String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => println!("Error: Failed to parse the request. Details: {}", e),
                            }
                        }
                        Err(e) => println!("Error: Failed to read from conn. Details: {}", e),
                    }
                }
                Err(err) => println!("Error: Cannot establish connection. Details: {}", err),
            }
        }
    }
}
