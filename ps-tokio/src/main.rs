use std::net::SocketAddr;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    signal,
    sync::broadcast::{self, Receiver, Sender},
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    //
    let tracing_subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(tracing_subscriber).unwrap();

    let endpoint = "localhost:8080";
    let listener = TcpListener::bind(endpoint)
        .await
        .expect(format!("Failed to listen to '{endpoint}'.").as_str());
    let (broadcast_tx, _) = broadcast::channel::<(String, SocketAddr)>(10);

    let token = CancellationToken::new();
    let cancel_token = token.clone();

    // Graceful shutdown (Ctrl+C) handler.
    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                tracing::warn!("Shutting down the tasks ...");
                cancel_token.cancel();
                return;
            }
            Err(_err) => {}
        }
    });

    loop {
        let mut socket: TcpStream;
        let addr: SocketAddr;

        tokio::select! {
            result = listener.accept() => {
                (socket, addr) = result.unwrap();
            }
            _ = token.cancelled() => {
                tracing::info!("Ending the listening loop ...");
                break;
            }
        }

        let broadcast_tx = broadcast_tx.clone();
        let rx = broadcast_tx.subscribe();
        let token = token.clone();

        // Handle each connection in a separate task.
        tokio::spawn(async move {
            tracing::info!("Spawning new task ...");
            let (stream_reader, mut stream_writer) = socket.split();
            handle_connection(
                stream_reader,
                &mut stream_writer,
                broadcast_tx,
                rx,
                addr,
                token,
            )
            .await;
        });
    }
}

async fn handle_connection<'a, Reader, Writer>(
    stream_reader: Reader,
    stream_writer: &mut Writer,
    broadcast_tx: Sender<(String, SocketAddr)>,
    mut broadcast_rx: Receiver<(String, SocketAddr)>,
    addr: SocketAddr,
    token: CancellationToken,
) where
    Reader: AsyncRead + Unpin,
    Writer: AsyncWrite + Unpin,
{
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
                        message = message.strip_suffix("\r\n").unwrap().to_string();
                        if message.len() == 0 {
                            continue;
                        }
                        tracing::info!("Received message: {}", message);
                        tracing::info!("Broadcasting message: {}", message);
                        broadcast_tx.send((format!("{}\n", message.clone()), addr)).unwrap();
                        message.clear();
                    },
                    Err(_) => break, // Mainly to cover the Ctrl+C and Enter case (on the client).
                }

            }
            result = broadcast_rx.recv() => {
                let (rcv_msg, rcv_addr) = result.unwrap();
                if rcv_addr != addr {
                    stream_writer.write_all(rcv_msg.as_bytes()).await.unwrap();
                }
            }
            _ = token.cancelled() => {
                tracing::info!("Task shutdown done.");
                break;
            }
        }
    }
}
