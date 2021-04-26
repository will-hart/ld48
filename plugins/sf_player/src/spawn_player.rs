use bevy::prelude::*;
use sf_core::{dims::Dims, LightingTarget, Player, Position};

pub fn spawn_player(
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