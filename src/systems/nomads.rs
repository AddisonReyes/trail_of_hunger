use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::entities::NomadOrder;
use crate::gameplay_config::GamePlayConfig;
use crate::world::{Spear, World};

pub fn update(dt: f32, world: &mut World, hunger: &mut i32, tuning: &GamePlayConfig) {
    let bounds = world.bounds;

    for n in &mut world.nomads {
        n.tick_attack_cd(dt);

        match n.order() {
            NomadOrder::Idle => {}
            NomadOrder::MoveTo(target) => {
                if n.get_position().distance(target) <= tuning.nomad_move_reach {
                    n.set_order(NomadOrder::Idle);
                    continue;
                }

                if n.move_towards(target, dt, tuning.nomad_speed, bounds) {
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
                    n.move_towards(target_pos, dt, tuning.nomad_speed, bounds);
                    continue;
                }

                if n.can_attack() {
                    let dir = (target_pos - n.get_position()).normalize_or_zero();
                    if dir.length_squared() > 0.0 {
                        world.spears.push(Spear {
                            pos: n.get_position(),
                            vel: dir * tuning.spear_speed,
                            ttl: tuning.spear_ttl,
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
                    n.move_towards(target_pos, dt, tuning.nomad_speed, bounds);
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
                }

                n.set_order(NomadOrder::Idle);
            }
        }
    }
}
