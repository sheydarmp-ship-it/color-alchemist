use crate::color::ColorRGB;

pub struct Player {
    pub name: String,
    pub score: u32,
    pub guess: ColorRGB,
}
impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            score: 0,
            guess: ColorRGB::new(128, 128, 128),
        }
    }
    pub fn add_score(&mut self, points: u32) {
        self.score += points;
    }
    pub fn reset_guess(&mut self) {
        self.guess = ColorRGB::new(128, 128, 128);
    }
}
