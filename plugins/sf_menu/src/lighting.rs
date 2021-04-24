use bevy::prelude::{Query, Res, ResMut};
use sf_core::{dims::Dims, map::Map, LightingTarget, Player};

const LIGHT_DISTANCE: u32 = 20;
const LIGHT_STOP: u32 = LIGHT_DISTANCE + 5;

#[derive(Default)]
pub struct LightingStatus {
    pub enabled: bool,
    pub disable_handled: bool,
}

pub fn point_lighting(
    mut map: ResMut<Map>,
    dims: Res<Dims>,
    mut status: ResMut<LightingStatus>,
    mut target_query: Query<(&LightingTarget, &Player)>,
) {
    if status.enabled {
        if status.disable_handled {
            return;
        }

        // disable lighting
        for lx in 0..dims.tex_w {
            for ly in 0..dims.tex_h {
                map.set_alpha(&dims, lx, ly, 255);
            }
        }

        status.disable_handled = true;
    }

    match target_query.single_mut() {
        Ok((_, player)) => {
            let x = player.pos.x.floor() as u32;
            let y = player.pos.y.floor() as u32;

            // apply lighting around the player
            for lx in x - x.min(LIGHT_STOP)..(x + LIGHT_STOP).min(dims.tex_w - 1) {
                for ly in y - y.min(LIGHT_STOP)..(y + LIGHT_STOP).min(dims.tex_h - 1) {
                    let dx = x.max(lx) - x.min(lx);
                    let dy = y.max(ly) - y.min(ly);
                    let ratio =
                        (dx * dx + dy * dy) as f64 / (LIGHT_DISTANCE * LIGHT_DISTANCE) as f64;
                    let alpha = 255. * (1. - ratio.clamp(0., 1.));

                    map.set_alpha(&dims, lx, ly, alpha.clamp(0., 255.).floor() as u8);
                }
            }
        }
        _ => {
            status.enabled = false;
        }
    }
}
