use bevy::{math::Vec2, prelude::*};
use sf_core::{
    dims::Dims,
    entity::{Particle, ParticleType},
    input::InputState,
    levels::spawn_level,
    map::Map,
    GameState, LightingTarget, Player, Position,
};

mod lighting_decay;
mod player_sink;
mod update_player_ui;

use lighting_decay::lighting_decay;
use player_sink::player_sink;

// TODO: jump acceleration
const JUMP_SIZE: u32 = 7;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading)
                .with_system(spawn_player.system().label("spawn_player")),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Loading)
                .with_system(spawn_level.system().after("spawn_player")),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(
                    calculate_player_movement
                        .system()
                        .label("calculate_player_movement"),
                )
                .with_system(player_sink.system().after("calculate_player_movement"))
                .with_system(
                    lighting_decay
                        .system()
                        .label("lighting_decay")
                        .after("calculate_player_movement"),
                )
                .with_system(
                    update_player_ui::update_player_fire
                        .system()
                        .after("lighting_decay"),
                )
                .with_system(
                    update_player_ui::update_player_slime
                        .system()
                        .after("lighting_decay"),
                ),
        )
        .add_system(animate_player.system());
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    dims: Res<Dims>,
) {
    let texture_handle = asset_server.load("character.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(5., 11.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player_pos = (10, 50);
    let mut player_tx = Transform::from_scale(Vec3::new(4., 4., 1.));
    player_tx.translation = dims
        .grid_to_world(player_pos.0, player_pos.1, Vec2::new(0., 24.))
        .extend(0.);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: player_tx,
            ..Default::default()
        })
        .insert(LightingTarget {
            lighting_strength: 15,
            lighting_decay_rate: 10.,
            light_growth_rate: 5,
            next_lighting_decay: 0.,
            max_light_strength: 25,
        })
        .insert(Timer::from_seconds(0.5, true))
        .insert(Position(player_pos.0, player_pos.1))
        .insert(Player {
            y_vel: 0,
            is_grounded: false,
            next_update: 0.,

            slime_target: 0,
            sink_rate: 1. / 5., // 5 per second
            next_sink: 0.,
        });

    println!("Spawned player");
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index =
                ((sprite.index as usize + 1) % (texture_atlas.textures.len() - 1)) as u32;
        }
    }
}

fn calculate_player_movement(
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
