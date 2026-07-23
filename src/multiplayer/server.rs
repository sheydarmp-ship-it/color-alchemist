use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use crate::multiplayer::{lobby::Lobby, protocol::Packet, lobby::RoundOutcome};

pub struct Server;

impl Server {
    pub fn run(addr: &str) {
        let lobby = Arc::new(Mutex::new(Lobby::new()));

        let listener = TcpListener::bind(addr).expect("Cannot bind server");

        println!("================================");
        println!(" Color Alchemist Server Started ");
        println!(" Address : {}", addr);
        println!("================================");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let lobby_clone = lobby.clone();

                    thread::spawn(move || {
                        Self::handle_client(stream, lobby_clone);
                    });
                }

                Err(e) => {
                    println!("Connection Error : {}", e);
                }
            }
        }
    }
    fn handle_client(mut stream: TcpStream, lobby: Arc<Mutex<Lobby>>) {
        let reader = BufReader::new(stream.try_clone().unwrap());

        let mut player_name = String::new();

        for line in reader.lines() {
            let line = match line {
                Ok(v) => v,
                Err(_) => break,
            };

            println!("Received : {}", line);

            let packet: Packet = serde_json::from_str(&line).unwrap();

            match packet {
                Packet::Join { name } => {
                    player_name = name.trim().to_string();

                    let mut lobby = lobby.lock().unwrap();

                    lobby.add_player(player_name.clone(), stream.try_clone().unwrap());

                    println!("{} joined.", player_name);
                    println!("Players : {}", lobby.players.len());

                    if lobby.players.len() == 1 {
                        let waiting = Packet::Waiting;

                        let json = serde_json::to_string(&waiting).unwrap();

                        stream.write_all(format!("{json}\n").as_bytes()).unwrap();

                        println!("Waiting for second player...");
                    } else if lobby.players.len() == 2 {
                        println!("Two players connected!");

                        let start = Packet::StartGame;

                        let start_json = serde_json::to_string(&start).unwrap();
                        lobby.new_target();

                        let target = Packet::TargetColor {
                            r: lobby.target.0,
                            g: lobby.target.1,
                            b: lobby.target.2,
                        };

                        let target_json = serde_json::to_string(&target).unwrap();

                        for player in &mut lobby.players {
                            player
                                .stream
                                .write_all(format!("{start_json}\n").as_bytes())
                                .unwrap();

                            player
                                .stream
                                .write_all(format!("{target_json}\n").as_bytes())
                                .unwrap();
                        }
                    }
                }
               Packet::Guess { r, g, b } => {

    let mut lobby = lobby.lock().unwrap();

    let accuracy = lobby.accuracy(r, g, b);

    println!(
        "{} guessed -> R:{} G:{} B:{}",
        player_name,
        r,
        g,
        b
    );

    println!("Accuracy : {:.2}%", accuracy);

    lobby.record_guess(&player_name, accuracy);

    match lobby.evaluate_round() {

        RoundOutcome::Waiting => {}

        RoundOutcome::Repeat => {

            println!("Round Repeat");

            lobby.new_target();

            let target = Packet::TargetColor {
                r: lobby.target.0,
                g: lobby.target.1,
                b: lobby.target.2,
            };

            let target_json =
                serde_json::to_string(&target).unwrap();

            for player in &mut lobby.players {

                player.submitted = false;
                player.time_up = false;
                player.accuracy = 0.0;

                player
                    .stream
                    .write_all(
                        format!("{target_json}\n").as_bytes()
                    )
                    .unwrap();
            }

        }

        RoundOutcome::Winner(name) => {

    println!("Winner : {}", name);

    for player in &mut lobby.players {

        let packet = Packet::RoundResult {
            accuracy: player.accuracy,
            win: player.name == name,
        };

        let json = serde_json::to_string(&packet).unwrap();

        let _ = player
            .stream
            .write_all(format!("{json}\n").as_bytes());
    }

    lobby.send_new_round();

}

    }

}
                Packet::TimeUp => {

    let mut lobby = lobby.lock().unwrap();

    lobby.record_time_up(&player_name);

    match lobby.evaluate_round() {

        RoundOutcome::Waiting => {}

       RoundOutcome::Repeat => {

    println!("Round Repeat");

    lobby.new_target();

    let target = Packet::TargetColor {
        r: lobby.target.0,
        g: lobby.target.1,
        b: lobby.target.2,
    };

    let target_json = serde_json::to_string(&target).unwrap();

    for player in &mut lobby.players {

        player.submitted = false;
        player.time_up = false;
        player.accuracy = 0.0;

        player
            .stream
            .write_all(format!("{target_json}\n").as_bytes())
            .unwrap();
    }

}

        RoundOutcome::Winner(name) => {

    println!("Winner : {}", name);

    for player in &mut lobby.players {

        let packet = Packet::RoundResult {
            accuracy: player.accuracy,
            win: player.name == name,
        };

        let json = serde_json::to_string(&packet).unwrap();

        player
            .stream
            .write_all(format!("{json}\n").as_bytes())
            .unwrap();
    }

}

    }

}
                Packet::Waiting => {}
                Packet::StartGame => {}
                Packet::Winner { .. } => {}
                Packet::RepeatRound => {}
                Packet::Leave => {
                    let mut lobby = lobby.lock().unwrap();

                    lobby.remove_player(&player_name);

                    println!("{} left.", player_name);

                    break;
                }

                Packet::RoundResult { .. } => {}

                Packet::TargetColor { .. } => {}
            }
        }

        println!("Connection Closed : {}", player_name);
    }
}
