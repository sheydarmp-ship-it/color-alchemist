use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncWriteExt;
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
        let (reader, mut writer) = socket.into_split();

let reader = BufReader::new(reader);
let mut lines = reader.lines();
let mut player_name = String::new();

while let Some(line) = lines.next_line().await.unwrap() {

    println!("Received: {}", line);

    let packet: Packet =
        serde_json::from_str(&line).unwrap();

    match packet {

    Packet::Join { name } => {

        let mut lobby =
            lobby.lock().await;

        lobby.add_player(name.clone());
        player_name = name.clone();

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

        let lobby = lobby.lock().await;

        let accuracy =
        lobby.accuracy(r, g, b);

        println!("Accuracy = {:.2}%",accuracy);
        let packet =
    Packet::RoundResult {
        accuracy,
    };

let json =
    serde_json::to_string(&packet)
        .unwrap();

writer
    .write_all(
        format!("{json}\n").as_bytes()
    )
    .await
    .unwrap();
    }

    Packet::RoundResult { .. } => {}

    Packet::Leave => {

        println!("Player left");

    }

    Packet::Ready => {

    let mut lobby = lobby.lock().await;

    lobby.set_ready(&player_name);

    println!("{player_name} is READY");

    if lobby.everyone_ready() {

        println!("====================");
        println!("ALL PLAYERS READY");
        println!("GAME STARTED");
        println!("====================");

    }}
}
    }
}}