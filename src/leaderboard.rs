use crate::save::PlayerSave;
use macroquad::prelude::*;

pub struct Leaderboard;

impl Leaderboard {
    pub fn draw(players: &[PlayerSave]) {
        clear_background(BLACK);

        draw_text(
            "LEADERBOARD",
            180.0,
            60.0,
            40.0,
            YELLOW,
        );

        let mut y = 120.0;

        draw_text(
            "Name",
            80.0,
            y,
            30.0,
            GREEN,
        );

        draw_text(
            "Easy",
            240.0,
            y,
            30.0,
            GREEN,
        );

        draw_text(
            "Medium",
            360.0,
            y,
            30.0,
            GREEN,
        );

        draw_text(
            "Hard",
            520.0,
            y,
            30.0,
            GREEN,
        );

        y += 40.0;

        for player in players {
            draw_text(
                &player.name,
                80.0,
                y,
                28.0,
                WHITE,
            );

            draw_text(
                &player.easy.to_string(),
                250.0,
                y,
                28.0,
                WHITE,
            );

            draw_text(
                &player.medium.to_string(),
                380.0,
                y,
                28.0,
                WHITE,
            );

            draw_text(
                &player.hard.to_string(),
                530.0,
                y,
                28.0,
                WHITE,
            );

            y += 35.0;
        }
    }
}