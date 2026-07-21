use macroquad::prelude::*;
use crate::gamestate::GameState;
use crate::gamestate::RoundResult;    
pub struct Renderer;
impl Renderer{
    pub fn new()->Self
    {
        Self
    }

     fn draw_background(&self){
        clear_background(DARKGRAY);
    }

    fn draw_target_color(&self,game:&GameState)
    {
        draw_rectangle(220.0,120.0,150.0,150.0, game.target.to_macroquad());
    }

    fn draw_player_color(&self, game:&GameState)
    {
        draw_rectangle(430.0,120.0,150.0,150.0, game.player.guess.to_macroquad());
    }

    fn draw_ui(&self, game: &GameState) {
    draw_text("Target Color",225.0,90.0,30.0,WHITE);
    draw_text("Your Color",420.0, 90.0,30.0, WHITE,);
    let score = format!("Score: {}", game.player.score);
    draw_text(&score, 20.0, 30.0, 30.0,YELLOW,);
    let best = format!("Best: {}", game.high_score);
    draw_text(&best,20.0,120.0,30.0,ORANGE,);
    let level = format!("Level: {}", game.level);
    draw_text(&level,20.0,60.0,30.0,GREEN,);
    let similarity = format!("Similarity: {:.1}%", game.similarity());
    draw_text(&similarity,285.0, 300.0,30.0, SKYBLUE,);
    draw_text("UP/DOWN : Red", 20.0, 500.0, 24.0, RED);
    draw_text("LEFT/RIGHT : Yellow", 20.0, 530.0, 24.0, YELLOW);
    draw_text("Q/A : Blue", 20.0, 560.0, 24.0, BLUE);
    draw_text("SPACE : Submit", 20.0, 590.0, 24.0, WHITE);
    draw_text(&game.message, 270.0, 330.0,35.0,ORANGE,);
    let time = format!("Time: {:.0}", game.time_left);
    draw_text(&time, 20.0, 150.0, 30.0, RED);
    draw_text(&game.network_message,250.0,370.0,28.0,GREEN);
}

fn draw_result_screen(&self, game: &GameState) {

    clear_background(BLACK);

    let title = match game.result {
        RoundResult::Win => "STAGE COMPLETE!",
        RoundResult::Fail => "STAGE FAILED!",
        RoundResult::TimeUp => "TIME UP!",
        RoundResult::Playing => "",
    };

    draw_text(title, 120.0, 120.0, 50.0, YELLOW);

    let similarity =
        format!("Similarity: {:.1}%", game.similarity());

    draw_text(&similarity, 120.0, 200.0, 35.0, WHITE);

    let score =
        format!("Score: {}", game.player.score);

    draw_text(&score, 120.0, 250.0, 35.0, GREEN);

    draw_text(
        "ENTER : Continue",
        120.0,
        360.0,
        30.0,
        LIGHTGRAY,
    );

    draw_text(
        "Esc : back",
        120.0,
        410.0,
        30.0,
        LIGHTGRAY,
    );
    if let Some(acc) = game.online_accuracy {

    draw_text(
        &format!("Server Accuracy: {:.2}%", acc),
        20.0,
        40.0,
        30.0,
        GREEN,
    );
}
}

pub fn render(&self, game: &GameState) {

    if game.result != RoundResult::Playing {

        self.draw_result_screen(game);

        return;

    }

    self.draw_background();

    self.draw_target_color(game);

    self.draw_player_color(game);

    self.draw_ui(game);
    
}
}