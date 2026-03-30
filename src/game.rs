use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::assets::Assets;
use crate::entities::{Animal, Nomad};
use crate::gameplay_config::GamePlayConfig;
use crate::gameplay_config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::input::InputState;
use crate::levels;
use crate::state::{CommandState, SelectionBox, UiHint};
use crate::world::World;
use crate::{render, systems, ui};
use crate::render::blood_layer::BloodLayer;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Menu,
    LevelSelect,
    InGame,
    GameOver,
}

#[derive(Clone, Copy, Debug)]
struct LevelTransition {
    timer: f32,
    next_level: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
enum ThemePhase {
    Playing,
    Gap,
}

#[derive(Clone, Copy, Debug)]
struct ThemeCycle {
    phase: ThemePhase,
    remaining: f32,
}

pub struct GameManager {
    assets: Assets,
    screen: Screen,
    world: World,
    blood_layer: BloodLayer,
    config: GamePlayConfig,
    active_config: GamePlayConfig,

    unlocked_max_level: usize,

    selection_box: Option<SelectionBox>,
    command: CommandState,

    hunger_timer: f32,
    hunger: i32,

    level_transition: Option<LevelTransition>,
    selected_level: usize,
    paused: bool,
    debug: bool,
    level: i32,

    last_mouse: Vec2,

    level_animals_total: usize,

    ui_hint: Option<UiHint>,

    theme_cycle: Option<ThemeCycle>,

    selection_sfx_cd: f32,
}

impl GameManager {
    pub async fn new() -> Self {
        let assets = Assets::load().await;
        let config = GamePlayConfig::default();
        let playfield_h = (WINDOW_HEIGHT as f32 - config.ui_top_bar_height).max(1.0);
        let bounds = vec2(WINDOW_WIDTH as f32, playfield_h);

        Self {
            assets,
            screen: Screen::Menu,
            world: World::new(bounds),
            blood_layer: BloodLayer::new(bounds.x as u32, bounds.y as u32),
            config,
            active_config: config,
            unlocked_max_level: 1,
            selection_box: None,
            command: CommandState::default(),
            hunger_timer: 0.0,
            hunger: config.hunger_start,
            level_transition: None,
            selected_level: 1,
            paused: false,
            debug: false,
            level: 1,
            last_mouse: vec2(0.0, 0.0),
            level_animals_total: 0,

            ui_hint: None,

            theme_cycle: None,

            selection_sfx_cd: 0.0,
        }
    }

    fn unlocked_cap(&self) -> usize {
        if self.debug {
            self.total_levels()
        } else {
            self.unlocked_max_level.clamp(1, self.total_levels())
        }
    }

    fn total_levels(&self) -> usize {
        levels::count().max(1)
    }

    pub fn update(&mut self, input: &InputState) {
        let dt = get_frame_time();

        self.last_mouse = input.mouse;
        self.tick_hint(dt);
        self.update_screen(input);

        // Keep BGM timing running even while paused.
        self.tick_game_theme(dt);

        if self.screen == Screen::InGame && !self.paused {
            self.update_game(input);
        }

        if input.d_pressed {
            self.print_data();
        }
    }

    pub fn draw(&self) {
        match self.screen {
            Screen::Menu => ui::draw_menu(self.assets.main_font.as_ref()),
            Screen::LevelSelect => ui::draw_level_select(
                self.assets.main_font.as_ref(),
                self.selected_level,
                self.unlocked_cap(),
                self.total_levels(),
            ),
            Screen::InGame => {
                render::world::draw_world(
                    &self.world,
                    &self.blood_layer,
                    self.last_mouse,
                    self.selection_box,
                    self.command.last_command,
                    &self.active_config,
                    self.assets.main_font.as_ref(),
                    self.level,
                );

                let paused_for_ui = self.paused && self.level_transition.is_none();
                let selected_nomads = self.world.nomads.iter().filter(|n| n.is_selected()).count();
                ui::draw_ingame_ui(
                    self.assets.main_font.as_ref(),
                    paused_for_ui,
                    self.hunger,
                    self.world.animals.len(),
                    self.level_animals_total,
                    &self.active_config,
                );

                if let Some(t) = self.level_transition {
                    ui::draw_level_complete_overlay(
                        self.assets.main_font.as_ref(),
                        self.level,
                        t.timer,
                        t.next_level.is_none(),
                    );
                } else if !paused_for_ui {
                    ui::draw_hover_label(
                        self.assets.main_font.as_ref(),
                        self.last_mouse,
                        &self.world,
                        selected_nomads,
                        &self.active_config,
                    );
                }
            }
            Screen::GameOver => ui::draw_game_over(self.assets.main_font.as_ref()),
        }

        if let Some(h) = &self.ui_hint {
            ui::draw_hint(self.assets.main_font.as_ref(), h);
        }
    }

    fn set_hint(&mut self, text: impl Into<String>, pos: Vec2, duration: f32) {
        let duration = duration.max(0.05);
        self.ui_hint = Some(UiHint {
            text: text.into(),
            pos,
            ttl: duration,
            duration,
        });
    }

    fn tick_hint(&mut self, dt: f32) {
        let Some(h) = &mut self.ui_hint else {
            return;
        };

        h.ttl -= dt;
        if h.ttl <= 0.0 {
            self.ui_hint = None;
        }
    }

    pub fn pause_game(&mut self, value: bool) {
        self.paused = value;
    }

    #[allow(dead_code)]
    pub fn debug_mode(&mut self, value: bool) {
        self.debug = value;
    }

    fn update_screen(&mut self, input: &InputState) {
        let mut click_accepted = false;

        match self.screen {
            Screen::Menu => {
                if input.enter_pressed {
                    self.screen = Screen::LevelSelect;
                    click_accepted = true;
                }
            }
            Screen::LevelSelect => {
                // Mouse wheel carousel: scroll down -> next level, scroll up -> previous level.
                if input.wheel_y < 0.0 {
                    self.selected_level = (self.selected_level + 1).min(self.total_levels());
                } else if input.wheel_y > 0.0 {
                    self.selected_level = self.selected_level.saturating_sub(1).max(1);
                }

                // Keep selection in range in case levels were removed.
                self.selected_level = self.selected_level.clamp(1, self.total_levels());

                if input.enter_pressed {
                    if self.selected_level <= self.unlocked_cap() {
                        self.start_level(self.selected_level);
                        self.screen = Screen::InGame;
                        click_accepted = true;
                    } else {
                        let w = screen_width();
                        let h = screen_height();
                        let ui_scale = ((w / WINDOW_WIDTH as f32)
                            .min(h / WINDOW_HEIGHT as f32))
                        .clamp(0.75, 1.6);
                        let base_y = h * 0.50;
                        self.set_hint(
                            "Locked. Beat earlier levels.",
                            vec2(w * 0.5, base_y - 62.0 * ui_scale),
                            1.2,
                        );
                    }
                }

                if input.escape_pressed {
                    self.screen = Screen::Menu;
                    click_accepted = true;
                }
            }
            Screen::InGame => {
                if input.escape_pressed && self.level_transition.is_none() {
                    self.pause_game(!self.paused);
                    click_accepted = true;
                }

                if self.paused && input.enter_pressed && self.level_transition.is_none() {
                    self.pause_game(false);
                    self.screen = Screen::LevelSelect;
                    self.stop_game_theme_cycle();
                    click_accepted = true;
                }
            }
            Screen::GameOver => {
                if input.enter_pressed {
                    let lvl = (self.level.max(1) as usize).clamp(1, self.total_levels());
                    self.selected_level = lvl;
                    self.start_level(lvl);
                    self.screen = Screen::InGame;
                    click_accepted = true;
                }

                if input.escape_pressed {
                    let lvl = (self.level.max(1) as usize).clamp(1, self.total_levels());
                    self.selected_level = lvl;
                    self.screen = Screen::Menu;
                    click_accepted = true;
                }
            }
        }

        if click_accepted {
            self.assets.play_click();
        }
    }

    fn start_level(&mut self, level: usize) {
        let Some(spec) = levels::get(level) else {
            // Invalid level id; return to level select.
            self.screen = Screen::LevelSelect;
            self.selected_level = self.total_levels();
            self.level_animals_total = 0;
            self.active_config = self.config;
            return;
        };

        self.start_game_theme_cycle();

        self.level = level as i32;
        self.pause_game(false);

        self.active_config = self.config.apply_overrides(spec.overrides);

        let playfield_h = (WINDOW_HEIGHT as f32 - self.active_config.ui_top_bar_height).max(1.0);
        self.world.bounds = vec2(WINDOW_WIDTH as f32, playfield_h);
        self.blood_layer
            .ensure_size(self.world.bounds.x as u32, self.world.bounds.y as u32);
        self.blood_layer.reset();

        self.level_animals_total = spec.animals;

        self.world.clear_entities();
        self.selection_box = None;
        self.command = CommandState::default();
        self.level_transition = None;

        let bounds = self.world.bounds;
        let spawn_margin = self.config.spawn_margin;

        let nomads_to_spawn = spec.nomads;
        for _ in 0..nomads_to_spawn {
            let pos = vec2(
                gen_range(spawn_margin, bounds.x - spawn_margin),
                gen_range(spawn_margin, bounds.y - spawn_margin),
            );
            self.world.nomads.push(Nomad::new_at(pos));
        }

        let animals_to_spawn = spec.animals;
        for _ in 0..animals_to_spawn {
            let pos = vec2(
                gen_range(spawn_margin, bounds.x - spawn_margin),
                gen_range(spawn_margin, bounds.y - spawn_margin),
            );
            let id = self.world.next_animal_id;
            self.world.next_animal_id += 1;
            self.world
                .animals
                .push(Animal::new_at(id, pos, &self.active_config));
        }

        self.hunger = spec.hunger_start;
        self.hunger_timer = 0.0;
    }

    fn update_game(&mut self, input: &InputState) {
        let dt = get_frame_time();

        self.selection_sfx_cd = (self.selection_sfx_cd - dt).max(0.0);

        // Convert screen mouse -> world mouse (playfield space).
        // Also prevent clicks from going through the top bar.
        let mut winput = *input;
        let bar_h = self.active_config.ui_top_bar_height;
        if winput.mouse.y <= bar_h {
            // Block starting new interactions on the top bar, but allow drag/release
            // to finish if the player moved the mouse into the bar mid-drag.
            winput.left_pressed = false;
            winput.right_pressed = false;
            winput.mouse.y = 0.0;
        } else {
            winput.mouse.y -= bar_h;
        }

        let selection = systems::selection::update(
            &winput,
            &mut self.world,
            &mut self.selection_box,
            &self.active_config,
        );

        // Light feedback when issuing commands without any selection.
        if self.level_transition.is_none()
            && winput.right_pressed
            && !self.world.nomads.iter().any(|n| n.is_selected())
        {
            self.set_hint("Select a nomad first", self.last_mouse, 1.0);
        }

        let mut command_issued = false;
        if self.level_transition.is_none() {
            command_issued = systems::commands::update(
                &winput,
                &mut self.world,
                &mut self.command,
                &self.active_config,
            );
        }

        // Play a random selection line, but gate it with a cooldown.
        if selection.changed && selection.selected && self.selection_sfx_cd <= 0.0 {
            self.assets.play_nomad_selection();
            self.selection_sfx_cd = 1.0;
        }
        if command_issued {
            self.assets.play_click();
        }

        let eats = systems::nomads::update(dt, &mut self.world, &mut self.hunger, &self.active_config);
        if eats > 0 {
            self.assets.play_eating();
        }

        systems::animals::update(dt, &mut self.world, &self.active_config, &mut self.blood_layer);

        let spear_hits = systems::spears::update(dt, &mut self.world, &self.active_config);
        if spear_hits > 0 {
            self.assets.play_hit_animal();
        }

        systems::commands::update_feedback(&self.world, &mut self.command);

        if self.level_transition.is_none() && self.world.animals.is_empty() {
            self.begin_level_transition();
        }

        self.update_level_transition(dt);

        if self.level_transition.is_some() {
            return;
        }

        self.hunger_timer += dt;
        if self.hunger_timer >= self.config.hunger_tick_seconds {
            self.hunger_timer = 0.0;
            self.hunger -= 1;

            if self.hunger <= 0 {
                self.hunger = 0;
                self.screen = Screen::GameOver;
                self.stop_game_theme_cycle();
                self.assets.play_game_over();
            }
        }
    }

    pub fn print_data(&self) {
        if !self.debug {
            return;
        }

        println!("\nGameManager");
        println!("\tscreen: {}", self.screen_name());
        println!(
            "\thunger: {}\thunger_timer: {}",
            self.hunger, self.hunger_timer
        );

        println!("\tnomads:");
        for n in &self.world.nomads {
            let pos = n.get_position();
            println!(
                "\t\t Nomad: Vec2( x:{:.1}, y:{:.1} ) selected:{}",
                pos.x,
                pos.y,
                n.is_selected()
            );
        }

        println!("\tanimals_remaining: {}", self.world.animals.len());
        println!("\tanimals:");
        for a in &self.world.animals {
            let pos = a.get_position();
            println!("\t\t Animal: Vec2( x:{:.1}, y:{:.1} )", pos.x, pos.y);
        }

        println!(
            "\tselected_level: {}\tlevel: {}\tpause: {}\tdebug: {}",
            self.selected_level, self.level, self.paused, self.debug
        );

        println!(
            "\tunlocked_max_level: {} (cap: {})",
            self.unlocked_max_level,
            self.unlocked_cap()
        );
    }

    fn screen_name(&self) -> &'static str {
        match self.screen {
            Screen::Menu => "Menu",
            Screen::LevelSelect => "LevelSelect",
            Screen::InGame => "InGame",
            Screen::GameOver => "GameOver",
        }
    }

    fn begin_level_transition(&mut self) {
        // Don't allow pause overlay during transitions.
        self.pause_game(false);

        self.stop_game_theme_cycle();
        self.assets.play_win();

        if !self.debug {
            let current = self.level.max(1) as usize;
            let next_unlock = (current + 1).min(self.total_levels());
            self.unlocked_max_level = self.unlocked_max_level.max(next_unlock);
        }

        let next_level = (self.level as usize).saturating_add(1);
        let next = if next_level <= self.total_levels() {
            Some(next_level)
        } else {
            None
        };

        self.level_transition = Some(LevelTransition {
            timer: self.config.transition_seconds,
            next_level: next,
        });

        self.command.last_command = None;
    }

    fn start_game_theme_cycle(&mut self) {
        self.assets.stop_game_theme();
        self.theme_cycle = None;

        let Some(seconds) = self.assets.bgm_game_theme_seconds else {
            // If we can't time the track, fall back to seamless looping.
            self.assets.play_game_theme_looped();
            return;
        };

        let seconds = seconds.max(0.05);
        self.assets.play_game_theme_once();
        self.theme_cycle = Some(ThemeCycle {
            phase: ThemePhase::Playing,
            remaining: seconds,
        });
    }

    fn stop_game_theme_cycle(&mut self) {
        self.theme_cycle = None;
        self.assets.stop_game_theme();
    }

    fn tick_game_theme(&mut self, dt: f32) {
        if self.screen != Screen::InGame {
            return;
        }
        if self.level_transition.is_some() {
            return;
        }

        let Some(mut cycle) = self.theme_cycle else {
            return;
        };

        cycle.remaining -= dt;
        if cycle.remaining > 0.0 {
            self.theme_cycle = Some(cycle);
            return;
        }

        match cycle.phase {
            ThemePhase::Playing => {
                // Track ended: wait 6 seconds, then restart.
                self.theme_cycle = Some(ThemeCycle {
                    phase: ThemePhase::Gap,
                    remaining: 6.0,
                });
            }
            ThemePhase::Gap => {
                let seconds = self.assets.bgm_game_theme_seconds.unwrap_or(0.0).max(0.05);
                self.assets.play_game_theme_once();
                self.theme_cycle = Some(ThemeCycle {
                    phase: ThemePhase::Playing,
                    remaining: seconds,
                });
            }
        }
    }

    fn update_level_transition(&mut self, dt: f32) {
        let Some(t) = &mut self.level_transition else {
            return;
        };

        t.timer -= dt;
        if t.timer > 0.0 {
            return;
        }

        let next = t.next_level;
        self.level_transition = None;

        match next {
            Some(lvl) => {
                self.selected_level = lvl;
                self.start_level(lvl);
                self.screen = Screen::InGame;
            }
            None => {
                self.screen = Screen::LevelSelect;
                self.selected_level = self.total_levels();
                self.pause_game(false);
                self.command.last_command = None;
            }
        }
    }
}
