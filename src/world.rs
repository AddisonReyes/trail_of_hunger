use macroquad::prelude::*;

use crate::entities::{Animal, Nomad};

#[derive(Clone, Copy, Debug)]
pub struct Corpse {
    pub id: u32,
    pub pos: Vec2,
    pub available: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Spear {
    pub pos: Vec2,
    pub vel: Vec2,
    pub ttl: f32,
}

pub struct World {
    pub bounds: Vec2,
    pub nomads: Vec<Nomad>,
    pub animals: Vec<Animal>,
    pub corpses: Vec<Corpse>,
    pub spears: Vec<Spear>,
    pub next_animal_id: u32,
    pub next_corpse_id: u32,
}

impl World {
    pub fn new(bounds: Vec2) -> Self {
        Self {
            bounds,
            nomads: Vec::new(),
            animals: Vec::new(),
            corpses: Vec::new(),
            spears: Vec::new(),
            next_animal_id: 1,
            next_corpse_id: 1,
        }
    }

    pub fn clear_entities(&mut self) {
        self.nomads.clear();
        self.animals.clear();
        self.corpses.clear();
        self.spears.clear();
    }
}
