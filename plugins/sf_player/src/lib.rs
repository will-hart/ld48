use bevy::prelude::*;
use sf_core::{
    game_over_ui::{despawn_game_over_ui, restart_game_watcher, spawn_game_over_ui},
    levels::spawn_level,
    ui::spawn_ui,
    GameState,
};

mod animate_player;
mod calculate_player_movement;
mod game_over_tracker;
mod lighting_decay;
mod player_sink;
mod spawn_player;
mod sync_transform_to_pos;
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
                .with_system(spawn_player.system().label("spawn_player"))
                .with_system(spawn_ui.system()),
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
                        .label("calculate_player_movement")
                        .before("game_over_tracker"),
                )
                .with_system(
                    sync_transform_to_pos::sync_transform_to_pos
                        .system()
                        .label("transform_sync_to_player")
                        .after("calculate_player_movement"),
                )
                .with_system(player_sink.system().after("calculate_player_movement"))
                .with_system(
                    lighting_decay
                        .system()
                        .label("lighting_decay")
                        .after("transform_sync_to_player")
                        .before("game_over_tracker"),
                )
                .with_system(
                    update_player_ui::update_player_fire
                        .system()
                        .after("lighting_decay")
                        .before("game_over_tracker"),
                )
                .with_system(
                    update_player_ui::update_player_slime
                        .system()
                        .after("lighting_decay")
                        .before("game_over_tracker"),
                )
                .with_system(
                    update_player_ui::update_player_message
                        .system()
                        .after("lighting_decay")
                        .before("game_over_tracker"),
                )
                .with_system(
                    update_player_ui::update_player_level
                        .system()
                        .after("lighting_decay")
                        .before("game_over_tracker"),
                )
                .with_system(
                    game_over_tracker::game_over_tracker
                        .system()
                        .label("game_over_tracker")
                        .after("transform_sync_to_player"),
                ),
        )
        .add_system(animate_player.system())
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(spawn_game_over_ui.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver).with_system(restart_game_watcher.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver).with_system(despawn_game_over_ui.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Victory).with_system(spawn_game_over_ui.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Victory).with_system(restart_game_watcher.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Victory).with_system(despawn_game_over_ui.system()),
        );
    }
}
