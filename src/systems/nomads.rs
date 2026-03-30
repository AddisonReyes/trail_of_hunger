use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::entities::NomadOrder;
use crate::gameplay_config::GamePlayConfig;
use crate::world::{Spear, World};

pub fn update(dt: f32, world: &mut World, hunger: &mut i32, tuning: &GamePlayConfig) -> u32 {
    let bounds = world.bounds;
    let nomad_radius = tuning.render_nomad_radius;

    let mut eats = 0_u32;

    for (nomad_index, n) in world.nomads.iter_mut().enumerate() {
        n.tick_attack_cd(dt);

        match n.order() {
            NomadOrder::Idle => {}
            NomadOrder::MoveTo(target) => {
                if n.get_position().distance(target) <= tuning.nomad_move_reach {
                    n.set_order(NomadOrder::Idle);
                    continue;
                }

                if n.move_towards(target, dt, tuning.nomad_speed, bounds, nomad_radius) {
                    n.set_order(NomadOrder::Idle);
                }
            }
            NomadOrder::Hunt(animal_id) => {
                let animal_pos = world
                    .animals
                    .iter()
                    .find(|a| a.id() == animal_id)
                    .map(|a| a.get_position());

                let Some(target_pos) = animal_pos else {
                    n.set_order(NomadOrder::Idle);
                    continue;
                };

                let dist = n.get_position().distance(target_pos);
                if dist > tuning.nomad_attack_range {
                    n.move_towards(target_pos, dt, tuning.nomad_speed, bounds, nomad_radius);
                    continue;
                }

                if n.can_attack() {
                    let dir = (target_pos - n.get_position()).normalize_or_zero();
                    if dir.length_squared() > 0.0 {
                        world.spears.push(Spear {
                            pos: n.get_position(),
                            vel: dir * tuning.spear_speed,
                            ttl: tuning.spear_ttl,
                            owner_nomad: Some(nomad_index),
                        });
                        n.reset_attack_cd(tuning.nomad_spear_cooldown);
                    }
                }
            }
            NomadOrder::Eat(corpse_id) => {
                let corpse_pos = world
                    .corpses
                    .iter()
                    .find(|c| c.id == corpse_id && c.available)
                    .map(|c| c.pos);

                let Some(target_pos) = corpse_pos else {
                    n.set_order(NomadOrder::Idle);
                    continue;
                };

                let dist = n.get_position().distance(target_pos);
                if dist > tuning.nomad_eat_range {
                    n.move_towards(target_pos, dt, tuning.nomad_speed, bounds, nomad_radius);
                    continue;
                }

                if let Some(c) = world
                    .corpses
                    .iter_mut()
                    .find(|c| c.id == corpse_id && c.available)
                {
                    c.available = false;
                    let gain = gen_range(tuning.eat_gain_min, tuning.eat_gain_max);
                    *hunger = (*hunger + gain).clamp(0, tuning.hunger_max);
                    eats += 1;
                }

                n.set_order(NomadOrder::Idle);
            }
        }
    }

    resolve_nomad_collisions(world, tuning);

    eats
}

fn resolve_nomad_collisions(world: &mut World, cfg: &GamePlayConfig) {
    let r = cfg.nomad_collision_radius;
    if r <= 0.0 {
        return;
    }

    let n = world.nomads.len();
    if n < 2 {
        return;
    }

    let bounds = world.bounds;
    let clamp_r = cfg.render_nomad_radius.max(0.0);
    let min_dist = r * 2.0;
    let min_dist2 = min_dist * min_dist;
    let strength = cfg.nomad_collision_strength.clamp(0.0, 2.0);

    for _ in 0..cfg.nomad_collision_iterations {
        for i in 0..n {
            for j in (i + 1)..n {
                let pi = world.nomads[i].get_position();
                let pj = world.nomads[j].get_position();

                let delta = pj - pi;
                let d2 = delta.length_squared();
                if d2 >= min_dist2 {
                    continue;
                }

                let d = d2.sqrt();
                let normal = if d > 0.0001 {
                    delta / d
                } else {
                    // Arbitrary but stable direction for perfect overlap.
                    vec2(1.0, 0.0)
                };

                let penetration = (min_dist - d).max(0.0);
                let push = normal * (penetration * 0.5 * strength);

                let new_pi = clamp_position(pi - push, bounds, clamp_r);
                let new_pj = clamp_position(pj + push, bounds, clamp_r);
                world.nomads[i].set_position(new_pi);
                world.nomads[j].set_position(new_pj);
            }
        }
    }
}

fn clamp_position(pos: Vec2, bounds: Vec2, radius: f32) -> Vec2 {
    let r = radius.max(0.0);

    let min_x = r;
    let max_x = (bounds.x - r).max(min_x);
    let min_y = r;
    let max_y = (bounds.y - r).max(min_y);

    vec2(pos.x.clamp(min_x, max_x), pos.y.clamp(min_y, max_y))
}
