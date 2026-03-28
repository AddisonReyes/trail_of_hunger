use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct InputState {
    pub mouse: Vec2,
    pub wheel_y: f32,
    pub left_pressed: bool,
    pub left_down: bool,
    pub left_released: bool,
    pub right_pressed: bool,
    pub shift_down: bool,

    pub enter_pressed: bool,
    pub escape_pressed: bool,
    pub d_pressed: bool,
}

pub fn gather_input() -> InputState {
    let (mx, my) = mouse_position();
    let (_wx, wy) = mouse_wheel();
    InputState {
        mouse: vec2(mx, my),
        wheel_y: wy,
        left_pressed: is_mouse_button_pressed(MouseButton::Left),
        left_down: is_mouse_button_down(MouseButton::Left),
        left_released: is_mouse_button_released(MouseButton::Left),
        right_pressed: is_mouse_button_pressed(MouseButton::Right),
        shift_down: is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift),

        enter_pressed: is_key_pressed(KeyCode::Enter),
        escape_pressed: is_key_pressed(KeyCode::Escape),
        d_pressed: is_key_pressed(KeyCode::D),
    }
}
