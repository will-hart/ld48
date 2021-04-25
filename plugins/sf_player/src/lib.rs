use bevy::{math::Vec2, prelude::*};
use sf_core::{
    dims::Dims,
    entity::{Particle, ParticleType},
    input::InputState,
    map::Map,
    GameState, LightingTarget, Player,
};

mod player_sink;
use player_sink::player_sink;

// TODO: jump acceleration
const JUMP_SIZE: u32 = 7;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(spawn_player.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(
                        calculate_player_movement
                            .system()
                            .label("calculate_player_movement"),
                    )
                    .with_system(player_sink.system().after("calculate_player_movement")),
            )
            .add_system(animate_player.system());
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    dims: Res<Dims>,
) {
    let texture_handle = asset_server.load("character.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(5., 11.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player_pos = (10, 50);
    let mut player_tx = Transform::from_scale(Vec3::new(4., 4., 1.));
    player_tx.translation = dims.grid_to_world(player_pos.0, player_pos.1).extend(0.);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: player_tx,
            ..Default::default()
        })
        .insert(LightingTarget)
        .insert(Timer::from_seconds(0.5, true))
        .insert(Player {
            pos: player_pos,
            y_vel: 0,
            is_grounded: false,
            next_update: 0.,
            slime_collected: 0,
            lighting_strength: 15,
        });

    state.set(GameState::Playing).unwrap();
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
    mut player_query: Query<(&mut Player, &mut Transform)>,
    particles: Query<(&Particle, Entity)>,
) {
    let dx: i32 = if input.left_pressed { -1 } else { 0 } + if input.right_pressed { 1 } else { 0 };
    let t = time.seconds_since_startup();

    for (mut player, mut tx) in player_query.iter_mut() {
        if t < player.next_update {
            continue;
        }
        player.next_update = t + (1. / 60.);

        player.is_grounded = if player.pos.1 > 3 {
            if let Some(entity) = map.get(player.pos.0, player.pos.1 - 1) {
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

        let new_x = (player.pos.0 as i32 + dx).clamp(0, dims.tex_w as i32) as u32;

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
        let new_y = if player.is_grounded {
            player.pos.1
        } else {
            player.pos.1 - 1
        } + player.y_vel;

        player.y_vel = player.y_vel.checked_sub(1).unwrap_or(player.y_vel);

        player.pos = (new_x, new_y);

        tx.translation = dims.grid_to_world(player.pos.0, player.pos.1).extend(0.);
    }
}
