use macroquad::prelude::*;

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
    loop {
        clear_background(WHITE);

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
