use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod resp;
mod resp_result;

#[tokio::main]
async fn main() {
    //
    let endpoint = "127.0.0.1:6379";
    let listener = TcpListener::bind(endpoint)
        .await
        .expect(&format!("Failed to listen to {}.", endpoint));

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("New connection from '{}'.", stream.peer_addr().unwrap());
                tokio::spawn(handle_connection(stream));
            }
            Err(e) => {
                println!("Failed to accept connection. Cause: '{}'.", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buff = [0; 512];
    loop {
        match stream.read(&mut buff).await {
            Ok(0) => {
                println!("Connection closed.");
                break;
            }
            Ok(_) => {
                let response = "+PONG\r\n";
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("Failed to write to connection. Cause: '{}'.", e);
                }
            }
            Err(e) => {
                println!("Failed to read from connection. Cause: '{}'.", e);
                break;
            }
        }
    }
}
