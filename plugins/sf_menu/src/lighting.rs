use bevy::prelude::{Res, ResMut};
use sf_core::{dims::Dims, input::InputState, map::Map};

const LIGHT_DISTANCE: u32 = 15;
const LIGHT_STOP: u32 = LIGHT_DISTANCE + 7;

pub fn point_lighting(mut map: ResMut<Map>, dims: Res<Dims>, input: Res<InputState>) {
    let x = input.cursor_pos.x.floor() as u32;
    let y = input.cursor_pos.y.floor() as u32;

    if x == 0 && y == 0 {
        // likely due to starting up, ignore
        return;
    }

    for lx in x - x.min(LIGHT_STOP)..(x + LIGHT_STOP).min(dims.tex_w - 1) {
        for ly in y - y.min(LIGHT_STOP)..(y + LIGHT_STOP).min(dims.tex_h - 1) {
            let dx = x.max(lx) - x.min(lx);
            let dy = y.max(ly) - y.min(ly);
            let ratio = (dx * dx + dy * dy) as f64 / (LIGHT_DISTANCE * LIGHT_DISTANCE) as f64;
            let alpha = 255. * (1. - ratio.clamp(0., 1.));

            map.set_alpha(&dims, lx, ly, alpha.clamp(0., 255.).floor() as u8);
        }
    }
}
