use tokio::net::TcpListener;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::accept_async;
use tokio::sync::{broadcast};
use serde_json::json;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("WebSocket server running on ws://127.0.0.1:8080");
    io::stdout().flush().unwrap();

    let (tx, _) = broadcast::channel::<String>(32);

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream).await.expect("Failed to accept");
        let (mut write, mut read) = ws_stream.split();
        let tx = tx.clone();
        
        let mut rx = tx.subscribe();

        // Handle incoming messages
        tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                let msg_text = msg.to_text().unwrap().trim();
                if msg_text == "next_slide" || msg_text == "prev_slide" {
                    let update = json!({
                        "command": msg_text
                    }).to_string();
                    
                    if let Err(e) = tx.send(update) {
                        println!("Broadcast error: {}", e);
                    }
                }
            }
        });

        // Handle outgoing messages
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Err(e) = write.send(msg.into()).await {
                    println!("Send error: {}", e);
                    break;
                }
            }
        });
    }
}