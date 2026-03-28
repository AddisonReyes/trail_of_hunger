use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::gameplay_config::GamePlayConfig;

fn wrap_position(mut pos: Vec2, bounds: Vec2) -> Vec2 {
    if pos.x < 0.0 {
        pos.x = bounds.x;
    } else if pos.x > bounds.x {
        pos.x = 0.0;
    }

    if pos.y < 0.0 {
        pos.y = bounds.y;
    } else if pos.y > bounds.y {
        pos.y = 0.0;
    }

    pos
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

    pub fn move_dir(&mut self, dir: Vec2, dt: f32, speed: f32, bounds: Vec2) {
        if dir.length_squared() == 0.0 {
            return;
        }

        self.position += dir.normalize() * speed * dt;
        self.position = wrap_position(self.position, bounds);
    }

    pub fn move_towards(&mut self, target: Vec2, dt: f32, speed: f32, bounds: Vec2) -> bool {
        let to = target - self.position;
        if to.length() <= 2.0 {
            return true;
        }

        self.move_dir(to, dt, speed, bounds);
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
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
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

        self.position += self.velocity * dt;
        self.position = wrap_position(self.position, bounds);
    }
}
