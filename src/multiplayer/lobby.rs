use crate::gamestate::Difficulty;

pub struct NetworkPlayer {
    pub name: String,
    pub difficulty: Difficulty,
}
pub struct Room {
    pub difficulty: Difficulty,
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

                    difficulty: Difficulty::Easy,

                    players: Vec::new(),

                },

                Room {

                    difficulty: Difficulty::Medium,

                    players: Vec::new(),

                },

                Room {

                    difficulty: Difficulty::Hard,

                    players: Vec::new(),

                },

            ],

        }

    }
    pub fn add_player(

    &mut self,

    name: String,

    difficulty: Difficulty,

) {

    for room in &mut self.rooms {

        if room.difficulty == difficulty {

            room.players.push(

                NetworkPlayer {

                    name,

                    difficulty,

                }

            );

            break;

        }

    }

}
pub fn remove_player(

    &mut self,

    name: &str,

) {

    for room in &mut self.rooms {

        room.players.retain(

            |player| player.name != name

        );

    }

}
}
impl Room {
    pub fn is_ready(&self) -> bool {
        self.players.len() >= 2
    }
}

impl Lobby {
    pub fn check_rooms(&self) {
        for room in &self.rooms {
            if room.is_ready() {
                println!("{:?} room is ready!", room.difficulty);
            }
        }
    }
}