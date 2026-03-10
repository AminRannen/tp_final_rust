
mod commands;
mod protocol;
mod store;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use protocol::Request;
use store::Store;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let store = Store::new();

    let listener = TcpListener::bind("127.0.0.1:7878").await
        .expect("impossible de bind sur 7878");
    println!("[*] Serveur lancé sur 127.0.0.1:7878");

    loop {
        let (socket, addr) = listener.accept().await
            .expect("erreur accept");
        println!("[*] Nouvelle connexion : {addr}");


        let store = store.clone();

        tokio::spawn(async move {
            handle_client(socket, store).await;
        });
    }
}

async fn handle_client(
    socket: tokio::net::TcpStream,
    store: Store,
) {
    let (read_half, mut write_half) = socket.into_split();
    let mut reader = BufReader::new(read_half);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!("[*] Client déconnecté");
                break;
            }
            Ok(_) => {
                let trimmed = line.trim();
                println!("[>] Reçu : {trimmed}");

                let response = match serde_json::from_str::<Request>(trimmed) {
                    Ok(req) => {
                        commands::handle_command(req, &store)
                    }
                    Err(_) => {
                        protocol::Response::error("invalid json")
                    }
                };

                let response_line = format!("{}\n", response.to_json());
                println!("[<] Envoi : {}", response.to_json());

                if write_half.write_all(response_line.as_bytes()).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("[!] Erreur : {e}");
                break;
            }
        }
    }
}
