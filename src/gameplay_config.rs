pub const WINDOW_TITLE: &str = "Trail of Hunger";
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 600;

pub const FONT_PATH: &str = "assets/fonts/alagard.ttf";

#[derive(Clone, Copy, Debug, Default)]
pub struct LevelOverrides {
    pub hunger_tick_seconds: Option<f32>,
    pub eat_gain_min: Option<i32>,
    pub eat_gain_max: Option<i32>,

    pub animal_wander_speed_min: Option<f32>,
    pub animal_wander_speed_max: Option<f32>,
    pub animal_flee_radius: Option<f32>,
    pub animal_flee_speed_base: Option<f32>,
    pub animal_flee_speed_max: Option<f32>,
    pub animal_flee_timer_min: Option<f32>,
    pub animal_flee_timer_max: Option<f32>,
}

pub const NO_OVERRIDES: LevelOverrides = LevelOverrides {
    hunger_tick_seconds: None,
    eat_gain_min: None,
    eat_gain_max: None,

    animal_wander_speed_min: None,
    animal_wander_speed_max: None,
    animal_flee_radius: None,
    animal_flee_speed_base: None,
    animal_flee_speed_max: None,
    animal_flee_timer_min: None,
    animal_flee_timer_max: None,
};

#[derive(Clone, Copy, Debug)]
pub struct GamePlayConfig {
    // UI
    pub ui_top_bar_height: f32,

    // Selection / input
    pub selection_nomad_radius: f32,
    pub selection_drag_threshold: f32,

    // Command picking
    pub pick_radius_animal: f32,
    pub pick_radius_corpse: f32,

    // Nomads
    pub nomad_speed: f32,
    pub nomad_attack_range: f32,
    pub nomad_spear_cooldown: f32,
    pub nomad_move_reach: f32,
    pub nomad_eat_range: f32,
    pub nomad_collision_radius: f32,
    pub nomad_collision_iterations: u32,
    pub nomad_collision_strength: f32,

    // Spears / projectiles
    pub spear_speed: f32,
    pub spear_ttl: f32,
    pub spear_hit_radius: f32,

    // Hunger
    pub hunger_start: i32,
    pub hunger_max: i32,
    pub hunger_tick_seconds: f32,
    pub eat_gain_min: i32,
    pub eat_gain_max: i32,

    // Animals AI
    pub animal_wander_speed_min: f32,
    pub animal_wander_speed_max: f32,
    pub animal_wander_timer_init_min: f32,
    pub animal_wander_timer_init_max: f32,
    pub animal_wander_timer_reset_min: f32,
    pub animal_wander_timer_reset_max: f32,
    pub animal_flee_radius: f32,
    pub animal_flee_speed_base: f32,
    pub animal_flee_speed_max: f32,
    pub animal_flee_timer_min: f32,
    pub animal_flee_timer_max: f32,

    // Animal "wound" flavor
    pub animal_wound_survive_chance: f32,
    pub animal_wounded_speed_mult: f32,
    pub animal_wounded_stutter_interval_min: f32,
    pub animal_wounded_stutter_interval_max: f32,
    pub animal_wounded_stutter_duration_min: f32,
    pub animal_wounded_stutter_duration_max: f32,
    pub animal_wounded_stutter_speed_mult: f32,
    pub animal_wounded_min_speed: f32,

    // Blood trail (subtle)
    pub blood_drop_interval_min: f32,
    pub blood_drop_interval_max: f32,
    pub blood_drop_radius: f32,

    // Level flow
    pub transition_seconds: f32,
    pub spawn_margin: f32,

    // Render sizes (purely visual but handy to tweak)
    pub render_nomad_radius: f32,
    pub render_animal_radius: f32,
    pub render_corpse_radius: f32,
}

impl Default for GamePlayConfig {
    fn default() -> Self {
        Self {
            ui_top_bar_height: 72.0,

            selection_nomad_radius: 14.0,
            selection_drag_threshold: 6.0,

            pick_radius_animal: 22.0,
            pick_radius_corpse: 18.0,

            nomad_speed: 90.0,
            nomad_attack_range: 60.0,
            nomad_spear_cooldown: 0.8,
            nomad_move_reach: 10.0,
            nomad_eat_range: 18.0,
            nomad_collision_radius: 9.0,
            nomad_collision_iterations: 2,
            nomad_collision_strength: 1.0,

            spear_speed: 220.0,
            spear_ttl: 1.5,
            spear_hit_radius: 10.0,

            hunger_start: 50,
            hunger_max: 100,
            hunger_tick_seconds: 1.0,
            eat_gain_min: 2,
            eat_gain_max: 4,

            animal_wander_speed_min: 10.0,
            animal_wander_speed_max: 22.0,
            animal_wander_timer_init_min: 0.3,
            animal_wander_timer_init_max: 1.2,
            animal_wander_timer_reset_min: 0.4,
            animal_wander_timer_reset_max: 1.4,
            animal_flee_radius: 60.0,
            animal_flee_speed_base: 80.0,
            animal_flee_speed_max: 110.0,
            animal_flee_timer_min: 0.2,
            animal_flee_timer_max: 0.6,

            animal_wound_survive_chance: 0.20,
            animal_wounded_speed_mult: 0.65,
            animal_wounded_stutter_interval_min: 0.9,
            animal_wounded_stutter_interval_max: 1.9,
            animal_wounded_stutter_duration_min: 0.07,
            animal_wounded_stutter_duration_max: 0.16,
            animal_wounded_stutter_speed_mult: 0.80,
            animal_wounded_min_speed: 4.0,

            blood_drop_interval_min: 0.10,
            blood_drop_interval_max: 0.16,
            blood_drop_radius: 2.6,

            transition_seconds: 4.0,
            spawn_margin: 20.0,

            render_nomad_radius: 8.0,
            render_animal_radius: 6.0,
            render_corpse_radius: 7.0,
        }
    }
}

impl GamePlayConfig {
    pub fn apply_overrides(&self, ov: LevelOverrides) -> Self {
        let mut cfg = *self;

        if let Some(v) = ov.hunger_tick_seconds {
            cfg.hunger_tick_seconds = v;
        }
        if let Some(v) = ov.eat_gain_min {
            cfg.eat_gain_min = v;
        }
        if let Some(v) = ov.eat_gain_max {
            cfg.eat_gain_max = v;
        }

        if let Some(v) = ov.animal_wander_speed_min {
            cfg.animal_wander_speed_min = v;
        }
        if let Some(v) = ov.animal_wander_speed_max {
            cfg.animal_wander_speed_max = v;
        }

        if let Some(v) = ov.animal_flee_radius {
            cfg.animal_flee_radius = v;
        }
        if let Some(v) = ov.animal_flee_speed_base {
            cfg.animal_flee_speed_base = v;
        }
        if let Some(v) = ov.animal_flee_speed_max {
            cfg.animal_flee_speed_max = v;
        }

        if let Some(v) = ov.animal_flee_timer_min {
            cfg.animal_flee_timer_min = v;
        }
        if let Some(v) = ov.animal_flee_timer_max {
            cfg.animal_flee_timer_max = v;
        }

        cfg
    }
}

impl GamePlayConfig {}
