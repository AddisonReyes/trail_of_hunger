use crate::gameplay_config::{LevelOverrides, NO_OVERRIDES};

#[derive(Clone, Copy, Debug)]
pub struct LevelSpec {
    pub nomads: usize,
    pub animals: usize,
    pub hunger_start: i32,
    pub overrides: LevelOverrides,
}

// Edit this table to add/remove levels.
// The rest of the game (selector, progression, transitions) will follow LEVELS.len().
pub const LEVELS: &[LevelSpec] = &[
    LevelSpec {
        nomads: 1,
        animals: 3,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(55.0),
            animal_flee_speed_base: Some(75.0),
            animal_flee_speed_max: Some(105.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 1,
        animals: 6,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(58.0),
            animal_flee_speed_max: Some(110.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 1,
        animals: 9,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(60.0),
            animal_flee_speed_max: Some(115.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 1,
        animals: 12,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(62.0),
            animal_flee_speed_max: Some(120.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 1,
        animals: 15,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(66.0),
            animal_flee_speed_max: Some(125.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 2,
        animals: 18,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(70.0),
            animal_flee_speed_max: Some(132.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 2,
        animals: 21,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(74.0),
            animal_flee_speed_max: Some(138.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 2,
        animals: 24,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(78.0),
            animal_flee_speed_max: Some(144.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 2,
        animals: 27,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(82.0),
            animal_flee_speed_max: Some(150.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 2,
        animals: 30,
        hunger_start: 50,
        overrides: LevelOverrides {
            animal_flee_radius: Some(86.0),
            animal_flee_speed_max: Some(158.0),
            ..NO_OVERRIDES
        },
    },
    // Phase 2: introduce 3 nomads, scale mainly by animal count.
    LevelSpec {
        nomads: 3,
        animals: 34,
        hunger_start: 50,
        overrides: LevelOverrides {
            hunger_tick_seconds: Some(0.92),
            animal_flee_radius: Some(86.0),
            animal_flee_speed_max: Some(158.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 3,
        animals: 38,
        hunger_start: 50,
        overrides: LevelOverrides {
            hunger_tick_seconds: Some(0.88),
            animal_flee_radius: Some(88.0),
            animal_flee_speed_max: Some(160.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 3,
        animals: 42,
        hunger_start: 50,
        overrides: LevelOverrides {
            hunger_tick_seconds: Some(0.84),
            animal_flee_radius: Some(90.0),
            animal_flee_speed_max: Some(162.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 3,
        animals: 46,
        hunger_start: 50,
        overrides: LevelOverrides {
            hunger_tick_seconds: Some(0.80),
            animal_flee_radius: Some(92.0),
            animal_flee_speed_max: Some(164.0),
            ..NO_OVERRIDES
        },
    },
    LevelSpec {
        nomads: 3,
        animals: 50,
        hunger_start: 50,
        overrides: LevelOverrides {
            hunger_tick_seconds: Some(0.76),
            animal_flee_radius: Some(94.0),
            animal_flee_speed_max: Some(166.0),
            ..NO_OVERRIDES
        },
    },
];

pub fn count() -> usize {
    LEVELS.len()
}

// Levels are 1-indexed in the UI/game.
pub fn get(level: usize) -> Option<LevelSpec> {
    LEVELS.get(level.saturating_sub(1)).copied()
}
