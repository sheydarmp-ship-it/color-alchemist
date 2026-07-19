mod multiplayer;
mod color;
mod gamestate;
mod menu;
mod player;
mod renderer;
mod save;
mod leaderboard;
use gamestate::{GameState, RoundResult};
use macroquad::prelude::*;
use menu::Menu;
use renderer::Renderer;
use save::SaveData;
use leaderboard::Leaderboard;
use multiplayer::MultiplayerState;
#[macroquad::main("Color Alchemist")]
async fn main() {
    let mut menu = Menu::new();
    let mut game: Option<GameState> = None;
    let mut save = SaveData::load();
    let renderer = Renderer::new();
    let mut show_leaderboard = false;
    loop {
        let mut back_to_menu = false;
        if is_key_pressed(KeyCode::L) {
        show_leaderboard = !show_leaderboard;
    }

        if show_leaderboard {
        Leaderboard::draw(&save.get_players());

    if is_key_pressed(KeyCode::Escape) {
        show_leaderboard = false;
    }

    next_frame().await;
    continue;
}
        if let Some(current_game) = game.as_mut() {
            update_game(
                current_game,
                &renderer,
                &mut back_to_menu,
                &mut save,
            );
        } else {
            update_menu(
                &mut menu,
                &mut game,
            
);
        }

        if back_to_menu {
            menu.back_to_difficulty();
            game = None;
             while get_char_pressed().is_some() {}
        }

        next_frame().await;
    }
}
fn update_game(
    game: &mut GameState,
    renderer: &Renderer,
    back_to_menu: &mut bool,
    save: &mut SaveData,
) {
    handle_game_input(game);

    game.update(get_frame_time());

    handle_result(game, back_to_menu,save);

    renderer.render(game);
}
fn update_menu(
    menu: &mut Menu,
    game: &mut Option<GameState>,
) {
    menu.update();
    menu.draw();

    if menu.is_ready() {

        let save = SaveData::load();

        let high_score = save.get_highscore(
            &menu.player_name,
            menu.difficulty,
        );

        *game = Some(GameState::new(
            &menu.player_name,
            menu.difficulty,
            high_score,
        ));
    }
}
fn handle_game_input(game: &mut GameState) {
    if game.result != RoundResult::Playing {
        return;
    }

    update_red(game);
    update_yellow(game);
    update_blue(game);

    if is_key_pressed(KeyCode::Space) {
        game.submit_guess();
    }
}
fn update_red(game: &mut GameState) {
    if is_key_pressed(KeyCode::Up) {
        game.player.guess.r =
            game.player.guess.r.saturating_add(5);
    }

    if is_key_pressed(KeyCode::Down) {
        game.player.guess.r =
            game.player.guess.r.saturating_sub(5);
    }
}
fn update_yellow(game: &mut GameState) {
    if is_key_pressed(KeyCode::Right) {
        game.player.guess.y =
            game.player.guess.y.saturating_add(5);
    }

    if is_key_pressed(KeyCode::Left) {
        game.player.guess.y =
            game.player.guess.y.saturating_sub(5);
    }
}
fn update_blue(game: &mut GameState) {
    if is_key_pressed(KeyCode::Q) {
        game.player.guess.b =
            game.player.guess.b.saturating_add(5);
    }

    if is_key_pressed(KeyCode::A) {
        game.player.guess.b =
            game.player.guess.b.saturating_sub(5);
    }
}
fn handle_result(
    game: &mut GameState,
    back_to_menu: &mut bool,
    save: &mut SaveData,
) {
    if game.result == RoundResult::Playing {
        return;
    }

    if is_key_pressed(KeyCode::Enter) {
        match game.result {
            RoundResult::Win => {
                save.update_highscore(
                    &game.player.name,
                    game.difficulty,
                    game.player.score,
                );

                game.next_level();
            }

            RoundResult::Fail | RoundResult::TimeUp => {
                save.update_highscore(
                    &game.player.name,
                    game.difficulty,
                    game.player.score,
                );

                game.restart_round();
            }

            RoundResult::Playing => {}
        }
    }

    if is_key_pressed(KeyCode::Escape) {
        save.update_highscore(
            &game.player.name,
            game.difficulty,
            game.player.score,
        );

        *back_to_menu = true;
    }
}