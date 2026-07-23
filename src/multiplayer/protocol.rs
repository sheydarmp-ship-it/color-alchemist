use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packet {
    Join { name: String },

    Waiting,

    StartGame,

    TargetColor { r: u8, g: u8, b: u8 },

    Guess { r: u8, g: u8, b: u8 },

    TimeUp,

    Winner {
    name: String,
    },

    RepeatRound,

    RoundResult { accuracy: f32, win: bool },

    Leave,
}
