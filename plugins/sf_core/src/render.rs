use bevy::prelude::*;

use crate::{map::Map, MainTexture};

pub fn render_texture(
    main_texture: ResMut<MainTexture>,
    mut textures: ResMut<Assets<Texture>>,
    map: Res<Map>,
) {
    let texture = textures.get_mut(main_texture.texture.clone()).unwrap();
    texture.data.copy_from_slice(&map.raw_texture.data);
}
