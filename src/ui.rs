use macroquad::prelude::*;

use crate::gameplay_config::GamePlayConfig;
use crate::gameplay_config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::world::World;

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
    animals_total: usize,
    cfg: &GamePlayConfig,
) {
    let w = screen_width();
    let h = screen_height();
    let ui_scale = ((w / WINDOW_WIDTH as f32).min(h / WINDOW_HEIGHT as f32)).clamp(0.75, 1.6);

    // Top bar
    let bar_h = cfg.ui_top_bar_height;
    draw_rectangle(0.0, 0.0, w, bar_h, Color::new(0.0, 0.0, 0.0, 0.62));
    draw_line(0.0, bar_h, w, bar_h, 1.0, Color::new(1.0, 1.0, 1.0, 0.12));

    let pad_x = 12.0 * ui_scale;
    let pad_y = 10.0 * ui_scale;
    let font_size = (16.0 * ui_scale) as u16;
    let col_gap = 16.0 * ui_scale;
    let col_w = ((w - pad_x * 2.0 - col_gap) * 0.5).max(1.0);

    let col1_x = pad_x;
    let col2_x = pad_x + col_w + col_gap;
    let row1_y = pad_y + 20.0 * ui_scale;
    let bar_y = pad_y + 32.0 * ui_scale;
    let bar_w = col_w;
    let bar_h2 = (10.0 * ui_scale).clamp(6.0, 14.0);

    // Hunger
    let hunger_max = cfg.hunger_max.max(1) as f32;
    let hunger_pct = (hunger as f32 / hunger_max).clamp(0.0, 1.0);
    let hunger_label = format!("Hunger {}/{}", hunger.max(0), cfg.hunger_max);
    draw_text_ex(
        &hunger_label,
        col1_x,
        row1_y,
        TextParams {
            font: main_font,
            font_size,
            color: WHITE,
            ..Default::default()
        },
    );

    let mut hunger_color = if hunger_pct > 0.6 {
        Color::new(0.35, 0.85, 0.45, 1.0)
    } else if hunger_pct > 0.3 {
        Color::new(0.95, 0.85, 0.25, 1.0)
    } else {
        Color::new(0.95, 0.35, 0.25, 1.0)
    };
    if hunger_pct <= 0.25 {
        let t = get_time() as f32;
        hunger_color.a = (0.65 + 0.35 * (t * 6.0).sin()).clamp(0.2, 1.0);
    }
    draw_bar(col1_x, bar_y, bar_w, bar_h2, hunger_pct, hunger_color);

    // Animals
    let animals_total = animals_total.max(1);
    let animals_done = animals_total.saturating_sub(animals_remaining.min(animals_total));
    let animals_pct = animals_done as f32 / animals_total as f32;
    let animals_label = format!("Animals {}/{}", animals_remaining, animals_total);
    draw_text_ex(
        &animals_label,
        col2_x,
        row1_y,
        TextParams {
            font: main_font,
            font_size,
            color: WHITE,
            ..Default::default()
        },
    );
    draw_bar(
        col2_x,
        bar_y,
        bar_w,
        bar_h2,
        animals_pct,
        Color::new(0.55, 0.7, 0.95, 0.95),
    );

    if !paused {
        return;
    }

    let overlay = Color::new(0.0, 0.0, 0.0, 0.55);
    draw_rectangle(0.0, 0.0, w, h, overlay);

    draw_centered_text(
        "Game paused",
        h * 0.47,
        main_font,
        (32.0 * ui_scale) as u16,
        WHITE,
    );

    draw_centered_text(
        "Press \'Esc\' to resume.",
        h * 0.55,
        main_font,
        (16.0 * ui_scale) as u16,
        GRAY,
    );

    draw_centered_text(
        "Press \'Enter\' to level select",
        h * 0.6,
        main_font,
        (16.0 * ui_scale) as u16,
        GRAY,
    );
}

pub fn draw_hover_label(
    main_font: Option<&Font>,
    mouse: Vec2,
    world: &World,
    selected_nomads: usize,
    cfg: &GamePlayConfig,
) {
    if mouse.y <= cfg.ui_top_bar_height {
        return;
    }

    let Some(text) = hovered_label(mouse, world, selected_nomads, cfg) else {
        return;
    };

    let w = screen_width();
    let h = screen_height();
    let ui_scale = ((w / WINDOW_WIDTH as f32).min(h / WINDOW_HEIGHT as f32)).clamp(0.75, 1.6);

    let font_size = (16.0 * ui_scale) as u16;
    let padding_x = 6.0 * ui_scale;
    let padding_y = 4.0 * ui_scale;

    let dims = measure_text(&text, main_font, font_size, 1.0);

    let mut x = mouse.x + 14.0 * ui_scale;
    let mut y = mouse.y - 10.0 * ui_scale;

    let bg_w = dims.width + padding_x * 2.0;
    let bg_h = dims.height + padding_y * 2.0;

    // Keep tooltip on-screen.
    if x + bg_w > w {
        x = (w - bg_w).max(0.0);
    }
    if y - bg_h < 0.0 {
        y = bg_h;
    }

    let bg_x = x;
    let bg_y = y - bg_h;
    draw_rectangle(bg_x, bg_y, bg_w, bg_h, Color::new(0.0, 0.0, 0.0, 0.72));
    draw_rectangle_lines(bg_x, bg_y, bg_w, bg_h, 1.0, Color::new(1.0, 1.0, 1.0, 0.25));

    draw_text_ex(
        &text,
        bg_x + padding_x,
        bg_y + padding_y + dims.height,
        TextParams {
            font: main_font,
            font_size,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub fn draw_hint(main_font: Option<&Font>, hint: &crate::state::UiHint) {
    let w = screen_width();
    let h = screen_height();

    let t = (hint.ttl / hint.duration).clamp(0.0, 1.0);
    let alpha = if t < 0.25 { t / 0.25 } else { 1.0 };

    let ui_scale = ((w / WINDOW_WIDTH as f32).min(h / WINDOW_HEIGHT as f32)).clamp(0.75, 1.6);
    let font_size = (16.0 * ui_scale) as u16;
    let padding_x = 8.0 * ui_scale;
    let padding_y = 6.0 * ui_scale;

    let dims = measure_text(&hint.text, main_font, font_size, 1.0);
    let mut x = hint.pos.x - dims.width * 0.5;
    let mut y = hint.pos.y;

    // Keep on-screen.
    let bg_w = dims.width + padding_x * 2.0;
    let bg_h = dims.height + padding_y * 2.0;
    x = x.clamp(6.0, (w - bg_w - 6.0).max(6.0));
    y = y.clamp(bg_h + 6.0, (h - 6.0).max(bg_h + 6.0));

    let bg_x = x;
    let bg_y = y - bg_h;

    draw_rectangle(
        bg_x,
        bg_y,
        bg_w,
        bg_h,
        Color::new(0.0, 0.0, 0.0, 0.72 * alpha),
    );
    draw_rectangle_lines(
        bg_x,
        bg_y,
        bg_w,
        bg_h,
        1.0,
        Color::new(1.0, 1.0, 1.0, 0.22 * alpha),
    );

    draw_text_ex(
        &hint.text,
        bg_x + padding_x,
        bg_y + padding_y + dims.height,
        TextParams {
            font: main_font,
            font_size,
            color: Color::new(1.0, 1.0, 1.0, alpha),
            ..Default::default()
        },
    );
}

fn hovered_label(
    mouse: Vec2,
    world: &World,
    selected_nomads: usize,
    cfg: &GamePlayConfig,
) -> Option<String> {
    let mouse = vec2(mouse.x, mouse.y - cfg.ui_top_bar_height);
    let nomad_r = cfg.render_nomad_radius + 4.0;
    let animal_r = cfg.render_animal_radius + 4.0;
    let corpse_r = cfg.render_corpse_radius + 4.0;

    let nomad_r2 = nomad_r * nomad_r;
    let animal_r2 = animal_r * animal_r;
    let corpse_r2 = corpse_r * corpse_r;

    if world
        .nomads
        .iter()
        .any(|n| n.get_position().distance_squared(mouse) <= nomad_r2)
    {
        return Some("Nomad".to_owned());
    }

    if world
        .animals
        .iter()
        .any(|a| a.get_position().distance_squared(mouse) <= animal_r2)
    {
        if selected_nomads > 0 {
            return Some("Animal (Right click to hunt)".to_owned());
        }
        return Some("Animal".to_owned());
    }

    if world
        .corpses
        .iter()
        .any(|c| c.available && c.pos.distance_squared(mouse) <= corpse_r2)
    {
        if selected_nomads > 0 {
            return Some("Meat (Right click to eat)".to_owned());
        }
        return Some("Meat".to_owned());
    }

    None
}

fn draw_bar(x: f32, y: f32, w: f32, h: f32, pct: f32, fill: Color) {
    let pct = pct.clamp(0.0, 1.0);
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.0, 0.0, 0.35));
    draw_rectangle(x, y, w * pct, h, fill);
    draw_rectangle_lines(x, y, w, h, 1.0, Color::new(1.0, 1.0, 1.0, 0.12));
}

pub fn draw_game_over(main_font: Option<&Font>) {
    let w = screen_width();
    let h = screen_height();
    let ui_scale = ((w / WINDOW_WIDTH as f32).min(h / WINDOW_HEIGHT as f32)).clamp(0.75, 1.6);

    clear_background(BLACK);

    draw_centered_text(
        "Game Over",
        h * 0.46,
        main_font,
        (32.0 * ui_scale) as u16,
        RED,
    );
    draw_centered_text(
        "Press \'Enter\' to retry",
        h * 0.56,
        main_font,
        (18.0 * ui_scale) as u16,
        GRAY,
    );
    draw_centered_text(
        "Press \'Esc\' to return to menu",
        h * 0.62,
        main_font,
        (16.0 * ui_scale) as u16,
        GRAY,
    );
}

pub fn draw_level_complete_overlay(
    main_font: Option<&Font>,
    level: i32,
    seconds_left: f32,
    is_final: bool,
) {
    let w = screen_width();
    let h = screen_height();
    let ui_scale = ((w / WINDOW_WIDTH as f32).min(h / WINDOW_HEIGHT as f32)).clamp(0.75, 1.6);

    let overlay = Color::new(0.0, 0.0, 0.0, 0.55);
    draw_rectangle(0.0, 0.0, w, h, overlay);

    if is_final {
        draw_centered_text(
            "You win!",
            h * 0.47,
            main_font,
            (32.0 * ui_scale) as u16,
            YELLOW,
        );
        draw_centered_text(
            "Returning to level select...",
            h * 0.55,
            main_font,
            (16.0 * ui_scale) as u16,
            GRAY,
        );
        return;
    }

    let secs = seconds_left.ceil().max(0.0) as i32;
    draw_centered_text(
        &format!("Level {} completed", level),
        h * 0.47,
        main_font,
        (32.0 * ui_scale) as u16,
        YELLOW,
    );
    draw_centered_text(
        &format!("Next level in {}", secs),
        h * 0.55,
        main_font,
        (16.0 * ui_scale) as u16,
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
