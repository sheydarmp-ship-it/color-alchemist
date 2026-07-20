use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

use crate::multiplayer::protocol::Packet;

pub struct Server;

impl Server {
    pub async fn run(addr: &str) {
        let listener =
            TcpListener::bind(addr)
                .await
                .unwrap();

        println!("Server started on {}", addr);

        loop {
            let (socket, _) =
                listener.accept().await.unwrap();

            tokio::spawn(async move {
                Self::handle_client(socket).await;
            });
        }
    }

    async fn handle_client(
        mut socket: TcpStream,
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
                println!("{name} joined");
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
                println!("Player left");
            }
        }
    }
}