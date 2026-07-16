use crate::color::ColorRGB;
use crate::player::Player;

pub struct GameState {
    pub target: ColorRGB,
    pub player: Player,
    pub level: u32,
    pub time_left: f32,
    pub game_over: bool,
    pub message: String,
}

impl GameState {
    pub fn new(player_name: &str) -> Self {
        Self {
            message: "Match the color!".to_string(),
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
    pub fn submit_guess(&mut self) {

    let similarity = self.similarity();

    let points = similarity.round() as u32;

    self.player.add_score(points);

     if similarity >= 90.0 {
         self.message = "Level Complete!".to_string();
        self.next_level();
    }
    else {
        self.message = "Try Again!".to_string();
    }

}
}