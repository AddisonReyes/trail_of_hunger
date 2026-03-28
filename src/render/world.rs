use macroquad::prelude::*;

use crate::gameplay_config::GamePlayConfig;
use crate::gameplay_config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::state::{CommandFeedback, CommandTarget, SelectionBox};
use crate::world::World;

pub fn draw_world(
    world: &World,
    selection_box: Option<SelectionBox>,
    last_command: Option<CommandFeedback>,
    tuning: &GamePlayConfig,
    main_font: Option<&Font>,
    level: i32,
) {
    clear_background(color_u8!(33, 104, 58, 255));

    let offset_y = tuning.ui_top_bar_height;
    let to_screen = |p: Vec2| vec2(p.x, p.y + offset_y);

    if level == 1 {
        draw_level1_tutorial(world, main_font, offset_y);
    }

    for a in &world.animals {
        let pos = to_screen(a.get_position());
        draw_circle(
            pos.x,
            pos.y,
            tuning.render_animal_radius,
            color_u8!(196, 160, 106, 255),
        );
    }

    for c in &world.corpses {
        let pos = to_screen(c.pos);
        let color = if c.available {
            color_u8!(180, 64, 48, 255)
        } else {
            color_u8!(90, 70, 70, 255)
        };
        draw_circle(pos.x, pos.y, tuning.render_corpse_radius, color);
    }

    for s in &world.spears {
        let sp = to_screen(s.pos);
        let tail = sp - s.vel.normalize_or_zero() * 8.0;
        draw_line(
            tail.x,
            tail.y,
            sp.x,
            sp.y,
            2.0,
            color_u8!(230, 230, 230, 255),
        );
    }

    if let Some(cmd) = last_command {
        let click = to_screen(cmd.click_pos);
        draw_circle(click.x, click.y, 3.5, WHITE);

        let end = match cmd.target {
            CommandTarget::Point(p) => p,
            CommandTarget::Animal(id) => world
                .animals
                .iter()
                .find(|a| a.id() == id)
                .map(|a| a.get_position())
                .unwrap_or(cmd.click_pos),
            CommandTarget::Corpse(id) => world
                .corpses
                .iter()
                .find(|c| c.id == id)
                .map(|c| c.pos)
                .unwrap_or(cmd.click_pos),
        };

        let end = to_screen(end);

        for n in &world.nomads {
            if n.order_id() == cmd.id && n.order() != crate::entities::NomadOrder::Idle {
                let p = to_screen(n.get_position());
                draw_line(p.x, p.y, end.x, end.y, 1.0, color_u8!(255, 255, 255, 120));
            }
        }
    }

    for n in &world.nomads {
        let pos = to_screen(n.get_position());
        let base = color_u8!(215, 226, 255, 255);
        draw_circle(pos.x, pos.y, tuning.render_nomad_radius, base);
        if n.is_selected() {
            draw_circle_lines(pos.x, pos.y, tuning.render_nomad_radius + 3.0, 2.0, YELLOW);
        }
    }

    // Selection rectangle (RTS-style).
    if is_mouse_button_down(MouseButton::Left)
        && let Some(b) = selection_box
    {
        let drag = (b.current - b.start).length();
        if drag >= tuning.selection_drag_threshold {
            let start = to_screen(b.start);
            let current = to_screen(b.current);

            let min_x = start.x.min(current.x);
            let max_x = start.x.max(current.x);
            let min_y = start.y.min(current.y);
            let max_y = start.y.max(current.y);
            let w = max_x - min_x;
            let h = max_y - min_y;

            let fill = color_u8!(255, 255, 255, 28);
            let border = color_u8!(255, 255, 255, 160);
            draw_rectangle(min_x, min_y, w, h, fill);
            draw_rectangle_lines(min_x, min_y, w, h, 2.0, border);
        }
    }
}

fn draw_level1_tutorial(world: &World, main_font: Option<&Font>, offset_y: f32) {
    // Simple in-world tutorial text, inspired by The Binding of Isaac floor labels.
    let w = world.bounds.x.max(1.0);
    let h = world.bounds.y.max(1.0);
    let ui_scale = ((screen_width() / WINDOW_WIDTH as f32)
        .min(screen_height() / WINDOW_HEIGHT as f32))
    .clamp(0.75, 1.6);

    let font_size = (18.0 * ui_scale) as u16;
    let color = Color::new(1.0, 1.0, 1.0, 0.55);
    let shadow = Color::new(0.0, 0.0, 0.0, 0.45);

    let lines = [
        "Left click: select nomads",
        "Drag: box select",
        "Right click: move / hunt / eat",
        "Esc: pause",
    ];

    let block_x = w * 0.5;
    let block_y = h * 0.76;
    let line_h = 18.0 * ui_scale;

    for (i, text) in lines.iter().enumerate() {
        let y = offset_y + block_y + i as f32 * line_h;
        let dims = measure_text(text, main_font, font_size, 1.0);
        let x = block_x - dims.width * 0.5;

        draw_text_ex(
            text,
            x + 1.0,
            y + 1.0,
            TextParams {
                font: main_font,
                font_size,
                color: shadow,
                ..Default::default()
            },
        );
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: main_font,
                font_size,
                color,
                ..Default::default()
            },
        );
    }
}
