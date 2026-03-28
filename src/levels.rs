#[derive(Clone, Copy, Debug)]
pub struct LevelSpec {
    pub nomads: usize,
    pub animals: usize,
    pub hunger_start: i32,
}

// Edit this table to add/remove levels.
// The rest of the game (selector, progression, transitions) will follow LEVELS.len().
pub const LEVELS: &[LevelSpec] = &[
    LevelSpec {
        nomads: 1,
        animals: 3,
        hunger_start: 50,
    },
    LevelSpec {
        nomads: 1,
        animals: 6,
        hunger_start: 50,
    },
    LevelSpec {
        nomads: 1,
        animals: 9,
        hunger_start: 50,
    },
    LevelSpec {
        nomads: 1,
        animals: 12,
        hunger_start: 50,
    },
    LevelSpec {
        nomads: 1,
        animals: 15,
        hunger_start: 50,
    },
    LevelSpec {
        nomads: 2,
        animals: 18,
        hunger_start: 50,
    },
    LevelSpec {
        nomads: 2,
        animals: 21,
        hunger_start: 50,
    },
    LevelSpec {
        nomads: 2,
        animals: 24,
        hunger_start: 50,
    },
    LevelSpec {
        nomads: 2,
        animals: 27,
        hunger_start: 50,
    },
    LevelSpec {
        nomads: 2,
        animals: 30,
        hunger_start: 50,
    },
];

pub fn count() -> usize {
    LEVELS.len()
}

// Levels are 1-indexed in the UI/game.
pub fn get(level: usize) -> Option<LevelSpec> {
    LEVELS.get(level.saturating_sub(1)).copied()
}
