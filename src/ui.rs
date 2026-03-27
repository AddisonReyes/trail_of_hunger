use macroquad::prelude::*;

pub const WINDOW_HEIGHT: i32 = 360;
pub const WINDOW_WIDTH: i32 = 640;

#[derive(Clone, Copy, PartialEq)]
pub enum View {
    Menu,
    LevelSelect,
    InGame,
    GameOver,
}

pub struct UiState {
    pub current_view: View,
    pub selected_level: usize,
    font: Option<Font>,
}

impl UiState {
    pub fn new(main_font: Option<Font>) -> Self {
        Self {
            current_view: View::Menu,
            selected_level: 0,
            font: main_font,
        }
    }

    pub fn update(&mut self) {
        match self.current_view {
            View::Menu => self.update_menu(),
            View::LevelSelect => self.update_level_select(),
            View::InGame => self.update_ingame(),
            View::GameOver => self.update_game_over(),
        }
    }

    pub fn draw(&self) {
        match self.current_view {
            View::Menu => draw_menu(self.font.as_ref()),
            View::LevelSelect => draw_level_select(self.font.as_ref(), self.selected_level),
            View::InGame => draw_ingame_ui(self.font.as_ref()),
            View::GameOver => draw_game_over(self.font.as_ref()),
        }
    }

    fn update_menu(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            self.current_view = View::LevelSelect;
        }
    }

    fn update_level_select(&mut self) {
        if is_key_pressed(KeyCode::Right) {
            self.selected_level += 1;
        }

        if is_key_pressed(KeyCode::Left) && self.selected_level > 1 {
            self.selected_level -= 1;
        }

        if is_key_pressed(KeyCode::Enter) {
            self.current_view = View::InGame;
        }

        if is_key_pressed(KeyCode::Escape) {
            self.current_view = View::Menu;
        }
    }

    fn update_ingame(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.current_view = View::GameOver;
        }
    }

    fn update_game_over(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            self.current_view = View::Menu;
        }
    }
}

pub fn draw_menu(main_font: Option<&Font>) {
    clear_background(BLACK);

    draw_text_ex(
        "Trail of Hunger",
        100.0,
        100.0,
        TextParams {
            font: main_font,
            font_size: 40,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(
        "Press ENTER to start",
        100.0,
        160.0,
        TextParams {
            font: main_font,
            font_size: 24,
            color: GRAY,
            ..Default::default()
        },
    );
}

pub fn draw_level_select(main_font: Option<&Font>, selected_level: usize) {
    clear_background(DARKGRAY);

    draw_text_ex(
        "Select Level",
        100.0,
        80.0,
        TextParams {
            font: main_font,
            font_size: 32,
            color: WHITE,
            ..Default::default()
        },
    );

    let text = format!("Level: {}", selected_level);

    draw_text_ex(
        &text,
        100.0,
        140.0,
        TextParams {
            font: main_font,
            font_size: 28,
            color: YELLOW,
            ..Default::default()
        },
    );

    draw_text_ex(
        "← → to change",
        100.0,
        200.0,
        TextParams {
            font: main_font,
            font_size: 20,
            color: GRAY,
            ..Default::default()
        },
    );

    draw_text_ex(
        "ENTER to play",
        100.0,
        230.0,
        TextParams {
            font: main_font,
            font_size: 20,
            color: GRAY,
            ..Default::default()
        },
    );
}

pub fn draw_ingame_ui(main_font: Option<&Font>) {
    // No limpiamos pantalla aquí

    draw_text_ex(
        "Hunger: 75",
        20.0,
        30.0,
        TextParams {
            font: main_font,
            font_size: 24,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(
        "Animals: 12",
        20.0,
        60.0,
        TextParams {
            font: main_font,
            font_size: 24,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub fn draw_game_over(main_font: Option<&Font>) {
    clear_background(BLACK);

    draw_text_ex(
        "Game Over",
        100.0,
        120.0,
        TextParams {
            font: main_font,
            font_size: 40,
            color: RED,
            ..Default::default()
        },
    );

    draw_text_ex(
        "Press ENTER to return to menu",
        100.0,
        180.0,
        TextParams {
            font: main_font,
            font_size: 24,
            color: GRAY,
            ..Default::default()
        },
    );
}
