use macroquad::prelude::*;
use crate::gamestate::GameState;    
pub struct Renderer;
impl Renderer{
    pub fn new()->Self
    {
        Self
    }
    
     fn draw_background(&self){
        clear_background(DARKGRAY);
    }

    fn draw_target_color(&self,gamestate:&GameState)
    {
        draw_rectangle(80.0,120.0,150.0,150.0, gamestate.target.to_macroquad());
    }

    fn draw_player_color(&self, gamestate:&GameState)
    {
        draw_rectangle(420.0,120.0,150.0,150.0, gamestate.player.guess.to_macroquad());
    }

    fn draw_ui(&self, game: &GameState) {
    draw_text("Target Color",80.0,90.0,30.0,WHITE);
    draw_text("Your Color",420.0, 90.0,30.0, WHITE,);
    let score = format!("Score: {}", game.player.score);
    draw_text(&score, 20.0, 30.0, 30.0,YELLOW,);
    let level = format!("Level: {}", game.level);
    draw_text(&level,20.0,60.0,30.0,GREEN,);
    let similarity = format!("Similarity: {:.1}%", game.similarity());
    draw_text(&similarity,20.0, 95.0,30.0, SKYBLUE,);
    draw_text("R/Y/B : Mix Colors",20.0,520.0,24.0,LIGHTGRAY,);
    draw_text("SPACE : Submit",20.0,550.0,24.0,LIGHTGRAY,);
}

pub fn render(&self, game: &GameState) {
    self.draw_background();
    self.draw_target_color(game);
    self.draw_player_color(game);
    self.draw_ui(game);
}}