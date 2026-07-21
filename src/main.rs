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
use multiplayer::client::Client;
use menu::GameMode;
use crate::multiplayer::protocol::Packet;
use std::thread;
use tokio::runtime::Runtime;
use multiplayer::server::Server;
 use crate::menu::MenuStep;
#[macroquad::main("Color Alchemist")]
async fn main() {
    let mut menu = Menu::new();
    let mut game: Option<GameState> = None;
    let mut save = SaveData::load();
    let renderer = Renderer::new();
    let mut show_leaderboard = false;
    let mut server_started = false;
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
    &mut server_started,
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
    server_started: &mut bool,
) { 
    menu.update();
    menu.draw();

    if menu.is_ready() {
        let save = SaveData::load();

        let high_score = save.get_highscore(
            &menu.player_name,
            menu.difficulty,
        );

       let mut state = GameState::new(
    &menu.player_name,
    menu.difficulty,
    high_score,
);

    if menu.mode == GameMode::Multiplayer {
 if menu.host_game && !*server_started {

        thread::spawn(|| {

        let rt = Runtime::new().unwrap();

        rt.block_on(async {

            Server::run("127.0.0.1:7878");

        });

    });
     *server_started = true;
    }
let mut client = Client::connect("127.0.0.1:7878");
let name = menu.player_name.trim().to_string();
client.send(Packet::Join {
    name,
});

let packet = client.receive();

match packet {

    Packet::TargetColor { r, g, b } => {

        state.target.r = r;
        state.target.y = g;
        state.target.b = b;

    }

    _ => {}

}

state.enable_online(client);
*game = Some(state);
return;
}
menu.step = MenuStep::Name;
menu.player_name.clear();
*game = Some(state);
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

    if game.is_online() {

    let client = game.client.as_mut().unwrap();

    client.send(
        Packet::Guess {

            r: game.player.guess.r,
            g: game.player.guess.y,
            b: game.player.guess.b,
        }
    );

    let reply = client.receive();

match reply {

    Packet::RoundResult { accuracy, win } => {

    game.online_accuracy = Some(accuracy);

    game.network_message =
        format!("Accuracy = {:.2}%", accuracy);

    if win {

        game.player.add_score(
            game.difficulty.score()
        );

        game.result = RoundResult::Win;

        game.message =
            "Level Complete!".to_string();

    } else {

        game.result = RoundResult::Fail;

        game.message =
            format!("Try Again! ({:.2}%)", accuracy);

    }

}

    _ => {}

}

} else {

    game.submit_guess();

}
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