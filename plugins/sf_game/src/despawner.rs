use bevy::prelude::*;
use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
    entity::Particle,
    levels::LevelMessage,
    map::Map,
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
    mut ui: Query<(&PlayingUiElement, Entity)>,
    mut level_messages: Query<(&LevelMessage, Entity)>,
) {
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

    // clear the map
    let bg = to_u8s(colours.background);
    map.clear(&dims, &bg);
}
