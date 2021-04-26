use bevy::prelude::*;
use sf_core::{
    levels::{LevelMessage, NextLevel},
    ui::{FireCount, LevelCount, SlimeCount, UiHelpMessage},
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

pub fn update_player_message(
    players: Query<&Player>,
    mut messages: Query<&mut Text, With<UiHelpMessage>>,
    level_messages: Query<&LevelMessage>,
) {
    let player = players.single().expect("Should have a spawned player");

    match level_messages.single() {
        Ok(lm) => {
            for mut text in messages.iter_mut() {
                text.sections[0].value = lm.0.clone();
            }
        }
        Err(_) => {
            for mut text in messages.iter_mut() {
                text.sections[0].value = if player.slime_target > 0 {
                    "Find slime, don't touch the ground".into()
                } else {
                    "Reach the ground to progress!".into()
                };
            }
        }
    }
}

pub fn update_player_level(
    next_level: Res<NextLevel>,
    mut levels: Query<&mut Text, With<LevelCount>>,
) {
    for mut text in levels.iter_mut() {
        text.sections[0].value = format!("Level {}", next_level.0 - 1);
    }
}
