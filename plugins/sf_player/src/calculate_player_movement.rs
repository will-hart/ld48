use bevy::prelude::*;
use sf_core::{
    dims::Dims,
    entity::{Particle, ParticleType},
    input::InputState,
    map::Map,
    AudioState, Player, Position,
};

// TODO: jump acceleration
const NUM_JUMP_FRAMES: usize = 10;
const JUMP_SIZE: [u32; NUM_JUMP_FRAMES] = [2, 3, 4, 2, 2, 1, 1, 1, 1, 1];
const JUMP_COOLDOWN: isize = 12;

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
        if t < player.next_update {
            continue;
        }
        player.next_update = t + (1. / 60.);

        player.is_grounded = !can_move((pos.0 as i32, pos.1 as i32 - 1), &mut map, &particles);

        let dx = if input.left_pressed { -1 } else { 0 } + if input.right_pressed { 1 } else { 0 };
        let new_x = (pos.0 as i32 + dx).clamp(0, dims.tex_w as i32) as u32;

        // update jumping
        // TODO: Check up for obstacles
        if player.is_grounded {
            if player.frames_since_jumped > 0 {
                // just landed
                audio.play_in_channel(asset_server.load("sounds/land.ogg"), &audio_state.channel);
                player.jump_cooldown = JUMP_COOLDOWN;
            }

            player.frames_since_jumped = 0;
            player.jump_cooldown -= 1;
            player.did_jump = false;

            if input.jump_pressed && player.jump_cooldown < 0 {
                // just jumped
                audio.play(asset_server.load("sounds/jump.ogg"));
                player.jump_cooldown = 0;
                player.did_jump = true;
                player.frames_since_jumped = 1;
                player.y_vel = JUMP_SIZE[0];
            } else {
                player.y_vel = 0;
            }
        } else {
            if player.frames_since_jumped > 0
                && player.did_jump
                && player.frames_since_jumped < NUM_JUMP_FRAMES
            {
                player.y_vel = JUMP_SIZE[player.frames_since_jumped];
            }

            player.frames_since_jumped += 1;
        }

        // check for downward movement
        // TODO: properly account for player sprite size
        // TODO: Fall damage
        let new_y = if player.is_grounded { pos.1 } else { pos.1 - 1 } + player.y_vel;
        player.y_vel = player.y_vel.checked_sub(1).unwrap_or(player.y_vel);

        // check player can move there
        if !can_move((new_x as i32, new_y as i32), &mut map, &particles) {
            continue;
        }

        pos.0 = new_x;
        pos.1 = new_y;

        tx.translation = dims.grid_to_world(pos.0, pos.1).extend(0.);
    }
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
