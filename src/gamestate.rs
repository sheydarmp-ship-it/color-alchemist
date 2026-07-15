use crate::color::ColorRGB;
use crate::player::Player;

pub struct GameState {
    pub target: ColorRGB,
    pub player: Player,
    pub level: u32,
    pub time_left: f32,
    pub game_over: bool,
}

impl GameState {
    pub fn new(player_name: &str) -> Self {
        Self {
            target: ColorRGB::random(),
            player: Player::new(player_name),
            level: 1,
            time_left: 60.0,
            game_over: false,
        }
    }

    pub fn similarity(&self) -> f32 {
        self.target.similarity(&self.player.guess)
    }

    pub fn next_level(&mut self) {
        self.level += 1;
        self.target = ColorRGB::random();
        self.player.reset_guess();
        self.time_left = 60.0;
    }

    pub fn end_game(&mut self) {
        self.game_over = true;
    }
}