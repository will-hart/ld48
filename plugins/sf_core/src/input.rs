use bevy::{input::mouse::MouseButtonInput, prelude::*};

use crate::dims::Dims;

#[derive(Default)]
pub struct InputState {
    pub mouse_down: bool,
    pub cursor_pos: Vec2,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub jump_pressed: bool,
}

pub fn input_capture(
    keys: Res<Input<KeyCode>>,
    mut evr_cursor: EventReader<CursorMoved>,
    mut evr_click: EventReader<MouseButtonInput>,
    dims: Res<Dims>,
    mut input: ResMut<InputState>,
) {
    for ev in evr_cursor.iter() {
        input.cursor_pos = dims.screen_to_grid(ev.position);
    }

    for ev in evr_click.iter() {
        if ev.button == MouseButton::Right {
            input.mouse_down = ev.state.is_pressed();
        }
    }

    input.left_pressed = keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left);
    input.right_pressed = keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right);
    input.jump_pressed = keys.pressed(KeyCode::Space)
        || keys.pressed(KeyCode::LShift)
        || keys.pressed(KeyCode::RShift)
        || keys.pressed(KeyCode::Up);
}
