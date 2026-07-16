use macroquad::prelude::*;
use crate::gamestate::Gamestate;
pub struct Renderer;
impl Renderer{
    pub fn new()->Self
    {
        Self
    }
    pub fn draw_background(&self){
        clear_background(DARKGRAY);
    }
    fn draw_target_color(&self,gamestate:&Gamestate)
    {
        draw_rectangle(80.0,120.0,150.0,150.0, game.target_color.to_macroquad());
    }
    fn draw_player_color(&self, gamestate:&Gamestate)
    {
        draw_rectangle(420.0,120.0,150.0,150.0, game.player.current_color.to_macroquad());
    }
}