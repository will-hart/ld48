use bevy::prelude::*;
use sf_core::{
    dims::Dims,
    entity::{Particle, ParticleType},
    input::InputState,
    map::Map,
    AudioState, Player, Position,
};

fn move_towards(current: f32, target: f32, max: f32) -> f32 {
    let delta = target - current;

    current + delta.abs().max(max.abs()) * delta.signum()
}

// TODO: jump acceleration
const JUMP_COOLDOWN: isize = 10;
const JUMP_HEIGHT: f32 = 10.;
const GRAVITY: f32 = 50.;

pub fn calculate_player_movement(
    time: Res<Time>,
    input: Res<InputState>,
    dims: Res<Dims>,
    asset_server: Res<AssetServer>,
    audio_state: Res<AudioState>,
    audio: Res<bevy_kira_audio::Audio>,
    mut map: ResMut<Map>,
    mut player_query: Query<(&mut Player, &Position, &mut Transform)>,
    particles: Query<(&Particle, Entity)>,
) {
    let dt = time.delta_seconds();

    for (mut player, pos, mut tx) in player_query.iter_mut() {
        player.is_grounded = !can_move((pos.0 as i32, pos.1 as i32 - 1), &mut map, &particles);
        let mut v = player.vel.clone();

        let dx = if input.left_pressed { -1 } else { 0 } + if input.right_pressed { 1 } else { 0 };

        // handle input (slowing down) vs input (accelerating)
        if dx == 0 {
            // slowing down
            v.x = move_towards(player.vel.x, 0., player.x_decel * dt);
        } else {
            // speeding up
            v.x = move_towards(
                player.vel.x,
                dx as f32 * player.move_speed,
                if player.is_grounded {
                    player.x_accel
                } else {
                    player.air_x_accel
                } * dt,
            );
        }

        // calculate jump velocity
        if player.is_grounded {
            if player.frames_since_jumped > 0 {
                // just landed
                audio.play_in_channel(asset_server.load("sounds/land.ogg"), &audio_state.channel);
                player.jump_cooldown = JUMP_COOLDOWN;
            }

            player.frames_since_jumped = 0;
            player.jump_cooldown -= 1;

            if input.jump_pressed && player.jump_cooldown < 0 {
                // just jumped
                audio.play(asset_server.load("sounds/jump.ogg"));
                player.jump_cooldown = 0;
                player.frames_since_jumped = 1;
                v.y = (5. * JUMP_HEIGHT * GRAVITY).sqrt();
                println!("{}", v.y);
            } else {
                // tried to jump but on the ground - stay put
                v.y = 0.;
            }
        } else {
            // player is jumping, just count the jump frames
            // TODO: we can use jump frames to calculate damage
            player.frames_since_jumped += 1;

            // apply gravity
            v.y -= GRAVITY;
        }

        // check if the move will translate the player into an obstacle, if so, set velocity to 0
        let (target_x, target_y) =
            dims.world_to_grid(tx.translation.truncate() + v + Vec2::new(0., 24.));
        if can_move((target_x as i32, target_y as i32), &mut map, &particles) {
            // actually translate the transform, the pos will be updated in a separate system (sync_transform_to_pos)
            tx.translation = tx.translation + v.extend(0.) * dt;
        }
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
