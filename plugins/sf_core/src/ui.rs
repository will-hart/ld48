use bevy::prelude::*;

pub struct SlimeCount;
pub struct FireCount;
pub struct LevelCount;
pub struct UiHelpMessage;
pub struct PlayingUiElement;

pub fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(PlayingUiElement)
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(16.0)),
                        justify_content: JustifyContent::FlexEnd,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .with_children(|container| {
                    // display slime target
                    container.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(16.0), Val::Auto),
                            ..Default::default()
                        },
                        material: materials.add(asset_server.load("slime.png").into()),
                        ..Default::default()
                    });

                    container
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "0",
                                TextStyle {
                                    font: asset_server.load("fonts/PressStart2P-Regular.ttf"),
                                    font_size: 12.,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(SlimeCount);

                    // display light value
                    container.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(16.0), Val::Auto),
                            ..Default::default()
                        },
                        material: materials.add(asset_server.load("light.png").into()),
                        ..Default::default()
                    });

                    container
                        .spawn_bundle(TextBundle {
                            style: Style {
                                margin: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "0",
                                TextStyle {
                                    font: asset_server.load("fonts/PressStart2P-Regular.ttf"),
                                    font_size: 12.,
                                    color: Color::WHITE,
                                },
                                Default::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(FireCount);
                });

            // display the current level
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Level 1",
                        TextStyle {
                            font: asset_server.load("fonts/PressStart2P-Regular.ttf"),
                            font_size: 10.,
                            color: Color::rgb(0.5, 0.5, 0.5),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(LevelCount);

            // display help message
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        margin: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/PressStart2P-Regular.ttf"),
                            font_size: 10.,
                            color: Color::rgb(0.5, 0.5, 0.5),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(UiHelpMessage);
        });

    println!("Spawned game UI");
}
