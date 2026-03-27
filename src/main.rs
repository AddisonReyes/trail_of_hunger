use macroquad::prelude::*;

use crate::game::GameManager;

mod assets;
mod entities;
mod game;
mod systems;
mod ui;

const WINDOW_HEIGHT: i32 = 180;
const WINDOW_WIDTH: i32 = 320;

fn window_conf() -> Conf {
    return Conf {
        window_title: "Trail of Hunger".to_owned(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        ..Default::default()
    };
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_manager = GameManager::new();
    game_manager.debug_mode(true);
    game_manager.print_data();

    loop {
        clear_background(WHITE);

        game_manager.update();
        game_manager.draw();

        draw_text(
            "Hello world",
            (WINDOW_WIDTH / 2) as f32 - 50.0,
            (WINDOW_HEIGHT / 2) as f32,
            20.0,
            DARKGRAY,
        );

        next_frame().await
    }
}
