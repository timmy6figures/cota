// For websocket
use std::{env, io::Error};
use futures_util::{future, StreamExt, TryStreamExt, stream::SplitSink, SinkExt};
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{WebSocketStream, tungstenite::Result};
use tokio_tungstenite::tungstenite::Message;

mod event;
pub use event::*;

mod message;
pub use message::*;

#[tokio::main]
pub async fn run() -> Result<(), Error> {
    //let _ = env_logger::try_init();
    //let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let addr = "localhost:8080".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) -> Result<()> {
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let mut ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("Connected");
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        // Handle msg!
        println!("From {}: [{}]", addr, msg);
        handle_message(&msg);
    }
    Ok(())
}

