use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::multiplayer::protocol::Packet;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect(addr: &str) -> Self {
        let stream = TcpStream::connect(addr).expect("Cannot connect to server");

        println!("Connected to server.");

        Self { stream }
    }
    pub fn send(&mut self, packet: Packet) {
        let json = serde_json::to_string(&packet).unwrap();

        let message = format!("{json}\n");

        self.stream.write_all(message.as_bytes()).unwrap();
    }
    pub fn receive(&mut self) -> Packet {
        let mut reader = BufReader::new(self.stream.try_clone().unwrap());

        let mut line = String::new();

        reader.read_line(&mut line).unwrap();

        serde_json::from_str(&line).unwrap()
    }
}
