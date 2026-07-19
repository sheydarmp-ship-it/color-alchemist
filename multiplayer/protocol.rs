use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Packet {
    Join {
        name: String,
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
}