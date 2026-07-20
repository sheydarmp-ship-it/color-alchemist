use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::multiplayer::protocol::Packet;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn connect(addr: &str) -> Self {
        let stream = TcpStream::connect(addr)
            .await
            .expect("Cannot connect to server");

        println!("Connected to server.");

        Self { stream }
    }

    pub async fn send(
        &mut self,
        packet: Packet,
    ) {
        let json =
    serde_json::to_string(&packet).unwrap();

    let message = format!("{json}\n");

    self.stream
    .write_all(message.as_bytes())
    .await
    .unwrap();
    }
}