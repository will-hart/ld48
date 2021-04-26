use bevy::prelude::*;
use sf_core::{levels::spawn_level, GameState};

mod animate_player;
mod calculate_player_movement;
mod lighting_decay;
mod player_sink;
mod spawn_player;
mod update_player_ui;

use animate_player::animate_player;
use calculate_player_movement::calculate_player_movement;
use lighting_decay::lighting_decay;
use player_sink::player_sink;
use spawn_player::spawn_player;

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
                )
                .with_system(
                    update_player_ui::update_player_message
                        .system()
                        .after("lighting_decay"),
                ),
        )
        .add_system(animate_player.system());
    }
}
