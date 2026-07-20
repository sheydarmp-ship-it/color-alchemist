use tokio::net::{TcpListener, TcpStream};

use crate::multiplayer::packet::receive_packet;
use crate::multiplayer::protocol::Packet;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr)
            .await
            .expect("Cannot bind server");

        println!("Server started on {}", addr);

        Self { listener }
    }

    pub async fn run(&self) {
        loop {
            let (socket, address) =
                self.listener.accept().await.unwrap();

            println!("Player connected: {}", address);

            tokio::spawn(async move {
                handle_client(socket).await;
            });
        }
    }
}

async fn handle_client(
    mut socket: TcpStream,
) {
    loop {
        let packet = receive_packet(&mut socket).await;

        let Some(packet) = packet else {
            println!("Client disconnected.");
            break;
        };

        println!("Packet => {:?}", packet);

        match packet {
            Packet::Join {
                name,
                difficulty,
            } => {
                println!(
                    "{} joined ({:?})",
                    name,
                    difficulty
                );
            }

            Packet::Guess { r, y, b } => {
                println!(
                    "Guess => ({}, {}, {})",
                    r,
                    y,
                    b
                );
            }

            Packet::Hint => {
                println!("Hint requested");
            }

            Packet::Ready => {
                println!("Player Ready");
            }

            Packet::Leave => {
                println!("Player Left");
                break;
            }

            Packet::StartGame => {
                println!("Start Game");
            }

            Packet::TargetColor { r, y, b } => {
                println!(
                    "Target => {} {} {}",
                    r,
                    y,
                    b
                );
            }

            Packet::Winner {
                name,
                similarity,
            } => {
                println!(
                    "Winner {} ({:.2}%)",
                    name,
                    similarity
                );
            }

            Packet::TimeUp => {
                println!("Time Up");
            }

            Packet::Waiting => {
                println!("Waiting...");
            }
        }
    }
}