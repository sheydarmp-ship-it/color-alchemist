use serde::{Deserialize, Serialize};

use crate::gamestate::Difficulty;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packet {
    Join {
        name: String,
        difficulty: Difficulty,
    },

    Ready,

    Guess {
        r: u8,
        y: u8,
        b: u8,
    },

    Hint,

    Leave,

    StartGame,

    TargetColor {
        r: u8,
        y: u8,
        b: u8,
    },

    Winner {
        name: String,
        similarity: f32,
    },

    TimeUp,

    Waiting,
}