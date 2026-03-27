use crate::assets::*;
use crate::entities::*;
use crate::ui::*;

pub enum GameState {
    Playing,
    GameOver,
}

pub struct GameManager {
    state: GameState,
    assets: Assets,
    nomads: Vec<Nomad>,
    animals: Vec<Animal>,
    animals_remaining: usize,
    spawn_timer: f32,
    view: UiState,
    pause: bool,
    debug: bool,
    level: i32,
}

impl GameManager {
    pub async fn new() -> Self {
        let assets_manager = Assets::load().await;
        let view: UiState = UiState::new(assets_manager.main_font.clone());

        let initial_nomads: Vec<Nomad> = Vec::new();
        let initial_animals: Vec<Animal> = Vec::new();
        let num_of_animals = initial_animals.len();

        return GameManager {
            state: GameState::Playing,
            nomads: initial_nomads,
            animals: initial_animals,
            animals_remaining: num_of_animals,
            spawn_timer: 0.0,
            assets: assets_manager,
            debug: false,
            pause: false,
            view: view,
            level: 1,
        };
    }

    pub fn update(&mut self) {
        match self.state {
            GameState::Playing => {}
            GameState::GameOver => {}
        }

        self.view.update();
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn pause_game(&mut self, value: bool) {
        self.pause = value;
    }

    pub fn debug_mode(&mut self, value: bool) {
        self.debug = value;
    }

    pub fn print_data(&self) {
        if !self.debug {
            return;
        }

        println!("GameManager");
        match self.state {
            GameState::GameOver => println!("\tstate: GameOver"),
            GameState::Playing => println!("\tstate: Playing"),
        }
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
        println!("\tpause: {}", self.pause);
        println!("\tdebug: {}", self.debug);
        println!("\tlevel: {}", self.level);
    }
}
