use bevy::{
    math::Vec2,
    prelude::{Commands, IntoSystem, Plugin},
};
use sf_core::{LightingTarget, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_startup_system(spawn_player.system());
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player {
            pos: Vec2::new(1., 1.),
            slime_collected: 0,
        })
        .insert(LightingTarget);
}
