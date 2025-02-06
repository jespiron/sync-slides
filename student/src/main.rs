use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use serde_json::Value;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();
    
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Connected to the presenter");
    io::stdout().flush().unwrap();

    let (_, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            println!("Received message: {}", msg);
            if let Ok(json_msg) = serde_json::from_str::<Value>(msg.to_text().unwrap()) {
                if let Some(slide_number) = json_msg.get("current_slide") {
                    println!("Current slide number: {}", slide_number);
                    io::stdout().flush().unwrap();
                }
            }
        }
    }
}