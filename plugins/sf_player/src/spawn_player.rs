use bevy::prelude::*;
use sf_core::{dims::Dims, LightingTarget, Player, Position};

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    dims: Res<Dims>,
) {
    let texture_handle = asset_server.load("character.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16., 32.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player_pos = (10, 50);
    let player_tx =
        Transform::from_translation(dims.grid_to_world(player_pos.0, player_pos.1).extend(0.));

    commands
        .spawn()
        .insert(LightingTarget {
            lighting_strength: 15,
            lighting_decay_rate: 10.,
            light_growth_rate: 20,
            next_lighting_decay: 0.,
            max_light_strength: 25,
        })
        .insert(player_tx.clone())
        .insert(GlobalTransform::from_translation(Vec3::ZERO))
        .insert(Timer::from_seconds(0.5, true))
        .insert(Position(player_pos.0, player_pos.1))
        .insert(Player {
            y_vel: 0,
            is_grounded: false,
            frames_since_jumped: 0,
            jump_cooldown: 0,
            did_jump: false,
            next_update: 0.,

            slime_target: 0,
            sink_rate: 1. / 10., // 10 per second
            next_sink: 0.,
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_translation(Vec3::new(0.0, 16.0, 0.0)),
                ..Default::default()
            });
        });

    println!("Spawned player");
}
