use macroquad::prelude::*;
use macroquad::rand::gen_range;

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
    pub fn new_at(id: u32, position: Vec2) -> Self {
        Self {
            id,
            position,
            velocity: random_unit_vec2() * gen_range(12.0, 20.0),
            wander_timer: gen_range(0.3, 1.2),
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

    pub fn update(&mut self, dt: f32, nomads: &[Nomad], bounds: Vec2) {
        const WANDER_SPEED_MIN: f32 = 10.0;
        const WANDER_SPEED_MAX: f32 = 22.0;
        const FLEE_RADIUS: f32 = 60.0;
        const FLEE_SPEED: f32 = 80.0;

        let mut flee = vec2(0.0, 0.0);
        for n in nomads {
            let delta = self.position - n.get_position();
            let d2 = delta.length_squared();
            if d2 > 0.0001 && d2 < (FLEE_RADIUS * FLEE_RADIUS) {
                // Weighted stronger when closer.
                flee += delta.normalize() * (1.0 / d2.sqrt());
            }
        }

        if flee.length_squared() > 0.0 {
            self.velocity = flee.normalize() * FLEE_SPEED;
            self.wander_timer = gen_range(0.2, 0.6);
        } else {
            self.wander_timer -= dt;
            if self.wander_timer <= 0.0 {
                self.wander_timer = gen_range(0.4, 1.4);
                self.velocity = random_unit_vec2() * gen_range(WANDER_SPEED_MIN, WANDER_SPEED_MAX);
            }
        }

        self.position += self.velocity * dt;
        self.position = wrap_position(self.position, bounds);
    }
}
