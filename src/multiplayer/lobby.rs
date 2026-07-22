use std::net::TcpStream;
pub struct Player {
    pub name: String,
    pub stream: TcpStream,
    // pub ready: bool,
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
        self.players.push(Player { name, stream });
    }

    pub fn remove_player(&mut self, name: &str) {
        self.players.retain(|p| p.name != name);
    }

    pub fn is_ready(&self) -> bool {
        self.players.len() >= 2
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
    /*pub fn set_ready(
        &mut self,
        name: &str,
    ) {
        for player in &mut self.players {

            if player.name == name {

               // player.ready = true;

            }

        }
    }*/

    /*pub fn everyone_ready(&self) -> bool {
        self.players.len() >= 2
            && self.players.iter().all(|p| p.ready)
    }*/
}
