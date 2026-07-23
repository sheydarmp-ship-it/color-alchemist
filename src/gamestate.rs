use crate::color::ColorRGB;
use crate::multiplayer::client::Client;
use crate::player::Player;
use crate::Packet;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, PartialEq)]
pub enum RoundResult {
    Playing,
    Win,
    Fail,
    TimeUp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn time_limit(&self) -> f32 {
        match self {
            Difficulty::Easy => 60.0,
            Difficulty::Medium => 30.0,
            Difficulty::Hard => 15.0,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Difficulty::Easy => 1,
            Difficulty::Medium => 2,
            Difficulty::Hard => 3,
        }
    }
}

pub struct GameState {
    pub target: ColorRGB,
    pub player: Player,

    pub level: u32,

    pub difficulty: Difficulty,

    pub time_left: f32,

    pub result: RoundResult,

    pub message: String,

    pub high_score: u32,

    pub online: bool,

    pub client: Option<Client>,

    pub online_accuracy: Option<f32>,

    pub network_message: String,

    pub waiting_for_player: bool,

    pub hint: String,
}

impl GameState {
    pub fn new(player_name: &str, difficulty: Difficulty, high_score: u32) -> Self {
        Self {
            target: ColorRGB::random(),
            player: Player::new(player_name),

            level: 1,

            difficulty,

            time_left: difficulty.time_limit(),

            result: RoundResult::Playing,

            message: "Match the color!".to_string(),

            high_score,

            online: false,

            client: None,

            online_accuracy: None,

            network_message: String::new(),

            waiting_for_player: false,

            hint: String::new(),
        }
    }

    pub fn similarity(&self) -> f32 {
        self.target.similarity(&self.player.guess)
    }

    pub fn submit_guess(&mut self) {
        if self.similarity() >= 90.0 {
            self.player.add_score(self.difficulty.score());
            if self.player.score > self.high_score {
                self.high_score = self.player.score;
            }
            self.result = RoundResult::Win;

            self.message = "Level Complete!".to_string();
        } else {
            self.result = RoundResult::Fail;

            self.message = format!("Try Again! ({:.1}%)", self.similarity());
        }
    }

    pub fn update(&mut self, delta_time: f32) {
    if self.result != RoundResult::Playing {
        return;
    }

    self.time_left -= delta_time;

    if self.time_left <= 0.0 {
        self.time_left = 0.0;

        if self.is_online() {
            if let Some(client) = self.client.as_mut() {
                client.send(Packet::TimeUp);
            }
        }

        self.result = RoundResult::TimeUp;
        self.message = "Time Up!".to_string();
    }
}

    pub fn next_level(&mut self) {
        self.level += 1;

        self.target = ColorRGB::random();

        self.restart_round();
    }

    pub fn restart_round(&mut self) {
        self.player.reset_guess();

        self.time_left = self.difficulty.time_limit();

        self.result = RoundResult::Playing;

        self.message = "Match the color!".to_string();
    }

    pub fn enable_online(&mut self, client: Client) {
        self.online = true;
        self.client = Some(client);
    }

    pub fn is_online(&self) -> bool {
        self.online
    }
}
