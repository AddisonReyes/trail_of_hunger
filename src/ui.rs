use macroquad::prelude::*;

use crate::gameplay_config::{WINDOW_HEIGHT, WINDOW_WIDTH};

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

pub fn draw_level_select(
    main_font: Option<&Font>,
    selected_level: usize,
    unlocked_max_level: usize,
    max_levels: usize,
) {
    clear_background(BLACK);

    let w = screen_width();
    let h = screen_height();
    let ui_scale = ((w / WINDOW_WIDTH as f32).min(h / WINDOW_HEIGHT as f32)).clamp(0.75, 1.6);

    draw_centered_text(
        "Select Level",
        h * 0.26,
        main_font,
        (32.0 * ui_scale) as u16,
        WHITE,
    );

    // Carousel layout: selected level is centered, neighbors to the sides.
    let cx = w * 0.5;
    let base_y = h * 0.50;
    let spacing = (w * 0.09).clamp(44.0, 80.0);
    let window = ((w / spacing).floor() as i32 / 2).clamp(2, 6); // levels shown on each side

    let sel = selected_level as i32;
    let min_lvl = 1_i32.max(sel - window);
    let max_lvl = (max_levels as i32).min(sel + window);

    for level in min_lvl..=max_lvl {
        let offset = level - sel;
        let a = (offset.abs() as f32 / window as f32).clamp(0.0, 1.0);

        let x = cx + offset as f32 * spacing;
        let y = base_y + a * (8.0 * ui_scale);

        let scale = (1.0 - a * 0.45).clamp(0.55, 1.0);
        let font_size = (54.0 * ui_scale * scale) as u16;
        let alpha = (1.0 - a * 0.65).clamp(0.25, 1.0);

        let is_unlocked = (level as usize) <= unlocked_max_level;

        let color = if offset == 0 {
            if is_unlocked {
                YELLOW
            } else {
                Color::new(0.85, 0.35, 0.35, 1.0)
            }
        } else if is_unlocked {
            Color::new(0.8, 0.8, 0.8, alpha)
        } else {
            Color::new(0.45, 0.45, 0.45, (alpha * 0.75).clamp(0.15, 1.0))
        };

        let text = format!("{}", level);
        let dims = measure_text(&text, main_font, font_size, 1.0);
        draw_text_ex(
            &text,
            x - dims.width * 0.5,
            y,
            TextParams {
                font: main_font,
                font_size,
                color,
                ..Default::default()
            },
        );

        if !is_unlocked {
            let lock = "LOCKED";
            let lock_size = (12.0 * ui_scale * scale).max(10.0) as u16;
            let lock_dims = measure_text(lock, main_font, lock_size, 1.0);
            draw_text_ex(
                lock,
                x - lock_dims.width * 0.5,
                y + (18.0 * ui_scale),
                TextParams {
                    font: main_font,
                    font_size: lock_size,
                    color: Color::new(0.55, 0.55, 0.55, (alpha * 0.8).clamp(0.15, 1.0)),
                    ..Default::default()
                },
            );
        }
    }

    draw_centered_text(
        "Mouse wheel: change level",
        h * 0.69,
        main_font,
        (16.0 * ui_scale) as u16,
        GRAY,
    );
    draw_centered_text(
        "Press \'Enter\' to play (unlocked only)",
        h * 0.76,
        main_font,
        (18.0 * ui_scale) as u16,
        GRAY,
    );
    draw_centered_text(
        "Press \'Esc\' to go back",
        h * 0.82,
        main_font,
        (16.0 * ui_scale) as u16,
        GRAY,
    );
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
    let x = screen_width() * 0.5 - dims.width * 0.5;

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
