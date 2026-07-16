mod color;
mod gamestate;
mod player;
mod renderer;
use macroquad::prelude::*;
use gamestate::GameState;
use renderer::Renderer;
#[macroquad::main("Color Alchemist")]
async fn main() {
let mut game: GameState = GameState::new("Player");
let renderer: Renderer = Renderer::new();

loop {

    if is_key_pressed(KeyCode::Up) {
    game.player.guess.r = game.player.guess.r.saturating_add(5);
    }
    if is_key_pressed(KeyCode::Down) {
    game.player.guess.r =game.player.guess.r.saturating_sub(5);
}
if is_key_pressed(KeyCode::Right) {
    game.player.guess.y =game.player.guess.y.saturating_add(5);
}

if is_key_pressed(KeyCode::Left) {
    game.player.guess.y = game.player.guess.y.saturating_sub(5);
}

if is_key_pressed(KeyCode::Q) {
    game.player.guess.b =game.player.guess.b.saturating_add(5);
}

if is_key_pressed(KeyCode::A) {
    game.player.guess.b =game.player.guess.b.saturating_sub(5);
}
if is_key_pressed(KeyCode::Space) {
    game.submit_guess();
}
    renderer.render(&game);

    next_frame().await;
}

}
