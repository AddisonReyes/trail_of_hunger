use crate::gameplay_config::GamePlayConfig;
use crate::input::InputState;
use crate::state::SelectionBox;
use crate::world::World;

#[derive(Clone, Copy, Debug, Default)]
pub struct SelectionResult {
    pub changed: bool,
    // True when the selection action resulted in at least one nomad being selected.
    // Used to gate selection SFX.
    pub selected: bool,
}

pub fn update(
    input: &InputState,
    world: &mut World,
    selection_box: &mut Option<SelectionBox>,
    tuning: &GamePlayConfig,
) -> SelectionResult {
    let nomad_select_radius = tuning.selection_nomad_radius;
    let drag_threshold = tuning.selection_drag_threshold;

    let mouse = input.mouse;
    let shift = input.shift_down;

    let mut result = SelectionResult::default();

    if input.left_pressed {
        *selection_box = Some(SelectionBox {
            start: mouse,
            current: mouse,
        });
    }

    if input.left_down
        && let Some(b) = selection_box.as_mut()
    {
        b.current = mouse;
    }

    if !input.left_released {
        return result;
    }

    let Some(b) = selection_box.take() else {
        return result;
    };

    let drag = (b.current - b.start).length();
    if drag < drag_threshold {
        // Click selection.
        let mut best_idx: Option<usize> = None;
        let mut best_d2 = f32::INFINITY;
        for (i, n) in world.nomads.iter().enumerate() {
            let d2 = n.get_position().distance_squared(mouse);
            if n.contains_point(mouse, nomad_select_radius) && d2 < best_d2 {
                best_idx = Some(i);
                best_d2 = d2;
            }
        }

        match best_idx {
            Some(i) => {
                if shift {
                    // Toggle is always a real change.
                    let was = world.nomads[i].is_selected();
                    world.nomads[i].toggle_selected();
                    result.changed = true;
                    result.selected = !was && world.nomads[i].is_selected();
                } else {
                    for n in &mut world.nomads {
                        if n.is_selected() {
                            n.set_selected(false);
                            result.changed = true;
                        }
                    }

                    if !world.nomads[i].is_selected() {
                        world.nomads[i].set_selected(true);
                        result.changed = true;
                    }

                    // If we clicked a nomad and anything changed, treat as a selection action.
                    if result.changed {
                        result.selected = true;
                    }
                }
            }
            None => {
                if !shift {
                    for n in &mut world.nomads {
                        if n.is_selected() {
                            n.set_selected(false);
                            result.changed = true;
                        }
                    }
                }
            }
        }

        return result;
    }

    // Box selection.
    let min_x = b.start.x.min(b.current.x);
    let max_x = b.start.x.max(b.current.x);
    let min_y = b.start.y.min(b.current.y);
    let max_y = b.start.y.max(b.current.y);

    for n in &mut world.nomads {
        let pos = n.get_position();
        let inside = pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y;

        if shift {
            if inside {
                if !n.is_selected() {
                    n.set_selected(true);
                    result.changed = true;
                    result.selected = true;
                }
            }
        } else {
            if n.is_selected() != inside {
                n.set_selected(inside);
                result.changed = true;
                if inside {
                    result.selected = true;
                }
            }
        }
    }

    result
}
