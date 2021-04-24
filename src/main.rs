// disable console opening on windows
// TODO: only in release builds
// #![windows_subsystem = "windows"]

use bevy::{prelude::*, render::texture::TextureFormat};
use bevy::{render::texture::Extent3d, DefaultPlugins};

use sf_core::{colors::Colors, dims::Dims, map::Map, CorePlugin, GameState, MainTexture};
use sf_menu::MenuPlugin;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 800;

const TEXTURE_STRIDE: u32 = 4;
const TEXTURE_TYPE: TextureFormat = TextureFormat::Rgba8Unorm;

fn main() {
    let colors = Colors::default();

    let mut app = App::build();

    app.add_state(GameState::Menu)
        .insert_resource(ClearColor(colors.menu))
        .insert_resource(colors)
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            title: "LD48: Sandfall".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(CorePlugin)
        .add_plugin(MenuPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // spawn a camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    // configure the window/texture dimensions
    let dims: Dims = (WINDOW_WIDTH, WINDOW_HEIGHT, TEXTURE_STRIDE).into();
    commands.insert_resource(dims);

    // create the texture to display
    let texture = Texture::new(
        Extent3d::new(dims.tex_w, dims.tex_h, 1),
        bevy::render::texture::TextureDimension::D2,
        vec![0; dims.texture_values()],
        TEXTURE_TYPE,
    );

    // spawn a sprite to display the texture and a resource to hold sprite data
    let th = textures.add(texture.clone());
    let material = materials.add(th.clone().into());

    let main_handles = MainTexture { texture: th };

    commands.insert_resource(main_handles);

    commands.spawn().insert_bundle(SpriteBundle {
        material,
        sprite: Sprite::new(Vec2::new(dims.win_w as f32, dims.win_h as f32)),
        ..Default::default()
    });

    // create the map to track entities
    let map = Map::new(dims, texture);
    commands.insert_resource(map);
}
