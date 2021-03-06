use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use input::input_capture;
use render::RenderPlugin;

pub mod colors;
pub mod dims;
pub mod entity;
pub mod game_over_ui;
pub mod input;
pub mod levels;
pub mod map;
pub mod render;
pub mod ui;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Loading,
    Playing,
    GameOver,
    Victory,
}

pub struct MainTexture {
    pub texture: Handle<Texture>,
}

// marker structs

pub struct MainCamera;
pub struct StaticEntity;
pub struct GameOver;
pub struct TimedDespawn;

pub struct Position(pub u32, pub u32);

pub struct Player {
    pub velocity: Vec2,
    pub move_speed: f32,
    pub is_grounded: bool,
    pub did_jump: bool,
    pub jump_cooldown: isize,

    pub next_update: f64,
    pub frames_since_jumped: usize,

    pub sink_rate: f64,
    pub slime_target: u32,
    pub next_sink: f64,
}
pub struct LightingTarget {
    pub lighting_strength: u32,
    pub lighting_decay_rate: f64,
    pub next_lighting_decay: f64,
    pub light_growth_rate: u32,
    pub max_light_strength: u32,
}

pub struct AudioState {
    pub channel: AudioChannel,
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_plugin(RenderPlugin)
            .add_system(input_capture.system());
    }
}
