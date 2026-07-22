use crate::gamestate::Difficulty;
use serde::{Deserialize, Serialize};
use std::fs;
const SAVE_FILE: &str = "save.json";

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct PlayerSave {
    pub name: String,
    pub easy: u32,
    pub medium: u32,
    pub hard: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    pub players: Vec<PlayerSave>,
}

impl SaveData {
    pub fn load() -> Self {
        match fs::read_to_string(SAVE_FILE) {
            Ok(text) => serde_json::from_str(&text).unwrap_or_default(),
            Err(_) => SaveData::default(),
        }
    }

    pub fn save(&self) {
        if let Ok(text) = serde_json::to_string_pretty(self) {
            let _ = fs::write(SAVE_FILE, text);
        }
    }

    pub fn find_player(&self, name: &str) -> Option<&PlayerSave> {
        self.players.iter().find(|p| p.name == name)
    }

    pub fn find_player_mut(&mut self, name: &str) -> Option<&mut PlayerSave> {
        self.players.iter_mut().find(|p| p.name == name)
    }

    pub fn create_player(&mut self, name: &str) {
        self.players.push(PlayerSave {
            name: name.to_string(),
            easy: 0,
            medium: 0,
            hard: 0,
        });
    }

    pub fn update_highscore(&mut self, name: &str, difficulty: Difficulty, score: u32) {
        if self.find_player(name).is_none() {
            self.create_player(name);
        }

        let player = self.find_player_mut(name).unwrap();

        match difficulty {
            Difficulty::Easy => {
                if score > player.easy {
                    player.easy = score;
                }
            }

            Difficulty::Medium => {
                if score > player.medium {
                    player.medium = score;
                }
            }

            Difficulty::Hard => {
                if score > player.hard {
                    player.hard = score;
                }
            }
        }

        self.save();
    }

    pub fn get_highscore(&self, name: &str, difficulty: Difficulty) -> u32 {
        if let Some(player) = self.find_player(name) {
            match difficulty {
                Difficulty::Easy => player.easy,
                Difficulty::Medium => player.medium,
                Difficulty::Hard => player.hard,
            }
        } else {
            0
        }
    }
    pub fn get_players(&self) -> Vec<PlayerSave> {
        self.players.clone()
    }
}
