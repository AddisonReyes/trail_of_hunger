use macroquad::prelude::*;

use crate::gameplay_config::GamePlayConfig;
use crate::state::{CommandFeedback, CommandTarget, SelectionBox};
use crate::world::World;

pub fn draw_world(
    world: &World,
    selection_box: Option<SelectionBox>,
    last_command: Option<CommandFeedback>,
    tuning: &GamePlayConfig,
) {
    clear_background(color_u8!(33, 104, 58, 255));

    for a in &world.animals {
        let pos = a.get_position();
        draw_circle(
            pos.x,
            pos.y,
            tuning.render_animal_radius,
            color_u8!(196, 160, 106, 255),
        );
    }

    for c in &world.corpses {
        let color = if c.available {
            color_u8!(180, 64, 48, 255)
        } else {
            color_u8!(90, 70, 70, 255)
        };
        draw_circle(c.pos.x, c.pos.y, tuning.render_corpse_radius, color);
    }

    for s in &world.spears {
        let tail = s.pos - s.vel.normalize_or_zero() * 8.0;
        draw_line(
            tail.x,
            tail.y,
            s.pos.x,
            s.pos.y,
            2.0,
            color_u8!(230, 230, 230, 255),
        );
    }

    if let Some(cmd) = last_command {
        draw_circle(cmd.click_pos.x, cmd.click_pos.y, 3.5, WHITE);

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

        for n in &world.nomads {
            if n.order_id() == cmd.id && n.order() != crate::entities::NomadOrder::Idle {
                let p = n.get_position();
                draw_line(p.x, p.y, end.x, end.y, 1.0, color_u8!(255, 255, 255, 120));
            }
        }
    }

    for n in &world.nomads {
        let pos = n.get_position();
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
            let min_x = b.start.x.min(b.current.x);
            let max_x = b.start.x.max(b.current.x);
            let min_y = b.start.y.min(b.current.y);
            let max_y = b.start.y.max(b.current.y);
            let w = max_x - min_x;
            let h = max_y - min_y;

            let fill = color_u8!(255, 255, 255, 28);
            let border = color_u8!(255, 255, 255, 160);
            draw_rectangle(min_x, min_y, w, h, fill);
            draw_rectangle_lines(min_x, min_y, w, h, 2.0, border);
        }
    }
}
