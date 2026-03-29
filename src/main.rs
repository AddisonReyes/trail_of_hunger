use crate::game::GameManager;
use crate::gameplay_config::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use crate::input::gather_input;
use macroquad::prelude::*;

mod assets;
mod entities;
mod game;
mod gameplay_config;
mod input;
mod levels;
mod render;
mod state;
mod systems;
mod ui;
mod world;

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_manager = GameManager::new().await;
    game_manager.debug_mode(true);

    loop {
        clear_background(BLACK);

        let input = gather_input();

        game_manager.update(&input);
        game_manager.draw();

        next_frame().await
    }
}
