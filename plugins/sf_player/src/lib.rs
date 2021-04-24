use bevy::{
    math::Vec2,
    prelude::*,
};
use sf_core::{GameState, LightingTarget, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(spawn_player.system()))
            .add_system(animate_player.system());
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    let texture_handle = asset_server.load("character.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(5., 11.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::new(4., 4., 1.)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.5, true))
        .insert(Player {
            pos: Vec2::new(1., 1.),
            slime_collected: 0,
        })
        .insert(LightingTarget);

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
            sprite.index = ((sprite.index as usize + 1) % (texture_atlas.textures.len() - 1)) as u32;
        }
    }
}