use bevy::prelude::*;
use sf_core::{
    dims::Dims,
    entity::{Particle, ParticleType},
    input::InputState,
    map::Map,
    AudioState, Player, Position,
};

const ACCELERATION: f32 = 7.5;
const JUMP_SIZE: f32 = 400.;
const TERMINAL_FALL_VELOCITY: f32 = -450.;
const UPDATE_RATE: f32 = 1. / 60.;
const AIR_SPEED_RATIO: f32 = 2.;

pub fn calculate_player_movement(
    time: Res<Time>,
    input: Res<InputState>,
    dims: Res<Dims>,
    asset_server: Res<AssetServer>,
    audio_state: Res<AudioState>,
    audio: Res<bevy_kira_audio::Audio>,
    mut map: ResMut<Map>,
    mut player_query: Query<(&mut Player, &mut Position, &mut Transform)>,
    particles: Query<(&Particle, Entity)>,
) {
    let t = time.seconds_since_startup();

    for (mut player, mut pos, mut tx) in player_query.iter_mut() {
        // throttle the player controller at approx 60fps
        if t < player.next_update {
            continue;
        }
        player.next_update = t + UPDATE_RATE as f64;

        let was_grounded = player.is_grounded;
        player.is_grounded = is_grounded((pos.0, pos.1), &mut map, &particles);

        if !was_grounded && player.is_grounded {
            player.did_jump = false;
            audio.play_in_channel(asset_server.load("sounds/land.ogg"), &audio_state.channel);
        }

        // calculate accelerations from inputs
        let mut dx: f32 =
            if input.left_pressed { -1. } else { 0. } + if input.right_pressed { 1. } else { 0. };
        if !player.is_grounded {
            // slighlty less mobile in the air?
            dx *= AIR_SPEED_RATIO;
        }

        if dx == 0. && player.velocity.x.abs() > 0. {
            // slow down
            dx = -player.velocity.x.signum();
        }

        let dy = if player.is_grounded {
            if input.jump_pressed && !player.did_jump {
                // jump
                player.did_jump = true;
                audio.play_in_channel(asset_server.load("sounds/jump.ogg"), &audio_state.channel);
                JUMP_SIZE
            } else {
                // stay on the ground
                0.
            }
        } else {
            // fall
            -2.5 * ACCELERATION
        };

        // calculate velocity
        let next_vel = Vec2::new(
            (player.velocity.x + ACCELERATION * dx).clamp(-player.move_speed, player.move_speed),
            (player.velocity.y + dy).max(TERMINAL_FALL_VELOCITY),
        );

        let next_world_pos = tx.translation.truncate() + next_vel * UPDATE_RATE;

        let mut next_grid_pos = dims.world_to_grid(next_world_pos);
        next_grid_pos.1 = next_grid_pos.1.clamp(0, dims.tex_h - 1);

        // check the player can move to the next position
        // TODO move as far as possible if unable to move here rather than just stopping in place
        if !can_move(
            (next_grid_pos.0 as i32, next_grid_pos.1 as i32),
            &mut map,
            &particles,
        ) {
            // stop
            player.velocity = Vec2::ZERO;
            continue;
        }

        pos.0 = next_grid_pos.0;
        pos.1 = next_grid_pos.1;
        player.velocity = next_vel;
        tx.translation = next_world_pos.extend(0.0);
    }
}

/// Returns true if the given position is above "on the ground",
/// usually meaning the grid square below the passed grid square is occupied
/// by a barrier.
fn is_grounded(
    pos: (u32, u32),
    map: &mut ResMut<Map>,
    particles: &Query<(&Particle, Entity)>,
) -> bool {
    !can_move((pos.0 as i32, pos.1 as i32 - 1), map, particles)
}

/// Checks if there is an obstacle at the given position + movement
fn can_move(
    target: (i32, i32),
    map: &mut ResMut<Map>,
    particles: &Query<(&Particle, Entity)>,
) -> bool {
    if target.0 < 0 || target.1 < 0 {
        return false;
    }

    if let Some(entity) = map.get(target.0 as u32, target.1 as u32) {
        match particles.get(entity) {
            Ok((particle, _)) => match particle.particle_type {
                // can't move through an obsctacle
                ParticleType::Obstacle => false,
                // can move through others
                _ => true,
            },
            // nothing found, ok to move
            _ => true,
        }
    } else {
        // no entity at the location
        true
    }
}
