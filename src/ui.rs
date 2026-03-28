use macroquad::prelude::*;

pub const WINDOW_HEIGHT: i32 = 360;
pub const WINDOW_WIDTH: i32 = 640;

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

    draw_centered_text("Use Left/Right to change", 240.0, main_font, 16, GRAY);
    draw_centered_text("Press \'Enter\' to play", 280.0, main_font, 32, GRAY);
}

pub fn draw_ingame_ui(
    main_font: Option<&Font>,
    paused: bool,
    hunger: i32,
    animals_remaining: usize,
) {
    draw_text_ex(
        &format!("Hunger: {}", hunger),
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
        &format!("Animals: {}", animals_remaining),
        20.0,
        60.0,
        TextParams {
            font: main_font,
            font_size: 16,
            color: WHITE,
            ..Default::default()
        },
    );

    if !paused {
        return;
    }

    draw_centered_text(
        "Game paused",
        (WINDOW_HEIGHT as f32 / 2.0) - 20.0,
        main_font,
        32,
        WHITE,
    );

    draw_centered_text(
        "Press \'Esc\' to resume.",
        (WINDOW_HEIGHT as f32 / 2.0) + 20.0,
        main_font,
        16,
        GRAY,
    );

    draw_centered_text(
        "Press \'Enter\' to level select",
        (WINDOW_HEIGHT as f32 / 2.0) + 40.0,
        main_font,
        16,
        GRAY,
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

pub fn draw_level_complete_overlay(
    main_font: Option<&Font>,
    level: i32,
    seconds_left: f32,
    is_final: bool,
) {
    let overlay = Color::new(0.0, 0.0, 0.0, 0.55);
    draw_rectangle(0.0, 0.0, WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, overlay);

    if is_final {
        draw_centered_text(
            "You win!",
            (WINDOW_HEIGHT as f32 / 2.0) - 20.0,
            main_font,
            32,
            YELLOW,
        );
        draw_centered_text(
            "Returning to level select...",
            (WINDOW_HEIGHT as f32 / 2.0) + 20.0,
            main_font,
            16,
            GRAY,
        );
        return;
    }

    let secs = seconds_left.ceil().max(0.0) as i32;
    draw_centered_text(
        &format!("Level {} complete", level),
        (WINDOW_HEIGHT as f32 / 2.0) - 20.0,
        main_font,
        32,
        YELLOW,
    );
    draw_centered_text(
        &format!("Next level in {}", secs),
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
