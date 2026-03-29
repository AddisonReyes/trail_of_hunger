use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::gameplay_config::GamePlayConfig;

fn clamp_position(pos: Vec2, bounds: Vec2, radius: f32) -> Vec2 {
    let r = radius.max(0.0);

    let min_x = r;
    let max_x = (bounds.x - r).max(min_x);
    let min_y = r;
    let max_y = (bounds.y - r).max(min_y);

    vec2(pos.x.clamp(min_x, max_x), pos.y.clamp(min_y, max_y))
}

fn random_unit_vec2() -> Vec2 {
    let a = gen_range(0.0, std::f32::consts::TAU);
    vec2(a.cos(), a.sin())
}

pub struct Nomad {
    position: Vec2,
    selected: bool,
    order: NomadOrder,
    order_id: u32,
    attack_cd: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NomadOrder {
    Idle,
    MoveTo(Vec2),
    Hunt(u32),
    Eat(u32),
}

impl Nomad {
    pub fn new_at(position: Vec2) -> Self {
        Self {
            position,
            selected: false,
            order: NomadOrder::Idle,
            order_id: 0,
            attack_cd: 0.0,
        }
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn order(&self) -> NomadOrder {
        self.order
    }

    pub fn order_id(&self) -> u32 {
        self.order_id
    }

    pub fn set_order(&mut self, order: NomadOrder) {
        self.order = order;
    }

    pub fn set_order_id(&mut self, id: u32) {
        self.order_id = id;
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn toggle_selected(&mut self) {
        self.selected = !self.selected;
    }

    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }

    pub fn contains_point(&self, point: Vec2, radius: f32) -> bool {
        self.position.distance(point) <= radius
    }

    pub fn move_dir(&mut self, dir: Vec2, dt: f32, speed: f32, bounds: Vec2, radius: f32) {
        if dir.length_squared() == 0.0 {
            return;
        }

        self.position += dir.normalize() * speed * dt;
        self.position = clamp_position(self.position, bounds, radius);
    }

    pub fn move_towards(
        &mut self,
        target: Vec2,
        dt: f32,
        speed: f32,
        bounds: Vec2,
        radius: f32,
    ) -> bool {
        let to = target - self.position;
        if to.length() <= 2.0 {
            return true;
        }

        self.move_dir(to, dt, speed, bounds, radius);
        false
    }

    pub fn tick_attack_cd(&mut self, dt: f32) {
        self.attack_cd -= dt;
        if self.attack_cd < 0.0 {
            self.attack_cd = 0.0;
        }
    }

    pub fn can_attack(&self) -> bool {
        self.attack_cd <= 0.0
    }

    pub fn reset_attack_cd(&mut self, seconds: f32) {
        self.attack_cd = seconds;
    }
}

pub struct Animal {
    id: u32,
    position: Vec2,
    velocity: Vec2,
    wander_timer: f32,
    hp: i32,

    wounded: bool,
    bleed_cd: f32,
    stutter_cd: f32,
    stutter_t: f32,
}

impl Animal {
    pub fn new_at(id: u32, position: Vec2, tuning: &GamePlayConfig) -> Self {
        Self {
            id,
            position,
            velocity: random_unit_vec2()
                * gen_range(
                    tuning.animal_wander_speed_min,
                    tuning.animal_wander_speed_max,
                ),
            wander_timer: gen_range(
                tuning.animal_wander_timer_init_min,
                tuning.animal_wander_timer_init_max,
            ),
            hp: 1,

            wounded: false,
            bleed_cd: gen_range(
                tuning.blood_drop_interval_min,
                tuning.blood_drop_interval_max,
            ),
            stutter_cd: gen_range(
                tuning.animal_wounded_stutter_interval_min,
                tuning.animal_wounded_stutter_interval_max,
            ),
            stutter_t: 0.0,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    pub fn set_hp(&mut self, hp: i32) {
        self.hp = hp;
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }

    pub fn is_wounded(&self) -> bool {
        self.wounded
    }

    pub fn wound(&mut self, tuning: &GamePlayConfig) {
        self.wounded = true;

        // Start bleeding quickly so the first drop appears soon.
        self.bleed_cd = gen_range(
            tuning.blood_drop_interval_min * 0.25,
            tuning.blood_drop_interval_max * 0.5,
        );

        // First stutter happens after a short delay.
        self.stutter_cd = gen_range(
            tuning.animal_wounded_stutter_interval_min,
            tuning.animal_wounded_stutter_interval_max,
        );
        self.stutter_t = 0.0;
    }

    pub fn tick_bleed(&mut self, dt: f32, tuning: &GamePlayConfig) -> Option<Vec2> {
        if !self.wounded {
            return None;
        }

        // Only leave a trail while moving (subtle, avoids puddling when stopped).
        if self.velocity.length_squared() <= 1.0 {
            return None;
        }

        self.bleed_cd -= dt;
        if self.bleed_cd > 0.0 {
            return None;
        }

        self.bleed_cd = gen_range(
            tuning.blood_drop_interval_min,
            tuning.blood_drop_interval_max,
        );
        Some(self.position)
    }

    pub fn get_position(&self) -> Vec2 {
        self.position
    }

    pub fn update(&mut self, dt: f32, nomads: &[Nomad], bounds: Vec2, tuning: &GamePlayConfig) {
        let flee_radius = tuning.animal_flee_radius;

        let mut flee = vec2(0.0, 0.0);
        let mut closest_d2 = f32::INFINITY;
        for n in nomads {
            let delta = self.position - n.get_position();
            let d2 = delta.length_squared();
            if d2 > 0.0001 && d2 < (flee_radius * flee_radius) {
                // Weighted stronger when closer.
                flee += delta.normalize() * (1.0 / d2.sqrt());
                closest_d2 = closest_d2.min(d2);
            }
        }

        if flee.length_squared() > 0.0 {
            // Move a bit faster when a nomad is very close.
            let r2 = flee_radius * flee_radius;
            let t = ((r2 - closest_d2) / r2).clamp(0.0, 1.0);
            let flee_speed = tuning.animal_flee_speed_base
                + (tuning.animal_flee_speed_max - tuning.animal_flee_speed_base) * t;
            self.velocity = flee.normalize() * flee_speed;
            self.wander_timer =
                gen_range(tuning.animal_flee_timer_min, tuning.animal_flee_timer_max);
        } else {
            self.wander_timer -= dt;
            if self.wander_timer <= 0.0 {
                self.wander_timer = gen_range(
                    tuning.animal_wander_timer_reset_min,
                    tuning.animal_wander_timer_reset_max,
                );
                self.velocity = random_unit_vec2()
                    * gen_range(
                        tuning.animal_wander_speed_min,
                        tuning.animal_wander_speed_max,
                    );
            }
        }

        if self.wounded {
            // Occasional brief stops to signal weakness (guilt-inducing, but not slapstick).
            self.stutter_t = (self.stutter_t - dt).max(0.0);
            if self.stutter_t <= 0.0 {
                self.stutter_cd -= dt;
                if self.stutter_cd <= 0.0 {
                    self.stutter_t = gen_range(
                        tuning.animal_wounded_stutter_duration_min,
                        tuning.animal_wounded_stutter_duration_max,
                    );
                    self.stutter_cd = gen_range(
                        tuning.animal_wounded_stutter_interval_min,
                        tuning.animal_wounded_stutter_interval_max,
                    );
                }
            }

            let mut speed_mult = tuning.animal_wounded_speed_mult;
            if self.stutter_t > 0.0 {
                speed_mult *= tuning.animal_wounded_stutter_speed_mult;
            }
            self.velocity *= speed_mult;

            // Ensure wounded animals keep moving (no full stops / no near-zero crawl).
            let min_speed = tuning.animal_wounded_min_speed.max(0.0);
            if min_speed > 0.0 {
                let len = self.velocity.length();
                if len < min_speed {
                    let dir = if len > 0.0001 {
                        self.velocity / len
                    } else {
                        random_unit_vec2()
                    };
                    self.velocity = dir * min_speed;
                }
            }
        }

        self.position += self.velocity * dt;

        // Bounce off edges instead of wrapping.
        let r = tuning.render_animal_radius.max(0.0);
        let min_x = r;
        let max_x = (bounds.x - r).max(min_x);
        let min_y = r;
        let max_y = (bounds.y - r).max(min_y);

        if self.position.x < min_x {
            self.position.x = min_x;
            self.velocity.x = self.velocity.x.abs();
        } else if self.position.x > max_x {
            self.position.x = max_x;
            self.velocity.x = -self.velocity.x.abs();
        }

        if self.position.y < min_y {
            self.position.y = min_y;
            self.velocity.y = self.velocity.y.abs();
        } else if self.position.y > max_y {
            self.position.y = max_y;
            self.velocity.y = -self.velocity.y.abs();
        }
    }
}
