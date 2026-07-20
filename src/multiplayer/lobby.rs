use std::collections::HashMap;

#[derive(Clone)]
pub struct NetworkPlayer {
    pub id: u32,
    pub name: String,
}

pub struct Lobby {
    players: HashMap<u32, NetworkPlayer>,
    next_id: u32,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_player(
        &mut self,
        name: String,
    ) -> u32 {

        let id = self.next_id;

        self.next_id += 1;

        self.players.insert(
            id,
            NetworkPlayer {
                id,
                name,
            },
        );

        id
    }

    pub fn remove_player(
        &mut self,
        id: u32,
    ) {

        self.players.remove(&id);

    }

    pub fn player_count(
        &self,
    ) -> usize {

        self.players.len()

    }

    pub fn is_ready(
        &self,
    ) -> bool {

        self.players.len() >= 2

    }
}