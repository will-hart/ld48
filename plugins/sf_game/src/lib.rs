use bevy::prelude::*;
use lighting::{point_lighting, LightingStatus};

use sf_core::GameState;

pub mod despawner;
pub mod lighting;
pub mod sand_updater;
pub mod sink_consumption;
pub mod spawner_emission;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum MenuStage {
    Movement,
    Spawning,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(LightingStatus {
            enabled: true,
            disable_handled: false,
        })
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(
                    sink_consumption::sink_consumption
                        .system()
                        .before(MenuStage::Movement),
                )
                .with_system(
                    sand_updater::sand_updater
                        .system()
                        .label(MenuStage::Movement),
                )
                .with_system(
                    spawner_emission::spawner_emission
                        .system()
                        .label(MenuStage::Spawning)
                        .after(MenuStage::Movement),
                )
                .with_system(point_lighting.system().after(MenuStage::Spawning)),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(despawner::despawner.system()),
        );
    }
}

#[derive(Default)]
pub struct LastSpawn {
    pub time: f64,
}
