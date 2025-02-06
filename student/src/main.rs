use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json::Value;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    println!("Connected to the presenter");
    io::stdout().flush().unwrap();

    let mut buf = [0; 1024];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }

        let msg = String::from_utf8_lossy(&buf[..n]);
        println!("Received message: {}", msg);
        io::stdout().flush().unwrap();
        if let Ok(json_msg) = serde_json::from_str::<Value>(&msg) {
            if let Some(slide_number) = json_msg.get("current_slide") {
                println!("Current slide number: {}", slide_number);
                io::stdout().flush().unwrap();
            }
        }
    }
}