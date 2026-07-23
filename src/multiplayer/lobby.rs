use std::net::TcpStream;
use std::io::Write;
use crate::multiplayer::protocol::Packet;
pub struct Player {
    pub name: String,
    pub stream: TcpStream,
     pub submitted: bool,
    pub time_up: bool,
    pub accuracy: f32,
}
pub enum RoundOutcome {
    Waiting,
    Winner(String),
    Repeat,
}

pub struct Lobby {
    pub players: Vec<Player>,
    pub target: (u8, u8, u8),
}

impl Lobby {
    pub fn new() -> Self {
        let mut lobby = Self {
            players: Vec::new(),
            target: (0, 0, 0),
        };

        lobby.new_target();

        lobby
    }

    pub fn add_player(&mut self, name: String, stream: TcpStream) {
       self.players.push(Player {name,stream, submitted: false,time_up: false,accuracy: 0.0});
    }

    pub fn remove_player(&mut self, name: &str) {
        self.players.retain(|p| p.name != name);
    }
    pub fn accuracy(&self, r: u8, g: u8, b: u8) -> f32 {
        let dr = (self.target.0 as f32 - r as f32).powi(2);

        let dg = (self.target.1 as f32 - g as f32).powi(2);

        let db = (self.target.2 as f32 - b as f32).powi(2);

        let distance = (dr + dg + db).sqrt();

        let max = (3.0_f32 * 255.0_f32 * 255.0_f32).sqrt();

        (1.0 - distance / max) * 100.0
    }

    pub fn new_target(&mut self) {
        self.target = (rand::random(), rand::random(), rand::random());
    }
    pub fn record_guess(
    &mut self,
    name: &str,
    accuracy: f32,
) {

    for player in &mut self.players {

        if player.name == name {

            player.submitted = true;
            player.accuracy = accuracy;

            break;
        }

    }

}

pub fn evaluate_round(
    &self,
) -> RoundOutcome {

    if self.players.len() < 2 {
        return RoundOutcome::Waiting;
    }

    let p1 = &self.players[0];
    let p2 = &self.players[1];

    if p1.time_up && p2.time_up {
    return RoundOutcome::Repeat;
    }
    if !p1.submitted && !p1.time_up {
        return RoundOutcome::Waiting;
    }

    if !p2.submitted && !p2.time_up {
        return RoundOutcome::Waiting;
    }
    if p1.submitted && p2.time_up {

    if p1.accuracy >= 90.0 {
        return RoundOutcome::Winner(
            p1.name.clone(),
        );
    }

    return RoundOutcome::Repeat;
}

if p2.submitted && p1.time_up {

    if p2.accuracy >= 90.0 {
        return RoundOutcome::Winner(
            p2.name.clone(),
        );
    }

    return RoundOutcome::Repeat;
}
if p1.submitted && p2.submitted {


    if p1.accuracy < 90.0 && p2.accuracy < 90.0 {
        return RoundOutcome::Repeat;
    }

    if p1.accuracy >= 90.0 && p2.accuracy < 90.0 {
        return RoundOutcome::Winner(
            p1.name.clone(),
        );
    }

    if p2.accuracy >= 90.0 && p1.accuracy < 90.0 {
        return RoundOutcome::Winner(
            p2.name.clone(),
        );
    }

    if p1.accuracy >= 90.0 && p2.accuracy >= 90.0 {

        if p1.accuracy >= p2.accuracy {
            return RoundOutcome::Winner(
                p1.name.clone(),
            );
        } else {
            return RoundOutcome::Winner(
                p2.name.clone(),
            );
        }

    }

}
    RoundOutcome::Repeat

}
pub fn record_time_up(
    &mut self,
    name: &str,
) {

    for player in &mut self.players {

        if player.name == name {

            player.time_up = true;

            break;
        }

    }

}
pub fn reset_round(&mut self){
    for player in &mut self.players {
    player.submitted = false;
    player.time_up = false;
    player.accuracy = 0.0;
}
}
pub fn send_new_round(&mut self) {

    self.new_target();
    self.reset_round();

    let target = Packet::TargetColor {
        r: self.target.0,
        g: self.target.1,
        b: self.target.2,
    };

    let json = serde_json::to_string(&target).unwrap();

    for player in &mut self.players {

        let _ = player
            .stream
            .write_all(format!("{json}\n").as_bytes());
    }
}
}
