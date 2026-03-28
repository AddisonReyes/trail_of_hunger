use macroquad::prelude::*;
use macroquad::rand::gen_range;

use crate::assets::Assets;
use crate::entities::{Animal, Nomad};
use crate::gameplay_config::GamePlayConfig;
use crate::gameplay_config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::input::InputState;
use crate::state::{CommandState, SelectionBox};
use crate::world::World;
use crate::{render, systems, ui};

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

pub struct GameManager {
    assets: Assets,
    screen: Screen,
    world: World,
    config: GamePlayConfig,

    selection_box: Option<SelectionBox>,
    command: CommandState,

    hunger_timer: f32,
    hunger: i32,

    level_transition: Option<LevelTransition>,
    selected_level: usize,
    paused: bool,
    debug: bool,
    level: i32,
}

impl GameManager {
    pub async fn new() -> Self {
        let assets = Assets::load().await;
        let config = GamePlayConfig::default();
        let bounds = vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

        Self {
            assets,
            screen: Screen::Menu,
            world: World::new(bounds),
            config,
            selection_box: None,
            command: CommandState::default(),
            hunger_timer: 0.0,
            hunger: config.hunger_start,
            level_transition: None,
            selected_level: 1,
            paused: false,
            debug: false,
            level: 1,
        }
    }

    pub fn update(&mut self, input: &InputState) {
        self.update_screen(input);

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
                self.config.max_levels,
            ),
            Screen::InGame => {
                render::world::draw_world(
                    &self.world,
                    self.selection_box,
                    self.command.last_command,
                    &self.config,
                );
                ui::draw_ingame_ui(
                    self.assets.main_font.as_ref(),
                    self.paused,
                    self.hunger,
                    self.world.animals.len(),
                );

                if let Some(t) = self.level_transition {
                    ui::draw_level_complete_overlay(
                        self.assets.main_font.as_ref(),
                        self.level,
                        t.timer,
                        t.next_level.is_none(),
                    );
                }
            }
            Screen::GameOver => ui::draw_game_over(self.assets.main_font.as_ref()),
        }
    }

    pub fn pause_game(&mut self, value: bool) {
        self.paused = value;
    }

    pub fn debug_mode(&mut self, value: bool) {
        self.debug = value;
    }

    fn update_screen(&mut self, input: &InputState) {
        match self.screen {
            Screen::Menu => {
                if input.enter_pressed {
                    self.screen = Screen::LevelSelect;
                }
            }
            Screen::LevelSelect => {
                if (input.right_pressed_key || input.d_pressed)
                    && self.selected_level < self.config.max_levels
                {
                    self.selected_level += 1;
                }

                if (input.left_pressed_key || input.a_pressed) && self.selected_level > 1 {
                    self.selected_level -= 1;
                }

                if input.enter_pressed {
                    self.start_level(self.selected_level);
                    self.screen = Screen::InGame;
                }

                if input.escape_pressed {
                    self.screen = Screen::Menu;
                }
            }
            Screen::InGame => {
                if input.escape_pressed {
                    self.pause_game(!self.paused);
                }

                if self.paused && input.enter_pressed {
                    self.pause_game(false);
                    self.screen = Screen::LevelSelect;
                }
            }
            Screen::GameOver => {
                if input.enter_pressed {
                    self.screen = Screen::Menu;
                }
            }
        }
    }

    fn start_level(&mut self, level: usize) {
        self.level = level as i32;
        self.pause_game(false);

        self.world.clear_entities();
        self.selection_box = None;
        self.command = CommandState::default();
        self.level_transition = None;

        let bounds = self.world.bounds;
        let spawn_margin = self.config.spawn_margin;

        let nomads_to_spawn = self.config.nomads_to_spawn(level);
        for _ in 0..nomads_to_spawn {
            let pos = vec2(
                gen_range(spawn_margin, bounds.x - spawn_margin),
                gen_range(spawn_margin, bounds.y - spawn_margin),
            );
            self.world.nomads.push(Nomad::new_at(pos));
        }

        let animals_to_spawn = self.config.animals_to_spawn(level);
        for _ in 0..animals_to_spawn {
            let pos = vec2(
                gen_range(spawn_margin, bounds.x - spawn_margin),
                gen_range(spawn_margin, bounds.y - spawn_margin),
            );
            let id = self.world.next_animal_id;
            self.world.next_animal_id += 1;
            self.world
                .animals
                .push(Animal::new_at(id, pos, &self.config));
        }

        self.hunger = self.config.hunger_start;
        self.hunger_timer = 0.0;
    }

    fn update_game(&mut self, input: &InputState) {
        let dt = get_frame_time();

        systems::selection::update(
            input,
            &mut self.world,
            &mut self.selection_box,
            &self.config,
        );
        if self.level_transition.is_none() {
            systems::commands::update(input, &mut self.world, &mut self.command, &self.config);
        }

        systems::nomads::update(dt, &mut self.world, &mut self.hunger, &self.config);
        systems::animals::update(dt, &mut self.world, &self.config);
        systems::spears::update(dt, &mut self.world, &self.config);
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
        let next_level = (self.level as usize).saturating_add(1);
        let next = if next_level <= self.config.max_levels {
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
                self.selected_level = self.config.max_levels;
                self.pause_game(false);
                self.command.last_command = None;
            }
        }
    }
}
