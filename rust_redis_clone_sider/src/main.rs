use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let endpoint = "127.0.0.1:6379";
    let listener = TcpListener::bind(endpoint).expect(&format!("Failed to listen to {}.", endpoint));

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection from '{}'.", stream.peer_addr().unwrap());
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("Failed to accept connection. Cause: '{}'.", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buff = [0; 512];
    loop {
        match stream.read(&mut buff) {
            Ok(0) => {
                println!("Connection closed.");
                break;
            }
            Ok(_) => {
                let response = "+PONG\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                println!("Failed to read from connection. Cause: '{}'.", e);
                break;
            }
        }
    }
}
