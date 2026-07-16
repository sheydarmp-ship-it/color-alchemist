mod color;
mod gamestate;
mod player;
mod renderer;
use macroquad::prelude::*;
use gamestate::GameState;
use renderer::Renderer;
#[macroquad::main("Color Alchemist")]
async fn main() {
let game: GameState = GameState::new("Player");
let renderer: Renderer = Renderer::new();

loop {
    renderer.render(&game);

    next_frame().await;
}

}
