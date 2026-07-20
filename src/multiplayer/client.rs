use tokio::net::TcpStream;

use crate::multiplayer::packet::{
    receive_packet,
    send_packet,
};

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
        packet: &Packet,
    ) {
        send_packet(&mut self.stream, packet).await;
    }

    pub async fn receive(
        &mut self,
    ) -> Option<Packet> {
        receive_packet(&mut self.stream).await
    }
}