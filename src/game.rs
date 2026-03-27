use crate::entities::*;

pub enum GameState {
    Menu,
    Playing,
    Pause,
    GameOver,
}

pub struct GameManager {
    state: GameState,
    // assets: Assets,
    nomads: Vec<Nomad>,
    animals: Vec<Animal>,
    animals_remaining: usize,
    spawn_timer: f32,
    level: i32,
}

impl GameManager {
    pub fn new() -> Self {
        // load assets

        // create initial nomads
        let mut initial_nomads: Vec<Nomad> = Vec::new();
        for _ in 0..2 {
            initial_nomads.push(Nomad::new());
        }

        // create initial animals
        let mut initial_animals: Vec<Animal> = Vec::new();
        for _ in 0..6 {
            initial_animals.push(Animal::new());
        }

        let num_of_animals = initial_animals.len();

        return GameManager {
            state: GameState::Playing,
            nomads: initial_nomads,
            animals: initial_animals,
            animals_remaining: num_of_animals,
            spawn_timer: 0.0,
            level: 1,
        };
    }

    pub fn print_data(&self) {
        println!("GameManager");
        match self.state {
            GameState::GameOver => println!("\tstate: GameOver"),
            GameState::Playing => println!("\tstate: Playing"),
            GameState::Pause => println!("\tstate: Pause"),
            GameState::Menu => println!("\tstate: Menu"),
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
        println!("\tlevel: {}", self.level);
    }
}
