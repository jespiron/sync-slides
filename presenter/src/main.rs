use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use serde_json::json;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server running on 127.0.0.1:8080");
    io::stdout().flush().unwrap();

    let (tx, _) = broadcast::channel::<String>(32);
    let current_slide = Arc::new(Mutex::new(0));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let (mut read_half, mut write_half) = socket.into_split();
        let tx = tx.clone();
        let current_slide = Arc::clone(&current_slide);
        
        let mut rx = tx.subscribe();
        
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Err(e) = write_half.write_all(msg.as_bytes()).await {
                    println!("Failed to write to client: {}", e);
                    break;
                }
            }
        });

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let n = match read_half.read(&mut buf).await {
                    Ok(n) if n == 0 => break,
                    Ok(n) => n,
                    Err(_) => break,
                };

                let msg = String::from_utf8_lossy(&buf[..n]);
                if msg.trim() == "next_slide" {
                    let mut slide = current_slide.lock().await;
                    *slide += 1;
                    println!("Bumped slide number to {}", *slide);
                    io::stdout().flush().unwrap();
                    
                    let slide_update = json!({ "current_slide": *slide }).to_string();
                    if let Err(e) = tx.send(slide_update) {
                        println!("Failed to broadcast: {}", e);
                    }
                }
            }
        });
    }
}