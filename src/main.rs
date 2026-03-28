use macroquad::prelude::*;

use crate::game::GameManager;

mod assets;
mod entities;
mod game;
mod systems;
mod ui;

fn window_conf() -> Conf {
    return Conf {
        window_title: "Trail of Hunger".to_owned(),
        window_height: ui::WINDOW_HEIGHT,
        window_width: ui::WINDOW_WIDTH,
        window_resizable: false,
        ..Default::default()
    };
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_manager = GameManager::new().await;
    game_manager.debug_mode(true);

    loop {
        clear_background(BLACK);

        game_manager.update();
        game_manager.draw();

        next_frame().await
    }
}
