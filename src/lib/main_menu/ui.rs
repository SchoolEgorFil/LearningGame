use std::time::Duration;

use bevy::{
    prelude::{
        AssetServer, BuildChildren, Button, ButtonBundle, Camera2dBundle, Changed, Color, Commands,
        Component, DespawnRecursiveExt, Entity, ImageBundle, NextState, NodeBundle, Query, Res,
        ResMut, State, TextBundle, With,
    },
    text::{ TextStyle},
    time::Time,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, PositionType,
        Style, UiRect, Val,
    },
};

use crate::{
    lib::tools::{consts::colors, transition::TransitionMarker},
    AppState,
};

use super::components::{
    MainMenuButtonEnum, MainMenuButtonMarker, MainMenuMarker, MainMenuResource, RootNode,
};

pub fn button_interactivity(
    mut main_menu_res: ResMut<MainMenuResource>,
    mut roots: Query<&mut Style, With<RootNode>>,
    mut button_interaction: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuButtonMarker),
        (Changed<Interaction>, With<Button>),
    >,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    if state.get() == &AppState::MainMenu {
        for (interaction, mut color, button_marker) in &mut button_interaction {
            match (&button_marker.0, *interaction) {
                (MainMenuButtonEnum::StartGame, Interaction::Pressed) => {
                    main_menu_res.to_game.started = true;
                }
                (MainMenuButtonEnum::Settings, Interaction::Pressed) => {
                    main_menu_res.to_settings.started = true;
                }
                _ => {}
            }
            match *interaction {
                Interaction::Pressed => {
                    *color = colors::button::DEFAULT_BG_ACTIVE.into();
                }
                Interaction::Hovered => {
                    *color = colors::button::DEFAULT_BG_HOVER.into();
                }
                Interaction::None => {
                    *color = colors::button::DEFAULT_BG.into();
                }
            }
        }

        if main_menu_res.to_game.started {
            if main_menu_res.to_game.tick(time.delta()) {
                next_state.set(AppState::InGame);
            }

            roots.for_each_mut(|mut p| {
                p.top = Val::Percent(0. - main_menu_res.to_game.ease_in_out().unwrap() * 100.)
            });
        }
        if main_menu_res.to_settings.started {
            if !main_menu_res.to_settings.timer.finished() {
                main_menu_res.to_settings.tick(time.delta());
            }
            roots.for_each_mut(|mut p| {
                p.left = Val::Percent(0. - main_menu_res.to_settings.ease_in().unwrap() * 100.)
            });
        }
    }
}

#[derive(Component)]
pub struct MainMenuImageMarker;

pub fn prepare_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(MainMenuMarker);

    commands.insert_resource(MainMenuResource {
        to_game: TransitionMarker::new(false, Duration::from_secs(2)),
        to_settings: TransitionMarker::new(false, Duration::from_secs(1)),
    });

    // commands.spawn(SpriteBundle {
    //     sprite: Sprite {
    //         ..Default::default()
    //     },
    //     texture: asset_server.load("splash/main_screen_sun.png"),
    //
    //     ..Default::default()
    // });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                // padding: UiRect::horizontal(Val::Percent(20.)),
                flex_direction: FlexDirection::Column,
                // justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            // background_color: colors::button::DEFAULT_BG_HOVER.into(),
            ..Default::default()
        })
        .insert(RootNode)
        .insert(MainMenuMarker)
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: bevy::ui::UiImage {
                    texture: asset_server.load("splash/main_screen_sun.png"),
                    ..Default::default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    min_width: Val::Vw(100.),
                    min_height: Val::Vh(100.),
                    width: Val::Px(0.01),
                    height: Val::Px(0.01),
                    aspect_ratio: Some(1920. / 1080.),
                    ..Default::default()
                },
                ..Default::default()
            });
            //button "Start"
            let mut style = TextBundle::from_section(
                "Фізичний офіс ідей",
                TextStyle {
                    font_size: 38.,
                    color: Color::BLACK,
                    font: asset_server.load("fonts\\FiraSans-Bold.ttf"),
                },
            );
            // style.style.width = Val::Percent(100.);
            // style.text.alignment = TextAlignment::Center;
            // style.style.align_items = AlignItems::Center;
            style.style.margin = UiRect::vertical(Val::Px(90.));
            parent.spawn(style);

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        // height: Val::Auto,
                        align_items: AlignItems::Start,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(20.),
                        padding: UiRect::left(Val::Px(30.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    let button_style = Style {
                        min_width: Val::Px(150.),
                        // height: Val::Px(200.0),
                        border: bevy::ui::UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(10.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,

                        ..Default::default()
                    };

                    parent
                        .spawn(ButtonBundle {
                            style: button_style.clone(),
                            // background_color: Color::rgb(0.65,0.65,0.65).into(),
                            ..Default::default()
                        })
                        .insert(MainMenuButtonMarker(MainMenuButtonEnum::StartGame))
                        .with_children(|p| {
                            p.spawn(TextBundle::from_section(
                                "Start",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: button_style.clone(),
                            ..Default::default()
                        })
                        .insert(MainMenuButtonMarker(MainMenuButtonEnum::Settings))
                        .with_children(|p| {
                            p.spawn(TextBundle::from_section(
                                "Settings",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::horizontal(Val::Percent(20.)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                margin: UiRect::left(Val::Percent(100.)),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: colors::button::DEFAULT_BG_ACTIVE.into(),
            ..Default::default()
        })
        .insert(RootNode)
        .insert(MainMenuMarker);
}

pub fn destroy_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuMarker>>) {
    for main_menu_entity in &query {
        commands.entity(main_menu_entity).despawn_recursive();
    }
    commands.remove_resource::<MainMenuResource>();
}
