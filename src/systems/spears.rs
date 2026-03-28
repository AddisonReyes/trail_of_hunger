use crate::entities::NomadOrder;
use crate::gameplay_config::GamePlayConfig;
use crate::world::{Corpse, World};

pub fn update(dt: f32, world: &mut World, tuning: &GamePlayConfig) {
    let bounds = world.bounds;

    let mut i = 0;
    while i < world.spears.len() {
        let mut remove_spear = false;
        {
            let s = &mut world.spears[i];
            s.ttl -= dt;
            s.pos += s.vel * dt;

            if s.ttl <= 0.0 {
                remove_spear = true;
            }

            if s.pos.x < 0.0 || s.pos.x > bounds.x || s.pos.y < 0.0 || s.pos.y > bounds.y {
                remove_spear = true;
            }

            if !remove_spear {
                let hit_r2 = tuning.spear_hit_radius * tuning.spear_hit_radius;
                if let Some(ai) = world
                    .animals
                    .iter()
                    .position(|a| a.get_position().distance_squared(s.pos) <= hit_r2)
                {
                    let animal_id = world.animals[ai].id();
                    world.animals[ai].take_damage(1);
                    remove_spear = true;

                    if world.animals[ai].is_dead() {
                        let dead_pos = world.animals[ai].get_position();
                        world.animals.swap_remove(ai);

                        let corpse_id = world.next_corpse_id;
                        world.next_corpse_id += 1;
                        world.corpses.push(Corpse {
                            id: corpse_id,
                            pos: dead_pos,
                            available: true,
                        });

                        for n in &mut world.nomads {
                            if n.order() == NomadOrder::Hunt(animal_id) {
                                n.set_order(NomadOrder::Idle);
                            }
                        }
                    }
                }
            }
        }

        if remove_spear {
            world.spears.swap_remove(i);
        } else {
            i += 1;
        }
    }
}
