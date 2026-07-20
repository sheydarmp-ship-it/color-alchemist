use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::multiplayer::lobby::Lobby;
use crate::multiplayer::protocol::Packet;

pub struct Server;

impl Server {
    pub async fn run(addr: &str) {
        let lobby =
    Arc::new(
        Mutex::new(
            Lobby::new()
        )
    );
        let listener =
            TcpListener::bind(addr)
                .await
                .unwrap();

        println!("Server started on {}", addr);

        loop {
    let (socket, _) = listener.accept().await.unwrap();

    let lobby_clone = lobby.clone();

    tokio::spawn(async move {
        Self::handle_client(socket, lobby_clone).await;
    });
}
    }

    async fn handle_client(
    mut socket: TcpStream,
    lobby: Arc<Mutex<Lobby>>,
) {
        let mut buffer = [0u8; 1024];

        let size =
            socket.read(&mut buffer)
                .await
                .unwrap();

        if size == 0 {
            return;
        }

        let text =
            String::from_utf8_lossy(&buffer[..size]);

        println!("Received: {}", text);

        let packet: Packet =
            serde_json::from_str(&text).unwrap();

        match packet {
            Packet::Join { name } => {
    let mut lobby = lobby.lock().await;

    lobby.add_player(name.clone());

    println!("{name} joined");

    if lobby.is_ready() {
        println!("Game can start!");
    }
}
            Packet::Guess { r, g, b } => {
                println!(
                    "Guess: {} {} {}",
                    r,
                    g,
                    b
                );
            }

            Packet::Leave => {
    let mut lobby = lobby.lock().await;

    lobby.remove_player("Unknown");
}
        }
    }
}