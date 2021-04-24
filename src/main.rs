// disable console opening on windows
// TODO: only in release builds
// #![windows_subsystem = "windows"]

use std::ops::Range;

use bevy::{prelude::*, render::texture::TextureFormat};
use bevy::{render::texture::Extent3d, DefaultPlugins};

use sf_core::colors::{to_u8s, Colors};

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 800;

const TEXTURE_TYPE: TextureFormat = TextureFormat::Rgba8Unorm;
const TEXTURE_STRIDE: u32 = 4;

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

/// https://github.com/dmitriy-shmilo/sio2/blob/master/src/util.rs#L17
#[inline]
pub fn window_size_to_scale(width: usize, height: usize) -> f32 {
    if width == 0 || height == 0 {
        1.
    } else if width < height {
        width as f32 / (WINDOW_WIDTH / TEXTURE_STRIDE) as f32
    } else {
        height as f32 / (WINDOW_HEIGHT / TEXTURE_STRIDE) as f32
    }
}

#[derive(Copy, Clone)]
pub struct Dims {
    pub tex_w: u32,
    pub tex_h: u32,
    pub tex_stride: u32,
    pub win_w: u32,
    pub win_h: u32,
}

impl From<(u32, u32, u32)> for Dims {
    fn from(input: (u32, u32, u32)) -> Self {
        Dims {
            tex_w: input.0 / input.2,
            tex_h: input.1 / input.2,
            tex_stride: input.2,
            win_w: input.0,
            win_h: input.1,
        }
    }
}

impl Dims {
    pub fn to_tex_index(&self, x: u32, y: u32) -> usize {
        ((x + (self.tex_h - y - 1) * self.tex_w) * 4) as usize
    }

    pub fn to_range(&self, x: u32, y: u32) -> Range<usize> {
        let offset = self.to_tex_index(x, y);
        offset..offset + 3
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut textures: ResMut<Assets<Texture>>,
    colors: Res<Colors>,
) {
    let dims: Dims = (WINDOW_WIDTH, WINDOW_HEIGHT, TEXTURE_STRIDE).into();
    commands.insert_resource(dims);

    let mut texture = Texture::new_fill(
        Extent3d::new(dims.tex_w, dims.tex_h, 1),
        bevy::render::texture::TextureDimension::D3,
        vec![120u8; 4 * dims.tex_w as usize * dims.tex_h as usize].as_slice(),
        TEXTURE_TYPE,
    );

    let sand = to_u8s(colors.sand);

    for x in 0..dims.tex_w {
        for y in 0..dims.tex_h {
            for (pixel, idx) in dims.to_range(x, y).enumerate() {
                texture.data[idx] = sand[pixel];
            }
        }
    }

    let th = textures.add(texture);

    let scale = Vec3::splat(window_size_to_scale(
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
    ));

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
    ))));

    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform::from_translation(Vec3::new(0., 10., 10.)));

    commands.spawn().insert_bundle(SpriteBundle {
        mesh: quad_handle.clone(),
        material: materials.add(th.into()),
        transform: Transform::from_scale(scale),
        ..Default::default()
    });
}
