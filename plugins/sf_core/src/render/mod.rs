use bevy::prelude::*;

use self::render_pipeline::LightSource;

pub mod render_pipeline;
pub mod render_texture;
pub mod setup_rendering;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(render_texture::render_texture.system())
            .add_asset::<LightSource>()
            .add_startup_system(setup_rendering::setup_rendering.system());
    }
}
