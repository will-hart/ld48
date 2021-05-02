use bevy::prelude::*;
use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::{Particle, Sink, Spawner},
    levels::LevelMessage,
    map::Map,
    render::render_pipeline::LightSource,
    ui::PlayingUiElement,
    Player,
};

pub fn despawner(
    mut commands: Commands,
    dims: Res<Dims>,
    colours: Res<Colors>,
    mut map: ResMut<Map>,
    mut particles: Query<(&Particle, Entity)>,
    mut players: Query<(&Player, Entity)>,
    mut spawners: Query<(&Spawner, Entity)>,
    mut sinks: Query<(&Sink, Entity)>,
    mut ui: Query<(&PlayingUiElement, Entity)>,
    mut level_messages: Query<(&LevelMessage, Entity)>,
    mut lighting: Query<&mut LightSource>,
) {
    // despawn entities and UI
    for (_, ent) in particles.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }

    for (_, ent) in players.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }

    for (_, ent) in ui.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }

    for (_, ent) in level_messages.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }

    for (_, ent) in spawners.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }

    for (_, ent) in sinks.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }

    // turn off the lights
    let mut light = lighting.single_mut().unwrap();
    light.strength = 0.001;
    light.pos = Vec2::ZERO;

    // clear the map
    let bg = to_u8s(colours.background);
    map.clear(&dims, &bg);
}
