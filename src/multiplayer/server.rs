use tokio::io::{AsyncBufReadExt, BufReader};
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
        let reader = BufReader::new(socket);

let mut lines = reader.lines();

while let Some(line) = lines.next_line().await.unwrap() {

    println!("Received: {}", line);

    let packet: Packet =
        serde_json::from_str(&line).unwrap();

    match packet {

    Packet::Join { name } => {

        let mut lobby =
            lobby.lock().await;

        lobby.add_player(name.clone());

        println!("Players: {}", lobby.players.len());

        println!("{name} joined");

        if lobby.is_ready() {
            println!("Game can start!");
        }
    }

    Packet::Guess { r, g, b } => {

        println!(
            "Player guessed -> R:{} G:{} B:{}",
            r,
            g,
            b
        );

    }

    Packet::Leave => {

        println!("Player left");

    }
}
    }
}}