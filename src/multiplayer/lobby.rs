use crate::multiplayer::protocol::Packet;

pub struct Player {
    pub name: String,
}

pub struct Lobby {
    pub players: Vec<Player>,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
        }
    }

    pub fn add_player(
        &mut self,
        name: String,
    ) {
        self.players.push(Player { name });

        println!(
            "Players: {}",
            self.players.len()
        );
    }

    pub fn remove_player(
        &mut self,
        name: &str,
    ) {
        self.players.retain(
            |p| p.name != name
        );

        println!(
            "Players: {}",
            self.players.len()
        );
    }

    pub fn player_count(&self) -> usize {
        self.players.len()
    }

    pub fn is_ready(&self) -> bool {
        self.players.len() >= 2
    }
}