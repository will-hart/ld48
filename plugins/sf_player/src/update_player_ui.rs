use bevy::prelude::*;
use sf_core::{
    ui::{FireCount, SlimeCount},
    LightingTarget, Player,
};

pub fn update_player_fire(
    players: Query<&LightingTarget, With<Player>>,
    mut fires: Query<&mut Text, With<FireCount>>,
) {
    let light = players.single().expect("Should have a spawned player");

    for mut text in fires.iter_mut() {
        text.sections[0].value = format!("{}", light.lighting_strength);
    }
}

pub fn update_player_slime(
    players: Query<&Player>,
    mut slimes: Query<&mut Text, With<SlimeCount>>,
) {
    let player = players.single().expect("Should have a spawned player");

    for mut text in slimes.iter_mut() {
        text.sections[0].value = format!("{}", player.slime_target);
    }
}