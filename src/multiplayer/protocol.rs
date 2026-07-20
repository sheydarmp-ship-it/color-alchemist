use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packet {
    Join {
        name: String,
    },

    Guess {
        r: u8,
        g: u8,
        b: u8,
    },

    Leave,
}