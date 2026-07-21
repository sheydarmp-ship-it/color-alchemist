use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packet {
    Join {
        name: String,
    },

    Ready,

    Guess {
        r: u8,
        g: u8,
        b: u8,
    },

    RoundResult {
        accuracy: f32,
    },

    Leave,
}