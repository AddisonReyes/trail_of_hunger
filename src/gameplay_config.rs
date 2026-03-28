pub const WINDOW_TITLE: &str = "Trail of Hunger";
pub const WINDOW_WIDTH: i32 = 640;
pub const WINDOW_HEIGHT: i32 = 360;

pub const FONT_PATH: &str = "assets/fonts/alagard.ttf";

#[derive(Clone, Copy, Debug)]
pub struct GamePlayConfig {
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

    // Level flow
    pub transition_seconds: f32,
    pub spawn_margin: f32,
    pub max_levels: usize,
    pub animals_per_level: usize,
    pub extra_nomad_level: usize,

    // Render sizes (purely visual but handy to tweak)
    pub render_nomad_radius: f32,
    pub render_animal_radius: f32,
    pub render_corpse_radius: f32,
}

impl Default for GamePlayConfig {
    fn default() -> Self {
        Self {
            selection_nomad_radius: 14.0,
            selection_drag_threshold: 6.0,

            pick_radius_animal: 22.0,
            pick_radius_corpse: 18.0,

            nomad_speed: 90.0,
            nomad_attack_range: 60.0,
            nomad_spear_cooldown: 0.8,
            nomad_move_reach: 10.0,
            nomad_eat_range: 18.0,

            spear_speed: 220.0,
            spear_ttl: 1.5,
            spear_hit_radius: 10.0,

            hunger_start: 45,
            hunger_max: 100,
            hunger_tick_seconds: 1.0,
            eat_gain_min: 1,
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

            transition_seconds: 4.0,
            spawn_margin: 20.0,
            max_levels: 10,
            animals_per_level: 3,
            extra_nomad_level: 6,

            render_nomad_radius: 8.0,
            render_animal_radius: 6.0,
            render_corpse_radius: 7.0,
        }
    }
}

impl GamePlayConfig {
    pub fn nomads_to_spawn(&self, level: usize) -> usize {
        if level >= self.extra_nomad_level {
            2
        } else {
            1
        }
    }

    pub fn animals_to_spawn(&self, level: usize) -> usize {
        self.animals_per_level * level
    }
}
