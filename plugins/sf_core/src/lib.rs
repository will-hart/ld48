use bevy::prelude::*;
use render::render_texture;

pub mod colors;
pub mod dims;
pub mod entity;
pub mod map;
pub mod render;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Playing,
}

pub struct MainTexture {
    pub texture: Handle<Texture>,
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(render_texture.system());
    }
}
