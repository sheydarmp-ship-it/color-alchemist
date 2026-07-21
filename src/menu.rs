use crate::gamestate::Difficulty;
use macroquad::prelude::*;
use crate::save::SaveData;
pub struct Menu {
    pub player_name: String,
    pub selected: usize,
    pub difficulty: Difficulty,
    pub step: MenuStep,
    pub mode: GameMode,
    pub backspace_timer: f32,
    pub host_game: bool,
}
#[derive(PartialEq)]
pub enum MenuStep {
    Name,
    Mode,
    Network,
    Difficulty,
    Ready,
    Leaderboard,
}

#[derive(PartialEq)]
pub enum GameMode {
    Single,
    Multiplayer,
}

impl Menu {
    
pub fn new() -> Self {
    Self {
        player_name: String::new(),
        selected: 0,
        difficulty: Difficulty::Easy,
        step: MenuStep::Name,
        mode: GameMode::Single,
        backspace_timer: 0.0,
        host_game: true,
    }
}

pub fn update(&mut self) {
    match self.step {
        MenuStep::Name => self.update_name(),
        MenuStep::Mode => self.update_mode(),
        MenuStep::Network => self.update_network(),
        MenuStep::Difficulty => self.update_difficulty(),
        MenuStep::Ready => self.update_ready(),
        MenuStep::Leaderboard => {}
    }

    self.handle_escape();
}

fn update_name(&mut self) {
    while let Some(c) = get_char_pressed() {
        self.player_name.push(c);
    }

    if is_key_pressed(KeyCode::Backspace) {
        self.player_name.pop();
    }

    if is_key_down(KeyCode::Backspace) {
        self.backspace_timer += get_frame_time();

        if self.backspace_timer >= 0.08 {
            self.player_name.pop();
            self.backspace_timer = 0.0;
        }
    } else {
        self.backspace_timer = 0.0;
    }

    if is_key_pressed(KeyCode::Enter) && !self.player_name.is_empty() {
        self.step = MenuStep::Mode;
        self.reset_selection();
    }
}

fn update_mode(&mut self) {
    if is_key_pressed(KeyCode::Down) {
    if self.selected < 2 {
        self.selected += 1;
    }
}

    if is_key_pressed(KeyCode::Up) {
    if self.selected > 0 {
        self.selected -= 1;
    }
}

    if is_key_pressed(KeyCode::Enter) {
    match self.selected {
        0 => {
            self.mode = GameMode::Single;
            self.reset_selection();
            self.step = MenuStep::Difficulty;
        }

        1 => {
            self.mode = GameMode::Multiplayer;
            self.reset_selection();
            self.step = MenuStep::Network;
        }

        2 => {
            self.reset_selection();
            self.step = MenuStep::Leaderboard;
        }

        _ => {}
    }
}
}

fn update_network(&mut self) {

    if is_key_pressed(KeyCode::Up) ||
       is_key_pressed(KeyCode::Down)
    {
        self.selected = 1 - self.selected;
    }

    if is_key_pressed(KeyCode::Enter) {

        self.host_game = self.selected == 0;

        self.step = MenuStep::Difficulty;

        self.reset_selection();
    }
}

fn update_difficulty(&mut self) {
    if is_key_pressed(KeyCode::Down) && self.selected < 2 {
        self.selected += 1;
    }

    if is_key_pressed(KeyCode::Up) && self.selected > 0 {
        self.selected -= 1;
    }

    if is_key_pressed(KeyCode::Enter) {
        self.difficulty = match self.selected {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            _ => Difficulty::Hard,
        };

        self.step = MenuStep::Ready;
    }
}

fn update_ready(&mut self) {
    // شروع بازی در main انجام می‌شود
}

fn handle_escape(&mut self) {
    if !is_key_pressed(KeyCode::Escape) {
        return;
    }

    match self.step {
        MenuStep::Difficulty => self.step = MenuStep::Mode,
        MenuStep::Mode => self.step = MenuStep::Name,
        MenuStep::Leaderboard => {self.step = MenuStep::Mode;}

        _ => {}
    }
}

pub fn back_to_difficulty(&mut self) {
    self.step = MenuStep::Difficulty;
    self.reset_selection();
}

pub fn reset_selection(&mut self) {
    self.selected = 0;
}

pub fn is_ready(&self) -> bool {
    self.step == MenuStep::Ready
}
pub fn draw(&self) {
    clear_background(BLACK);

    match self.step {
        MenuStep::Name => self.draw_name(),
        MenuStep::Mode => self.draw_mode(),
        MenuStep::Network => self.draw_network(),
        MenuStep::Difficulty => self.draw_difficulty(),
        MenuStep::Ready => self.draw_ready(),
        MenuStep::Leaderboard => self.draw_leaderboard(),
    }
}
fn draw_name(&self) {
    draw_text("COLOR ALCHEMIST",120.0,80.0,45.0,YELLOW);
    draw_text("Enter Your Name",150.0,170.0,35.0,WHITE);

    let name = format!("{}_", self.player_name);
    draw_text(&name,170.0,240.0,40.0,GREEN);

    draw_text("ENTER : Continue",150.0,330.0,28.0,LIGHTGRAY);
    draw_text("BACKSPACE : Delete",130.0,370.0,28.0,LIGHTGRAY);
}

fn draw_mode(&self) {
    draw_text("Choose Game Mode",120.0,120.0,40.0,YELLOW);

    let single =
        if self.selected == 0 { GREEN } else { WHITE };

    let multi =
        if self.selected == 1 { GREEN } else { WHITE };
    let leader =
         if self.selected == 2 { GREEN } else { WHITE };


    draw_text("Single Player",170.0,220.0,35.0,single);
    draw_text("Multiplayer",170.0,280.0,35.0,multi);
    draw_text("Leaderboard",170.0,340.0,35.0,leader);
    draw_text("UP/DOWN : Move",120.0,380.0,28.0,LIGHTGRAY);
    draw_text("ENTER : Select",120.0,420.0,28.0,LIGHTGRAY);
}
fn draw_difficulty(&self) {
    draw_text("Choose Difficulty",120.0,120.0,40.0,YELLOW);

    let easy_color =
        if self.selected == 0 { GREEN } else { WHITE };

    let medium_color =
        if self.selected == 1 { GREEN } else { WHITE };

    let hard_color =
        if self.selected == 2 { GREEN } else { WHITE };

    draw_text("Easy",200.0,220.0,35.0,easy_color);
    draw_text("Medium",200.0,280.0,35.0,medium_color);
    draw_text("Hard",200.0,340.0,35.0,hard_color);

    draw_text("UP/DOWN : Move",120.0,430.0,28.0,LIGHTGRAY);
    draw_text("ENTER : Select",120.0,470.0,28.0,LIGHTGRAY);
}
fn draw_network(&self) {
    let host =
    if self.selected == 0 { GREEN } else { WHITE };

let join =
    if self.selected == 1 { GREEN } else { WHITE };

draw_text("Network",170.0,100.0,45.0,YELLOW);

draw_text("Host Game",170.0,220.0,35.0,host);

draw_text("Join Game",170.0,280.0,35.0,join);

draw_text(
    "UP/DOWN : Select",
    100.0,
    400.0,
    28.0,
    LIGHTGRAY,
);

draw_text(
    "ENTER : Continue",
    100.0,
    440.0,
    28.0,
    LIGHTGRAY,
);}
fn draw_ready(&self) {
    draw_text("Ready to Start!",150.0,140.0,45.0,GREEN);

    let player =
        format!("Player : {}", self.player_name);
    draw_text(&player,150.0,220.0,35.0,WHITE);

    let mode = match self.mode {
        GameMode::Single => "Single Player",
        GameMode::Multiplayer => "Multiplayer",
    };

    let mode_text = format!("Mode : {}", mode);
    draw_text(&mode_text,150.0,270.0,35.0,WHITE);

    let difficulty = match self.difficulty {
        Difficulty::Easy => "Easy",
        Difficulty::Medium => "Medium",
        Difficulty::Hard => "Hard",
    };

    let diff =
    format!("Difficulty : {}", difficulty);

draw_text(
    &diff,
    150.0,
    320.0,
    35.0,
    WHITE,
);
}
fn draw_leaderboard(&self) {
    clear_background(BLACK);

    draw_text("LEADERBOARD",170.0,70.0,45.0,YELLOW);

    let mut save = SaveData::load();

    save.players.sort_by(|a, b| {
        let sa = a.easy + a.medium + a.hard;
        let sb = b.easy + b.medium + b.hard;

        sb.cmp(&sa)
    });

    for (i, player) in save.players.iter().enumerate() {

        let total =
            player.easy + player.medium + player.hard;

        let line = format!( "{}. {}   {}",i + 1,player.name, total,);

        draw_text(&line,120.0, 130.0 + i as f32 * 35.0,30.0,WHITE);
    }

    draw_text("ESC : Back", 120.0, 520.0,28.0,LIGHTGRAY);
}
}
