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
            selected_level: 1,
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

    draw_centered_text(
        "Trail of Hunger",
        (WINDOW_HEIGHT as f32 / 2.0) - 20.0,
        main_font,
        32,
        WHITE,
    );

    draw_centered_text(
        "Press \'Enter\' to start",
        (WINDOW_HEIGHT as f32 / 2.0) + 20.0,
        main_font,
        16,
        GRAY,
    );
}

pub fn draw_level_select(main_font: Option<&Font>, selected_level: usize) {
    clear_background(BLACK);

    draw_centered_text("Select Level", 80.0, main_font, 32, WHITE);

    let starting_x = (WINDOW_WIDTH / 5) as f32;
    for level in 1..11 {
        let text = format!("{}", level);
        let x = starting_x + 32.0 * level as f32;
        let y = 140.0;

        if level == selected_level {
            draw_text_ex(
                &text,
                x,
                y,
                TextParams {
                    font: main_font,
                    font_size: 36,
                    color: YELLOW,
                    ..Default::default()
                },
            );
        } else {
            draw_text_ex(
                &text,
                x,
                y,
                TextParams {
                    font: main_font,
                    font_size: 32,
                    color: GRAY,
                    ..Default::default()
                },
            );
        }
    }

    draw_centered_text("Use arrows or WASD to change", 240.0, main_font, 16, GRAY);
    draw_centered_text("Press \'Enter\' to play", 280.0, main_font, 32, GRAY);
}

pub fn draw_ingame_ui(main_font: Option<&Font>) {
    // No limpiamos pantalla aquí

    draw_text_ex(
        "Hunger: 75",
        20.0,
        30.0,
        TextParams {
            font: main_font,
            font_size: 16,
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
            font_size: 16,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub fn draw_game_over(main_font: Option<&Font>) {
    clear_background(BLACK);

    draw_centered_text(
        "Game Over",
        (WINDOW_HEIGHT as f32 / 2.0) - 20.0,
        main_font,
        32,
        RED,
    );
    draw_centered_text(
        "Press \'Enter\' to return to menu",
        (WINDOW_HEIGHT as f32 / 2.0) + 20.0,
        main_font,
        16,
        GRAY,
    );
}

fn draw_centered_text(text: &str, y: f32, font: Option<&Font>, font_size: u16, text_color: Color) {
    let dims = measure_text(text, font, font_size, 1.0);
    let x = WINDOW_WIDTH as f32 / 2.0 - dims.width / 2.0;

    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font,
            font_size,
            color: text_color,
            ..Default::default()
        },
    );
}
