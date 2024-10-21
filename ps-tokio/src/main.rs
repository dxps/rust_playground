use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    signal,
    sync::broadcast,
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    //
    let endpoint = "localhost:8080";
    let listener = TcpListener::bind(endpoint)
        .await
        .expect(format!("Failed to listen to '{endpoint}'.").as_str());
    let (broadcast_tx, _) = broadcast::channel(10);

    let token = CancellationToken::new();
    let cancel_token = token.clone();

    // Graceful shutdown (Ctrl+C) handler.
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                println!("Shutting down the tasks ...");
                cancel_token.cancel();
                return;
            }
            Err(_err) => {}
        }
    });

    loop {
        let broadcast_tx = broadcast_tx.clone();
        let mut rx = broadcast_tx.subscribe();
        let (mut socket, addr) = listener.accept().await.unwrap();

        let token = token.clone();

        tokio::spawn(async move {
            let (stream_reader, mut stream_writer) = socket.split();
            let mut message = String::new();
            let mut reader = BufReader::new(stream_reader);
            loop {
                tokio::select! {
                    result = reader.read_line(&mut message) => {
                        match result {
                            Ok(bytes) => {
                                if bytes == 0 {
                                    break;
                                }
                                broadcast_tx.send((message.clone(), addr)).unwrap();
                                message.clear();
                            },
                            Err(_) => break, // Mainly to cover the Ctrl+C and Enter case (on the client).
                        }

                    }
                    result = rx.recv() => {
                        let (rcv_msg, rcv_addr) = result.unwrap();
                        if rcv_addr != addr {
                            stream_writer.write_all(rcv_msg.as_bytes()).await.unwrap();
                        }
                    }
                    _ = token.cancelled() => {
                        println!("Task shutdown done.");
                        break;
                    }
                }
            }
        });
    }
}
