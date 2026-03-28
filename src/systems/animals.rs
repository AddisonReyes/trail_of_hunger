use crate::gameplay_config::GamePlayConfig;
use crate::world::World;

pub fn update(dt: f32, world: &mut World, tuning: &GamePlayConfig) {
    let bounds = world.bounds;
    for a in &mut world.animals {
        a.update(dt, &world.nomads, bounds, tuning);
    }
}
