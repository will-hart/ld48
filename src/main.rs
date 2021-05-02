// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

use bevy::{
    prelude::*,
    render::{pipeline::PipelineDescriptor, pipeline::RenderPipeline, texture::TextureFormat},
};
use bevy::{render::texture::Extent3d, DefaultPlugins};
use bevy_kira_audio::{AudioChannel, AudioPlugin};

use sf_core::{
    colors::Colors,
    dims::Dims,
    input::InputState,
    levels::NextLevel,
    map::Map,
    render::render_pipeline::{get_custom_pipeline, LightSource},
    AudioState, CorePlugin, GameState, MainCamera, MainTexture,
};
use sf_game::GamePlugin;
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
            title: "LD48: Slimefall".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CorePlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(GamePlugin)
        .add_startup_system(setup.system())
        .insert_resource(NextLevel(1))
        .run();
}

fn setup(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: ResMut<AssetServer>,
    audio: Res<bevy_kira_audio::Audio>,
    mut shaders: ResMut<Assets<Shader>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    // load audio
    asset_server.load_untyped("sounds/jump.ogg");
    asset_server.load_untyped("sounds/land.ogg");
    asset_server.load_untyped("sounds/pickup.ogg");
    asset_server.load_untyped("sounds/victory.ogg");
    asset_server.load_untyped("sounds/death.ogg");

    // configure audio
    let audio_state = AudioState {
        channel: AudioChannel::new("first".to_owned()),
    };
    audio.set_volume(0.1);
    commands.insert_resource(audio_state);

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
    let black = vec![0, 0, 0, 0];
    let mut initial: Vec<u8> = vec![];
    let bounds = dims.texture_values() / 4;
    for _ in 0..bounds {
        initial.append(&mut black.clone());
    }
    let texture = Texture::new(
        Extent3d::new(dims.tex_w, dims.tex_h, 1),
        bevy::render::texture::TextureDimension::D2,
        initial,
        TEXTURE_TYPE,
    );

    // create a custom shader pipeline for the world sprite
    let pipeline_handle = pipelines.add(get_custom_pipeline(&mut shaders));

    // spawn a sprite to display the texture and a resource to hold sprite data
    let th = textures.add(texture.clone());
    let material = materials.add(th.clone().into());

    let main_handles = MainTexture { texture: th };

    commands.insert_resource(main_handles);

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material,
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            sprite: Sprite::new(Vec2::new(dims.win_w as f32, dims.win_h as f32)),
            ..Default::default()
        })
        .insert(LightSource {
            pos: Vec2::new(50., 20.),
            strength: 30.,
            dims: Vec2::new(dims.tex_w as f32, dims.tex_h as f32),
        });

    // create the map to track entities
    let map = Map::new(dims, texture);
    commands.insert_resource(map);

    // move to loading state
    state.set(GameState::Loading).unwrap();
}
