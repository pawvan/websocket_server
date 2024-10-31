use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::{stream::StreamExt, SinkExt};
use log::{info, error};

#[tokio::main]
async fn main() {
    env_logger::init();
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Unable to bind TCP listener");

    info!("WebSocket server listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let addr = stream.peer_addr().unwrap();
        tokio::spawn(handle_connection(stream, addr));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, addr: SocketAddr) {
    let ws_stream = accept_async(stream).await.expect("Error during WebSocket handshake");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                info!("Received message: {:?}", msg);
                if write.send(msg).await.is_err() {
                    error!("Failed to send message to {}", addr);
                    return;
                }
            }
            Err(e) => {
                error!("Error while reading message: {}", e);
                return;
            }
        }
    }
}
