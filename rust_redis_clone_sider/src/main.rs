use resp::bytes_to_resp;
use server::process_request;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

mod resp;
mod resp_result;
mod server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //
    println!("The server has started.\n");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!("accepted new connection.");
                tokio::spawn(handle_connection(stream));
            }
            Err(e) => {
                println!("error: '{}'.", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    //
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("connection closed");
                break;
            }
            Ok(size) => {
                let mut index = 0;
                let request = match bytes_to_resp(&buffer[..size], &mut index) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("error parsing request: '{}'.", e);
                        return;
                    }
                };

                let response = match process_request(request) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("error processing request: '{}'.", e);
                        return;
                    }
                };

                if let Err(e) = stream.write_all(response.to_string().as_bytes()).await {
                    eprintln!("error writing output: '{}'.", e);
                }
            }
            Err(e) => {
                println!("error reading input: '{}'.", e);
            }
        }
    }
}
