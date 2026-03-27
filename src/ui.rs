use macroquad::prelude::*;

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
}

impl UiState {
    pub fn new() -> Self {
        Self {
            current_view: View::Menu,
            selected_level: 0,
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
            View::Menu => draw_menu(),
            View::LevelSelect => draw_level_select(self.selected_level),
            View::InGame => draw_ingame_ui(),
            View::GameOver => draw_game_over(),
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

fn draw_menu() {
    clear_background(BLACK);

    draw_text("Trail of Hunger", 100.0, 100.0, 40.0, WHITE);
    draw_text("Press ENTER to start", 100.0, 160.0, 24.0, GRAY);
}

fn draw_level_select(selected_level: usize) {
    clear_background(DARKGRAY);

    draw_text("Select Level", 100.0, 80.0, 32.0, WHITE);

    let text = format!("Level: {}", selected_level);
    draw_text(&text, 100.0, 140.0, 28.0, YELLOW);

    draw_text("← → to change", 100.0, 200.0, 20.0, GRAY);
    draw_text("ENTER to play", 100.0, 230.0, 20.0, GRAY);
}

fn draw_ingame_ui() {
    // No limpiamos pantalla aquí, eso lo hace el juego

    draw_text("Hunger: 75", 20.0, 30.0, 24.0, WHITE);
    draw_text("Animals: 12", 20.0, 60.0, 24.0, WHITE);
}

fn draw_game_over() {
    clear_background(BLACK);

    draw_text("Game Over", 100.0, 120.0, 40.0, RED);
    draw_text("Press ENTER to return to menu", 100.0, 180.0, 24.0, GRAY);
}
