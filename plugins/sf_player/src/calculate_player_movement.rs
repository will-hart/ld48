use bevy::prelude::*;
use sf_core::{
    dims::Dims,
    entity::{Particle, ParticleType},
    input::InputState,
    map::Map,
    Player, Position,
};

// TODO: jump acceleration
const JUMP_SIZE: u32 = 7;

pub fn calculate_player_movement(
    time: Res<Time>,
    input: Res<InputState>,
    dims: Res<Dims>,
    mut map: ResMut<Map>,
    mut player_query: Query<(&mut Player, &mut Position, &mut Transform)>,
    particles: Query<(&Particle, Entity)>,
) {
    let dx: i32 = if input.left_pressed { -1 } else { 0 } + if input.right_pressed { 1 } else { 0 };
    let t = time.seconds_since_startup();

    for (mut player, mut pos, mut tx) in player_query.iter_mut() {
        if t < player.next_update {
            continue;
        }
        player.next_update = t + (1. / 60.);

        player.is_grounded = if pos.1 > 0 {
            if let Some(entity) = map.get(pos.0, pos.1 - 1) {
                match particles.get(entity) {
                    Ok((particle, _)) => match particle.particle_type {
                        ParticleType::Obstacle => true,
                        _ => false,
                    },
                    _ => false,
                }
            } else {
                false
            }
        } else {
            true
        };

        let new_x = (pos.0 as i32 + dx).clamp(0, dims.tex_w as i32) as u32;

        // update jumping
        // TODO: Check up for obstacles
        if player.is_grounded {
            if input.jump_pressed {
                player.y_vel = JUMP_SIZE;
            } else {
                player.y_vel = 0;
            }
        }

        // check for downward movement
        // TODO: properly account for player sprite size, or set player sprite pivot to feet
        // TODO: Fall damage
        let new_y = if player.is_grounded { pos.1 } else { pos.1 - 1 } + player.y_vel;

        player.y_vel = player.y_vel.checked_sub(1).unwrap_or(player.y_vel);

        pos.0 = new_x;
        pos.1 = new_y;

        tx.translation = dims
            .grid_to_world(pos.0, pos.1, Vec2::new(0., 24.))
            .extend(0.);
    }
}