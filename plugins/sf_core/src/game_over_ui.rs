use bevy::prelude::*;

use crate::{levels::NextLevel, GameState};
pub struct GameOverUi;

pub fn despawn_game_over_ui(mut commands: Commands, mut items: Query<Entity, With<GameOverUi>>) {
    for ent in items.iter_mut() {
        commands.entity(ent).despawn_recursive();
    }
}

pub fn restart_game_watcher(
    keys: Res<Input<KeyCode>>,
    mut next_level: ResMut<NextLevel>,
    mut state: ResMut<State<GameState>>,
) {
    if keys.pressed(KeyCode::R) {
        next_level.0 = 1;
        state.set(GameState::Loading).unwrap();
    }
}

pub fn spawn_game_over_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<State<GameState>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(GameOverUi)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    match state.current() {
                        GameState::GameOver => "GAME OVER (collect the slime, then drop)",
                        GameState::Victory => "You won!",
                        _ => "Huh?",
                    },
                    TextStyle {
                        font: asset_server.load("fonts/PressStart2P-Regular.ttf"),
                        font_size: 16.,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
            parent.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "Press 'r' to try again",
                    TextStyle {
                        font: asset_server.load("fonts/PressStart2P-Regular.ttf"),
                        font_size: 12.,
                        color: Color::WHITE,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

    println!("Spawned game UI");
}
