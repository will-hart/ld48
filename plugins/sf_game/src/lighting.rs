use bevy::{
    math::Vec2,
    prelude::{Query, Res, ResMut},
};
use sf_core::{dims::Dims, render::render_pipeline::LightSource, LightingTarget, Position};

#[derive(Default)]
pub struct LightingStatus {
    pub enabled: bool,
    pub disable_handled: bool,
}

/// Applies a set of points lights as an alpha value.
pub fn point_lighting(
    mut status: ResMut<LightingStatus>,
    mut world: Query<&mut LightSource>,
    dims: Res<Dims>,
    mut lights: Query<(&LightingTarget, &Position)>,
) {
    if !status.enabled {
        if status.disable_handled {
            return;
        }

        // "disable" lighting by setting a massive distance.
        println!("Disabling lighting");
        let mut shader_data = world.single_mut().unwrap();
        shader_data.strength = 100000.;
        status.disable_handled = true;
    }

    // for now should only be one light :(
    match lights.single_mut() {
        Ok((light, pos)) => match world.single_mut() {
            Ok(mut shader_data) => {
                shader_data.strength = light.lighting_strength as f32;
                shader_data.pos = Vec2::new(pos.0 as f32, (dims.tex_h - pos.1) as f32);
            }
            _ => {
                status.enabled = false;
            }
        },
        _ => {
            status.enabled = false;
        }
    }
}
