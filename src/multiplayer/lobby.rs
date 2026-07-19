pub struct NetworkPlayer {
    pub name: String,
}
pub struct Room {
    pub difficulty: String,
    pub players: Vec<NetworkPlayer>,
}
pub struct Lobby {
    pub rooms: Vec<Room>,
}
impl Lobby {
    pub fn new() -> Self {
        Self {
            rooms: vec![
                Room {
                    difficulty: "Easy".to_string(),
                    players: Vec::new(),
                },
                Room {
                    difficulty: "Medium".to_string(),
                    players: Vec::new(),
                },
                Room {
                    difficulty: "Hard".to_string(),
                    players: Vec::new(),
                },
            ],
        }
    }
    pub fn add_player(
    &mut self,
    name: String,
    difficulty: String,
) {
    for room in &mut self.rooms {

        if room.difficulty == difficulty {

            room.players.push(NetworkPlayer {
                name,
            });

            return;
        }
    }
}
}
impl Room {
    pub fn is_ready(&self) -> bool {
        self.players.len() >= 2
    }
    pub fn check_rooms(&self) {
    for room in &self.rooms {

        if room.is_ready() {

            println!(
                "{} room is ready!",
                room.difficulty
            );

        }

    }
}
}