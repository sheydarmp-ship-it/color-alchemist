use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::multiplayer::protocol::Packet;

pub async fn send_packet(
    stream: &mut TcpStream,
    packet: &Packet,
) {
    let text = serde_json::to_string(packet).unwrap();

    stream
        .write_all(text.as_bytes())
        .await
        .unwrap();
}

pub async fn receive_packet(
    stream: &mut TcpStream,
) -> Option<Packet> {
    let mut buffer = [0u8; 1024];

    let size = stream.read(&mut buffer).await.ok()?;

    if size == 0 {
        return None;
    }

    let text = String::from_utf8_lossy(&buffer[..size]);

    serde_json::from_str(&text).ok()
}