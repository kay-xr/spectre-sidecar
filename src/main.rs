use std::env;
use std::path::PathBuf;
use std::time::Duration;
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tungstenite::Message;

mod initialization;
pub mod log_watcher;
mod regex;
pub mod tail;

// meow :3

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize directory
    let default_dir: PathBuf = {
        let home = dirs::home_dir()
            .unwrap_or_else(|| panic!("couldn’t find home directory via dirs::home_dir()"));
        home.join("AppData")
            .join("LocalLow")
            .join("VRChat")
            .join("VRChat")
    };
    
    // Argument parsing
    let mut port: u16 = 40602;
    let mut dir: PathBuf = default_dir;

    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    
    while i < args.len() {
        match args[i].as_str() {
            "-p" => {
                if i + 1 >= args.len() {
                    panic!("`-p` flag given but no port number was provided");
                }
                port = args[i + 1]
                    .parse::<u16>()
                    .unwrap_or_else(|_| panic!("Failed to parse `{}` as a port number", args[i + 1]));
                i += 2;
            }
            "-d" => {
                if i + 1 >= args.len() {
                    panic!("`-d` flag given but no directory path was provided");
                }
                dir = PathBuf::from(&args[i + 1]);
                i += 2;
            }
            other => {
                panic!("Unknown argument `{}`", other);
            }
        }
    }
    
    // Initialization Messages
    println!("Using port = {}", port);
    println!("Watching directory = {:?}", dir);
    
    initialization::print_output_warning().await;
    
    // Regex Initialization
    let patterns = regex::get_patterns();

    let (tx, _rx) = broadcast::channel::<String>(100);

    // File tailing and file swapping tasks
    {
        let tail_dir = dir.clone();
        let tail_patterns = patterns.clone();
        let tail_tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = tail::tail_multiple_logs(&tail_dir, &tail_patterns, tail_tx).await {
                eprintln!("tail_multiple_logs error: {}", e);
            }
        });
    }
    
    // Websocket suckit server
    {
        let ws_tx = tx.clone();
        tokio::spawn(async move {
            // Bind and accept connections
            let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
                .await
                .expect(&format!("Failed to bind to 127.0.0.1:{}", port));
            println!("WebSocket server listening on ws://127.0.0.1:{}", port);

            while let Ok((stream, addr)) = listener.accept().await {
                println!("New WS client from {}", addr);

                let peer_tx = ws_tx.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, peer_tx).await {
                        eprintln!("WebSocket handler error: {}", e);
                    }
                });
            }
        });
    }

    // keep running until I die
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

async fn handle_connection(raw_stream: TcpStream, tx: broadcast::Sender<String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_stream = tokio_tungstenite::accept_async(raw_stream).await?;

    let (mut ws_sink, _ws_stream_rx) = ws_stream.split();

    let mut rx = tx.subscribe();

    loop {
        let msg = rx.recv().await?;
        ws_sink.send(Message::Text(msg.into())).await?;
    }
}
