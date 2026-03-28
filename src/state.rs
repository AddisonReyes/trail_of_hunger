use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct SelectionBox {
    pub start: Vec2,
    pub current: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub enum CommandTarget {
    Point(Vec2),
    Animal(u32),
    Corpse(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct CommandFeedback {
    pub id: u32,
    pub click_pos: Vec2,
    pub target: CommandTarget,
}

#[derive(Debug)]
pub struct CommandState {
    pub last_command: Option<CommandFeedback>,
    pub next_command_id: u32,
}

impl Default for CommandState {
    fn default() -> Self {
        Self {
            last_command: None,
            next_command_id: 1,
        }
    }
}
