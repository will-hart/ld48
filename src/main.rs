// disable console opening on windows
// TODO: only in release builds
// #![windows_subsystem = "windows"]

use bevy::{prelude::*, render::texture::TextureFormat};
use bevy::{render::texture::Extent3d, DefaultPlugins};

use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
};

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 800;

const TEXTURE_STRIDE: u32 = 4;
const TEXTURE_TYPE: TextureFormat = TextureFormat::Rgba8Unorm;

fn main() {
    let colors = Colors::default();

    let mut app = App::build();

    app.insert_resource(ClearColor(colors.menu))
        .insert_resource(colors)
        .insert_resource(WindowDescriptor {
            width: WINDOW_HEIGHT as f32,
            height: WINDOW_WIDTH as f32,
            title: "LD48: Sandfall".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    colours: Res<Colors>,
) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());

    let dims: Dims = (WINDOW_WIDTH, WINDOW_HEIGHT, TEXTURE_STRIDE).into();
    commands.insert_resource(dims);

    let mut texture = Texture::new(
        Extent3d::new(dims.tex_w, dims.tex_h, 1),
        bevy::render::texture::TextureDimension::D2,
        vec![0; dims.texture_values()],
        TEXTURE_TYPE,
    );

    let bg = to_u8s(colours.menu);

    for x in 0..dims.tex_w {
        for y in 0..dims.tex_h {
            for (pixel, idx) in dims.to_range(x, y).enumerate() {
                texture.data[idx] = bg[pixel];
            }
        }
    }

    let th = textures.add(texture);
    let material = materials.add(th.into());

    commands.spawn().insert_bundle(SpriteBundle {
        material,
        sprite: Sprite::new(Vec2::new(dims.win_w as f32, dims.win_h as f32)),
        ..Default::default()
    });
}
