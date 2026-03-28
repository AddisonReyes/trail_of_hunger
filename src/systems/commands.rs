use macroquad::prelude::*;

use crate::entities::NomadOrder;
use crate::gameplay_config::GamePlayConfig;
use crate::input::InputState;
use crate::state::{CommandFeedback, CommandState, CommandTarget};
use crate::world::World;

pub fn update(
    input: &InputState,
    world: &mut World,
    cmd: &mut CommandState,
    tuning: &GamePlayConfig,
) {
    if !input.right_pressed {
        return;
    }

    if !world.nomads.iter().any(|n| n.is_selected()) {
        return;
    }

    let mouse = input.mouse;

    let command_id = cmd.next_command_id;
    cmd.next_command_id += 1;

    if let Some(corpse_id) = pick_corpse_id(world, mouse, tuning.pick_radius_corpse) {
        for n in &mut world.nomads {
            if n.is_selected() {
                n.set_order(NomadOrder::Eat(corpse_id));
                n.set_order_id(command_id);
            }
        }

        cmd.last_command = Some(CommandFeedback {
            id: command_id,
            click_pos: mouse,
            target: CommandTarget::Corpse(corpse_id),
        });
        return;
    }

    if let Some(animal_id) = pick_animal_id(world, mouse, tuning.pick_radius_animal) {
        for n in &mut world.nomads {
            if n.is_selected() {
                n.set_order(NomadOrder::Hunt(animal_id));
                n.set_order_id(command_id);
            }
        }

        cmd.last_command = Some(CommandFeedback {
            id: command_id,
            click_pos: mouse,
            target: CommandTarget::Animal(animal_id),
        });
        return;
    }

    for n in &mut world.nomads {
        if n.is_selected() {
            n.set_order(NomadOrder::MoveTo(mouse));
            n.set_order_id(command_id);
        }
    }

    cmd.last_command = Some(CommandFeedback {
        id: command_id,
        click_pos: mouse,
        target: CommandTarget::Point(mouse),
    });
}

pub fn update_feedback(world: &World, cmd: &mut CommandState) {
    let Some(last) = cmd.last_command else {
        return;
    };

    let any_active = world
        .nomads
        .iter()
        .any(|n| n.order_id() == last.id && n.order() != NomadOrder::Idle);

    if !any_active {
        cmd.last_command = None;
    }
}

fn pick_animal_id(world: &World, point: Vec2, radius: f32) -> Option<u32> {
    let mut best: Option<(u32, f32)> = None;
    for a in &world.animals {
        let d2 = a.get_position().distance_squared(point);
        if d2 <= radius * radius {
            if best.is_none() || d2 < best.unwrap().1 {
                best = Some((a.id(), d2));
            }
        }
    }
    best.map(|b| b.0)
}

fn pick_corpse_id(world: &World, point: Vec2, radius: f32) -> Option<u32> {
    let mut best: Option<(u32, f32)> = None;
    for c in &world.corpses {
        if !c.available {
            continue;
        }

        let d2 = c.pos.distance_squared(point);
        if d2 <= radius * radius {
            if best.is_none() || d2 < best.unwrap().1 {
                best = Some((c.id, d2));
            }
        }
    }
    best.map(|b| b.0)
}
