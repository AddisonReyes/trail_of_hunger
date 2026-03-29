use crate::gameplay_config::GamePlayConfig;
use crate::render::blood_layer::BloodLayer;
use crate::world::World;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

pub fn update(dt: f32, world: &mut World, tuning: &GamePlayConfig, blood: &mut BloodLayer) {
    let bounds = world.bounds;
    for a in &mut world.animals {
        a.update(dt, &world.nomads, bounds, tuning);

        if a.is_wounded()
            && let Some(p) = a.tick_bleed(dt, tuning)
        {
            // Tiny jitter to avoid perfectly straight dotted lines.
            let jitter = vec2(gen_range(-1.2, 1.2), gen_range(-1.2, 1.2));
            let base = p + jitter;
            // Slightly stronger than "sutil": visible, but not neon.
            blood.paint_drop(
                base,
                tuning.blood_drop_radius,
                Color::new(0.46, 0.08, 0.07, 0.30),
            );
            // Small darker core to read against grass.
            blood.paint_drop(
                base,
                (tuning.blood_drop_radius * 0.72).max(0.0),
                Color::new(0.30, 0.03, 0.03, 0.22),
            );
        }
    }
}
