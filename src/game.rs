use macroquad::prelude::*;

use crate::assets::*;
use crate::entities::*;

use crate::ui;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Menu,
    LevelSelect,
    InGame,
    GameOver,
}

pub struct GameManager {
    screen: Screen,
    assets: Assets,
    nomads: Vec<Nomad>,
    animals: Vec<Animal>,
    animals_remaining: usize,
    spawn_timer: f32,
    paused: bool,
    debug: bool,
    level: i32,
    selected_level: usize,
    hunger: i32,
}

impl GameManager {
    pub async fn new() -> Self {
        let assets_manager = Assets::load().await;

        let initial_nomads: Vec<Nomad> = Vec::new();
        let initial_animals: Vec<Animal> = Vec::new();
        let num_of_animals = initial_animals.len();

        return GameManager {
            screen: Screen::Menu,
            nomads: initial_nomads,
            animals: initial_animals,
            animals_remaining: num_of_animals,
            spawn_timer: 0.0,
            assets: assets_manager,
            debug: false,
            paused: false,
            level: 1,
            selected_level: 1,
            hunger: 75,
        };
    }

    pub fn update(&mut self) {
        self.update_screen();

        if self.screen == Screen::InGame && !self.paused {
            self.update_game();
        }

        if self.screen == Screen::InGame {
            self.print_data();
        }
    }

    pub fn draw(&self) {
        match self.screen {
            Screen::Menu => ui::draw_menu(self.assets.main_font.as_ref()),
            Screen::LevelSelect => {
                ui::draw_level_select(self.assets.main_font.as_ref(), self.selected_level)
            }
            Screen::InGame => ui::draw_ingame_ui(
                self.assets.main_font.as_ref(),
                self.paused,
                self.hunger,
                self.animals_remaining,
            ),
            Screen::GameOver => ui::draw_game_over(self.assets.main_font.as_ref()),
        }
    }

    pub fn pause_game(&mut self, value: bool) {
        self.paused = value;
    }

    pub fn debug_mode(&mut self, value: bool) {
        self.debug = value;
    }

    fn update_screen(&mut self) {
        match self.screen {
            Screen::Menu => {
                if is_key_pressed(KeyCode::Enter) {
                    self.screen = Screen::LevelSelect;
                }
            }
            Screen::LevelSelect => {
                if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                    if self.selected_level < 10 {
                        self.selected_level += 1;
                    }
                }

                if (is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A))
                    && self.selected_level > 1
                {
                    self.selected_level -= 1;
                }

                if is_key_pressed(KeyCode::Enter) {
                    self.start_level(self.selected_level);
                    self.screen = Screen::InGame;
                }

                if is_key_pressed(KeyCode::Escape) {
                    self.screen = Screen::Menu;
                }
            }
            Screen::InGame => {
                if is_key_pressed(KeyCode::Escape) {
                    self.pause_game(!self.paused);
                }

                if self.paused && is_key_pressed(KeyCode::Enter) {
                    self.pause_game(false);
                    self.screen = Screen::LevelSelect;
                }
            }
            Screen::GameOver => {
                if is_key_pressed(KeyCode::Enter) {
                    self.screen = Screen::Menu;
                }
            }
        }
    }

    fn start_level(&mut self, level: usize) {
        self.level = level as i32;
        self.pause_game(false);

        // TODO: inicializar entidades por nivel.
        self.nomads.clear();
        self.animals.clear();

        self.hunger = 75;
        self.animals_remaining = 12;
        self.spawn_timer = 0.0;
    }

    fn update_game(&mut self) {
        // TODO: mover la logica del juego a sistemas/entidades.
        self.spawn_timer += get_frame_time();

        // Dummy: pierde 1 de hambre cada ~1s.
        if self.spawn_timer >= 1.0 {
            self.spawn_timer = 0.0;
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
        // println!("    assets: {},", self.assets);
        println!("\tnomads:");
        for n in &self.nomads {
            let pos: Point = n.get_position();
            println!("\t\t Nomad: Point( x:{}, y:{} )", pos.x, pos.y);
        }

        println!("\tanimals:");
        for n in &self.animals {
            let pos: Point = n.get_position();
            println!("\t\t Animal: Point( x:{}, y:{} )", pos.x, pos.y);
        }

        println!("\tanimals_remaining: {}", self.animals_remaining);
        println!("\tspawn_timer: {}", self.spawn_timer);
        println!("\tpause: {}", self.paused);
        println!("\tdebug: {}", self.debug);
        println!("\tlevel: {}", self.level);
        println!("\tselected_level: {}", self.selected_level);
        println!("\thunger: {}", self.hunger);
    }

    fn screen_name(&self) -> &'static str {
        match self.screen {
            Screen::Menu => "Menu",
            Screen::LevelSelect => "LevelSelect",
            Screen::InGame => "InGame",
            Screen::GameOver => "GameOver",
        }
    }
}
