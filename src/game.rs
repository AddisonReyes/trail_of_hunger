use macroquad::prelude::*;
use macroquad::rand::gen_range;

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

#[derive(Clone, Copy, Debug)]
struct SelectionBox {
    start: Vec2,
    current: Vec2,
}

#[derive(Clone, Copy, Debug)]
struct Corpse {
    id: u32,
    pos: Vec2,
    available: bool,
}

#[derive(Clone, Copy, Debug)]
struct Spear {
    pos: Vec2,
    vel: Vec2,
    ttl: f32,
}

#[derive(Clone, Copy, Debug)]
struct LevelTransition {
    timer: f32,
    next_level: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
enum CommandTarget {
    Point(Vec2),
    Animal(u32),
    Corpse(u32),
}

#[derive(Clone, Copy, Debug)]
struct CommandFeedback {
    id: u32,
    click_pos: Vec2,
    target: CommandTarget,
}

pub struct GameManager {
    assets: Assets,
    screen: Screen,

    // Nomads data
    nomads: Vec<Nomad>,
    selection_box: Option<SelectionBox>,
    last_command: Option<CommandFeedback>,
    next_command_id: u32,
    hunger_timer: f32,
    hunger: i32,

    // Animals data
    animals_remaining: usize,
    animals: Vec<Animal>,
    corpses: Vec<Corpse>,
    spears: Vec<Spear>,
    next_animal_id: u32,
    next_corpse_id: u32,

    // Level data
    level_transition: Option<LevelTransition>,
    selected_level: usize,
    paused: bool,
    debug: bool,
    level: i32,
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
            selection_box: None,
            last_command: None,
            next_command_id: 1,
            level_transition: None,
            animals: initial_animals,
            animals_remaining: num_of_animals,
            corpses: Vec::new(),
            spears: Vec::new(),
            next_animal_id: 1,
            next_corpse_id: 1,
            hunger_timer: 0.0,
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

        if is_key_pressed(KeyCode::D) {
            self.print_data();
        }
    }

    pub fn draw(&self) {
        match self.screen {
            Screen::Menu => ui::draw_menu(self.assets.main_font.as_ref()),
            Screen::LevelSelect => {
                ui::draw_level_select(self.assets.main_font.as_ref(), self.selected_level)
            }
            Screen::InGame => {
                self.draw_world();
                ui::draw_ingame_ui(
                    self.assets.main_font.as_ref(),
                    self.paused,
                    self.hunger,
                    self.animals_remaining,
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

        self.nomads.clear();
        self.animals.clear();
        self.corpses.clear();
        self.spears.clear();
        self.selection_box = None;
        self.last_command = None;
        self.level_transition = None;

        let bounds = self.world_bounds();
        let spawn_margin = 20.0;

        let nomads_to_spawn = if level >= 6 { 2 } else { 1 };
        for _ in 0..nomads_to_spawn {
            let pos = vec2(
                gen_range(spawn_margin, bounds.x - spawn_margin),
                gen_range(spawn_margin, bounds.y - spawn_margin),
            );
            self.nomads.push(Nomad::new_at(pos));
        }

        let animals_to_spawn = 3 * level;
        for _ in 0..animals_to_spawn {
            let pos = vec2(
                gen_range(spawn_margin, bounds.x - spawn_margin),
                gen_range(spawn_margin, bounds.y - spawn_margin),
            );
            let id = self.next_animal_id;
            self.next_animal_id += 1;
            self.animals.push(Animal::new_at(id, pos));
        }

        self.hunger = 75;
        self.animals_remaining = self.animals.len();
        self.hunger_timer = 0.0;
    }

    fn update_game(&mut self) {
        let dt = get_frame_time();

        self.update_selection();
        if self.level_transition.is_none() {
            self.update_right_click_orders();
        }

        self.update_nomads_orders(dt);
        self.update_animals(dt);
        self.update_spears(dt);
        self.update_command_feedback();

        if self.level_transition.is_none() && self.animals.is_empty() {
            self.begin_level_transition();
        }

        self.update_level_transition(dt);

        if self.level_transition.is_some() {
            return;
        }

        self.hunger_timer += dt;
        if self.hunger_timer >= 1.0 {
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

        print!("\thunger: {}", self.hunger);
        println!("\thunger_timer: {}", self.hunger_timer);
        println!("\tnomads:");
        for n in &self.nomads {
            let pos = n.get_position();
            println!(
                "\t\t Nomad: Vec2( x:{:.1}, y:{:.1} ) selected:{}",
                pos.x,
                pos.y,
                n.is_selected()
            );
        }

        println!("\tanimals_remaining: {}", self.animals_remaining);
        println!("\tanimals:");
        for n in &self.animals {
            let pos = n.get_position();
            println!("\t\t Animal: Vec2( x:{:.1}, y:{:.1} )", pos.x, pos.y);
        }

        print!("\tselected_level: {}", self.selected_level);
        println!("\tlevel: {}", self.level);
        print!("\tpause: {}", self.paused);
        println!("\tdebug: {}", self.debug);
    }

    fn screen_name(&self) -> &'static str {
        match self.screen {
            Screen::Menu => "Menu",
            Screen::LevelSelect => "LevelSelect",
            Screen::InGame => "InGame",
            Screen::GameOver => "GameOver",
        }
    }

    fn world_bounds(&self) -> Vec2 {
        vec2(ui::WINDOW_WIDTH as f32, ui::WINDOW_HEIGHT as f32)
    }

    fn any_nomad_selected(&self) -> bool {
        self.nomads.iter().any(|n| n.is_selected())
    }

    fn pick_animal_id(&self, point: Vec2, radius: f32) -> Option<u32> {
        let mut best: Option<(u32, f32)> = None;
        for a in &self.animals {
            let d2 = a.get_position().distance_squared(point);
            if d2 <= radius * radius {
                if best.is_none() || d2 < best.unwrap().1 {
                    best = Some((a.id(), d2));
                }
            }
        }
        best.map(|b| b.0)
    }

    fn pick_corpse_id(&self, point: Vec2, radius: f32) -> Option<u32> {
        let mut best: Option<(u32, f32)> = None;
        for c in &self.corpses {
            if !c.available {
                continue;
            }

            let d2 = c.pos.distance_squared(point);
            if d2 <= radius * radius {
                if best.is_none() || d2 < best.unwrap().1 {
                    best = Some((c.id, d2));
                }
            }
        }
        best.map(|b| b.0)
    }

    fn update_selection(&mut self) {
        const NOMAD_SELECT_RADIUS: f32 = 14.0;
        const DRAG_THRESHOLD: f32 = 6.0;

        let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let (mx, my) = mouse_position();
        let mouse = vec2(mx, my);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.selection_box = Some(SelectionBox {
                start: mouse,
                current: mouse,
            });
        }

        if is_mouse_button_down(MouseButton::Left) {
            if let Some(b) = &mut self.selection_box {
                b.current = mouse;
            }
        }

        if !is_mouse_button_released(MouseButton::Left) {
            return;
        }

        let Some(b) = self.selection_box.take() else {
            return;
        };

        let drag = (b.current - b.start).length();
        if drag < DRAG_THRESHOLD {
            // Click selection.
            let mut best_idx: Option<usize> = None;
            let mut best_d2 = f32::INFINITY;
            for (i, n) in self.nomads.iter().enumerate() {
                let d2 = n.get_position().distance_squared(mouse);
                if n.contains_point(mouse, NOMAD_SELECT_RADIUS) && d2 < best_d2 {
                    best_idx = Some(i);
                    best_d2 = d2;
                }
            }

            match best_idx {
                Some(i) => {
                    if shift {
                        self.nomads[i].toggle_selected();
                    } else {
                        for n in &mut self.nomads {
                            n.set_selected(false);
                        }
                        self.nomads[i].set_selected(true);
                    }
                }
                None => {
                    if !shift {
                        for n in &mut self.nomads {
                            n.set_selected(false);
                        }
                    }
                }
            }

            return;
        }

        // Box selection.
        let min_x = b.start.x.min(b.current.x);
        let max_x = b.start.x.max(b.current.x);
        let min_y = b.start.y.min(b.current.y);
        let max_y = b.start.y.max(b.current.y);

        for n in &mut self.nomads {
            let pos = n.get_position();
            let inside = pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y;

            if shift {
                if inside {
                    n.set_selected(true);
                }
            } else {
                n.set_selected(inside);
            }
        }
    }

    fn update_right_click_orders(&mut self) {
        const PICK_RADIUS_ANIMAL: f32 = 22.0;
        const PICK_RADIUS_CORPSE: f32 = 18.0;

        if !is_mouse_button_pressed(MouseButton::Right) {
            return;
        }

        if !self.any_nomad_selected() {
            return;
        }

        let (mx, my) = mouse_position();
        let mouse = vec2(mx, my);

        let command_id = self.next_command_id;
        self.next_command_id += 1;

        if let Some(corpse_id) = self.pick_corpse_id(mouse, PICK_RADIUS_CORPSE) {
            for n in &mut self.nomads {
                if n.is_selected() {
                    n.set_order(NomadOrder::Eat(corpse_id));
                    n.set_order_id(command_id);
                }
            }

            self.last_command = Some(CommandFeedback {
                id: command_id,
                click_pos: mouse,
                target: CommandTarget::Corpse(corpse_id),
            });
            return;
        }

        if let Some(animal_id) = self.pick_animal_id(mouse, PICK_RADIUS_ANIMAL) {
            for n in &mut self.nomads {
                if n.is_selected() {
                    n.set_order(NomadOrder::Hunt(animal_id));
                    n.set_order_id(command_id);
                }
            }

            self.last_command = Some(CommandFeedback {
                id: command_id,
                click_pos: mouse,
                target: CommandTarget::Animal(animal_id),
            });
            return;
        }

        for n in &mut self.nomads {
            if n.is_selected() {
                n.set_order(NomadOrder::MoveTo(mouse));
                n.set_order_id(command_id);
            }
        }

        self.last_command = Some(CommandFeedback {
            id: command_id,
            click_pos: mouse,
            target: CommandTarget::Point(mouse),
        });
    }

    fn update_nomads_orders(&mut self, dt: f32) {
        const NOMAD_SPEED: f32 = 90.0;
        const NOMAD_ATTACK_RANGE: f32 = 60.0;
        const NOMAD_SPEAR_COOLDOWN: f32 = 0.8;
        const SPEAR_SPEED: f32 = 220.0;
        const MOVE_REACH: f32 = 10.0;
        const EAT_RANGE: f32 = 18.0;
        const HUNGER_MAX: i32 = 100;

        let bounds = self.world_bounds();

        for n in &mut self.nomads {
            n.tick_attack_cd(dt);

            match n.order() {
                NomadOrder::Idle => {}
                NomadOrder::MoveTo(target) => {
                    if n.get_position().distance(target) <= MOVE_REACH {
                        n.set_order(NomadOrder::Idle);
                        continue;
                    }

                    if n.move_towards(target, dt, NOMAD_SPEED, bounds) {
                        n.set_order(NomadOrder::Idle);
                    }
                }
                NomadOrder::Hunt(animal_id) => {
                    let animal_pos = self
                        .animals
                        .iter()
                        .find(|a| a.id() == animal_id)
                        .map(|a| a.get_position());

                    let Some(target_pos) = animal_pos else {
                        n.set_order(NomadOrder::Idle);
                        continue;
                    };

                    let dist = n.get_position().distance(target_pos);
                    if dist > NOMAD_ATTACK_RANGE {
                        n.move_towards(target_pos, dt, NOMAD_SPEED, bounds);
                        continue;
                    }

                    if n.can_attack() {
                        let dir = (target_pos - n.get_position()).normalize_or_zero();
                        if dir.length_squared() > 0.0 {
                            self.spears.push(Spear {
                                pos: n.get_position(),
                                vel: dir * SPEAR_SPEED,
                                ttl: 1.5,
                            });
                            n.reset_attack_cd(NOMAD_SPEAR_COOLDOWN);
                        }
                    }
                }
                NomadOrder::Eat(corpse_id) => {
                    let corpse_pos = self
                        .corpses
                        .iter()
                        .find(|c| c.id == corpse_id && c.available)
                        .map(|c| c.pos);

                    let Some(target_pos) = corpse_pos else {
                        n.set_order(NomadOrder::Idle);
                        continue;
                    };

                    let dist = n.get_position().distance(target_pos);
                    if dist > EAT_RANGE {
                        n.move_towards(target_pos, dt, NOMAD_SPEED, bounds);
                        continue;
                    }

                    if let Some(c) = self
                        .corpses
                        .iter_mut()
                        .find(|c| c.id == corpse_id && c.available)
                    {
                        c.available = false;
                        let gain = gen_range(1, 4);
                        self.hunger = (self.hunger + gain).clamp(0, HUNGER_MAX);
                    }

                    n.set_order(NomadOrder::Idle);
                }
            }
        }
    }

    fn update_animals(&mut self, dt: f32) {
        let bounds = self.world_bounds();
        for a in &mut self.animals {
            a.update(dt, &self.nomads, bounds);
        }

        self.animals_remaining = self.animals.len();
    }

    fn update_spears(&mut self, dt: f32) {
        const HIT_RADIUS: f32 = 10.0;
        let bounds = self.world_bounds();

        let mut i = 0;
        while i < self.spears.len() {
            let mut remove_spear = false;
            {
                let s = &mut self.spears[i];
                s.ttl -= dt;
                s.pos += s.vel * dt;

                if s.ttl <= 0.0 {
                    remove_spear = true;
                }

                if s.pos.x < 0.0 || s.pos.x > bounds.x || s.pos.y < 0.0 || s.pos.y > bounds.y {
                    remove_spear = true;
                }

                if !remove_spear {
                    if let Some(ai) = self.animals.iter().position(|a| {
                        a.get_position().distance_squared(s.pos) <= HIT_RADIUS * HIT_RADIUS
                    }) {
                        let animal_id = self.animals[ai].id();
                        self.animals[ai].take_damage(1);
                        remove_spear = true;

                        if self.animals[ai].is_dead() {
                            let dead_pos = self.animals[ai].get_position();
                            self.animals.swap_remove(ai);

                            let corpse_id = self.next_corpse_id;
                            self.next_corpse_id += 1;
                            self.corpses.push(Corpse {
                                id: corpse_id,
                                pos: dead_pos,
                                available: true,
                            });

                            for n in &mut self.nomads {
                                if n.order() == NomadOrder::Hunt(animal_id) {
                                    n.set_order(NomadOrder::Idle);
                                }
                            }
                        }
                    }
                }
            }

            if remove_spear {
                self.spears.swap_remove(i);
            } else {
                i += 1;
            }
        }

        self.animals_remaining = self.animals.len();
    }

    fn update_command_feedback(&mut self) {
        let Some(cmd) = self.last_command else {
            return;
        };

        let any_active = self
            .nomads
            .iter()
            .any(|n| n.order_id() == cmd.id && n.order() != NomadOrder::Idle);

        if !any_active {
            self.last_command = None;
        }
    }

    fn begin_level_transition(&mut self) {
        const TRANSITION_SECONDS: f32 = 4.0;

        let next_level = (self.level as usize).saturating_add(1);
        let next = if next_level <= 10 {
            Some(next_level)
        } else {
            None
        };

        self.level_transition = Some(LevelTransition {
            timer: TRANSITION_SECONDS,
            next_level: next,
        });

        // Clear order feedback so the win overlay is the main signal.
        self.last_command = None;
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
                // Final level completed.
                self.screen = Screen::LevelSelect;
                self.selected_level = 10;
                self.pause_game(false);
                self.last_command = None;
            }
        }
    }

    fn draw_world(&self) {
        clear_background(color_u8!(33, 104, 58, 255));

        const NOMAD_RADIUS: f32 = 8.0;
        const ANIMAL_RADIUS: f32 = 6.0;
        const CORPSE_RADIUS: f32 = 7.0;

        for a in &self.animals {
            let pos = a.get_position();
            draw_circle(pos.x, pos.y, ANIMAL_RADIUS, color_u8!(196, 160, 106, 255));
        }

        for c in &self.corpses {
            let color = if c.available {
                color_u8!(180, 64, 48, 255)
            } else {
                color_u8!(90, 70, 70, 255)
            };
            draw_circle(c.pos.x, c.pos.y, CORPSE_RADIUS, color);
        }

        for s in &self.spears {
            let tail = s.pos - s.vel.normalize_or_zero() * 8.0;
            draw_line(
                tail.x,
                tail.y,
                s.pos.x,
                s.pos.y,
                2.0,
                color_u8!(230, 230, 230, 255),
            );
        }

        if let Some(cmd) = self.last_command {
            draw_circle(cmd.click_pos.x, cmd.click_pos.y, 3.5, WHITE);

            let end = match cmd.target {
                CommandTarget::Point(p) => p,
                CommandTarget::Animal(id) => self
                    .animals
                    .iter()
                    .find(|a| a.id() == id)
                    .map(|a| a.get_position())
                    .unwrap_or(cmd.click_pos),
                CommandTarget::Corpse(id) => self
                    .corpses
                    .iter()
                    .find(|c| c.id == id)
                    .map(|c| c.pos)
                    .unwrap_or(cmd.click_pos),
            };

            for n in &self.nomads {
                if n.order_id() == cmd.id && n.order() != NomadOrder::Idle {
                    let p = n.get_position();
                    draw_line(p.x, p.y, end.x, end.y, 1.0, color_u8!(255, 255, 255, 120));
                }
            }
        }

        for n in &self.nomads {
            let pos = n.get_position();
            let base = color_u8!(215, 226, 255, 255);
            draw_circle(pos.x, pos.y, NOMAD_RADIUS, base);
            if n.is_selected() {
                draw_circle_lines(pos.x, pos.y, NOMAD_RADIUS + 3.0, 2.0, YELLOW);
            }
        }

        // Selection rectangle (RTS-style).
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(b) = self.selection_box {
                const DRAG_THRESHOLD: f32 = 6.0;
                let drag = (b.current - b.start).length();
                if drag >= DRAG_THRESHOLD {
                    let min_x = b.start.x.min(b.current.x);
                    let max_x = b.start.x.max(b.current.x);
                    let min_y = b.start.y.min(b.current.y);
                    let max_y = b.start.y.max(b.current.y);
                    let w = max_x - min_x;
                    let h = max_y - min_y;

                    let fill = color_u8!(255, 255, 255, 28);
                    let border = color_u8!(255, 255, 255, 160);
                    draw_rectangle(min_x, min_y, w, h, fill);
                    draw_rectangle_lines(min_x, min_y, w, h, 2.0, border);
                }
            }
        }
    }
}
