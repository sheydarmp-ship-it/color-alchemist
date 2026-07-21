use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::multiplayer::{
    lobby::Lobby,
    protocol::Packet,
};

pub struct Server;

impl Server {
    pub fn run(addr: &str) {

        let lobby = Arc::new(
            Mutex::new(
                Lobby::new()
            )
        );

        let listener =
            TcpListener::bind(addr)
                .expect("Cannot bind server");

        println!("================================");
        println!(" Color Alchemist Server Started ");
        println!(" Address : {}", addr);
        println!("================================");

        for stream in listener.incoming() {

            match stream {

                Ok(stream) => {

                    let lobby_clone =
                        lobby.clone();

                    thread::spawn(move || {

                        Self::handle_client(
                            stream,
                            lobby_clone,
                        );

                    });

                }

                Err(e) => {

                    println!(
                        "Connection Error : {}",
                        e
                    );

                }

            }

        }

    }
    fn handle_client(
        mut stream: TcpStream,
        lobby: Arc<Mutex<Lobby>>,
    ) {

        let reader =
            BufReader::new(
                stream.try_clone().unwrap()
            );

        let mut player_name = String::new();

        for line in reader.lines() {

            let line = match line {
                Ok(v) => v,
                Err(_) => break,
            };

            println!("Received : {}", line);

            let packet: Packet =
                serde_json::from_str(&line)
                    .unwrap();

            match packet {

                Packet::Join { name } => {

                    player_name = name.clone();

                    let mut lobby =
                        lobby.lock().unwrap();

                    lobby.add_player(name.clone());

                    println!("{} joined.", name);

                    println!("Players : {}", lobby.players.len());

                    let packet = Packet::TargetColor {
                    r: lobby.target.0,
                    g: lobby.target.1,
                    b: lobby.target.2,};

let json = serde_json::to_string(&packet).unwrap();

stream
    .write_all(format!("{json}\n").as_bytes())
    .unwrap();

if lobby.is_ready() {
    println!("Enough players. Waiting for READY...");
}

                    }

                

                Packet::Ready => {

                    let mut lobby =
                        lobby.lock().unwrap();

                    lobby.set_ready(
                        &player_name
                    );

                    println!(
                        "{} is READY",
                        player_name
                    );

                    if lobby.everyone_ready() {

                        println!("==================");
                        println!("GAME START");
                        println!("==================");

                    }

                }
                                Packet::Guess { r, g, b } => {
                   
                    let lobby =
                        lobby.lock().unwrap();

                    let accuracy =
                        lobby.accuracy(r, g, b);

                    println!(
                        "{} guessed -> R:{} G:{} B:{}",
                        player_name,
                        r,
                        g,
                        b
                    );

                    println!(
                        "Accuracy : {:.2}%",
                        accuracy
                    );

                    let reply = Packet::RoundResult {
                     accuracy,
                     win: accuracy >= 90.0,
                    };

                    let json =
                        serde_json::to_string(&reply)
                            .unwrap();

                    if let Err(e) = stream.write_all(
                        format!("{json}\n").as_bytes(),
                    ) {
                        println!("Send Error: {}", e);
                        break;
                    }

                    if accuracy >= 90.0 {

                        println!(
                            "{} reached target!",
                            player_name
                        );

                    }

                }

                Packet::Leave => {

                    let mut lobby =
                        lobby.lock().unwrap();

                    lobby.remove_player(
                        &player_name
                    );

                    println!(
                        "{} left.",
                        player_name
                    );

                    break;

                }

                Packet::RoundResult { .. } => {}

           
                Packet::TargetColor { .. } => {}
            }

        }

        println!(
            "Connection Closed : {}",
            player_name
        );
    }
    }

