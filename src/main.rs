// disable console opening on windows
// TODO: only in release builds
// #![windows_subsystem = "windows"]

use bevy::{prelude::*, render::texture::TextureFormat};
use bevy::{render::texture::Extent3d, DefaultPlugins};

use sf_core::{
    colors::{to_u8s, Colors},
    dims::Dims,
    input::InputState,
    levels::NextLevel,
    map::Map,
    CorePlugin, GameState, MainCamera, MainTexture,
};
use sf_menu::MenuPlugin;
use sf_player::PlayerPlugin;

const WINDOW_WIDTH: u32 = 1200;
const WINDOW_HEIGHT: u32 = 800;

const TEXTURE_STRIDE: u32 = 8;
const TEXTURE_TYPE: TextureFormat = TextureFormat::Rgba8Unorm;

fn main() {
    let colors = Colors::default();

    let mut app = App::build();

    app.add_state(GameState::Menu)
        .insert_resource(ClearColor(colors.background))
        .insert_resource(colors)
        .insert_resource(InputState::default())
        .insert_resource(WindowDescriptor {
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            title: "LD48: Sandfall".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .add_plugin(CorePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MenuPlugin)
        .add_startup_system(setup.system())
        .insert_resource(NextLevel(1))
        .run();
}

fn setup(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    colours: Res<Colors>,
) {
    // spawn a camera
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    commands.spawn_bundle(UiCameraBundle::default());

    // configure the window/texture dimensions
    let dims: Dims = (WINDOW_WIDTH, WINDOW_HEIGHT, TEXTURE_STRIDE).into();
    commands.insert_resource(dims);

    // create the texture to display
    let mut ground = to_u8s(colours.walls).to_vec();
    ground.push(0);

    let mut initial: Vec<u8> = vec![];
    let bounds = dims.texture_values() / 4;
    for _ in 0..bounds {
        initial.append(&mut ground.clone());
    }

    let texture = Texture::new(
        Extent3d::new(dims.tex_w, dims.tex_h, 1),
        bevy::render::texture::TextureDimension::D2,
        initial,
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

    // move to loading state
    state.set(GameState::Loading).unwrap();
}
