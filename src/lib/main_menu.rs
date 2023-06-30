use std::time::Duration;

use bevy::prelude::*;

use super::{transition::TransitionMarker, AppState, colors};

#[derive(Component)]
pub struct MainMenuMarker;

#[derive(Resource)]
pub struct MainMenuResource {
    to_game: TransitionMarker,
    to_settings: TransitionMarker,
}

enum MainMenuButtonEnum {
    start_game,
    settings,
    exit
}

#[derive(Component)]
pub struct MainMenuButtonMarker(MainMenuButtonEnum);

#[derive(Component)]
pub struct RootNode;

pub fn button_interactivity(
        mut main_menu_res: ResMut<MainMenuResource>,
        mut roots: Query<&mut Style, With<RootNode>>,
        mut button_interaction: Query<
                (&Interaction, &mut BackgroundColor, &MainMenuButtonMarker),
                (Changed<Interaction>, With<Button>)>, 
        state: Res<State<AppState>>,
        mut next_state: ResMut<NextState<AppState>>,
        time: Res<Time>
) {
        if state.0==AppState::MainMenu {
            for (interaction, mut color, button_marker) in &mut button_interaction {
                match (&button_marker.0, *interaction) {
                    (MainMenuButtonEnum::start_game,Interaction::Clicked) => {
                        main_menu_res.to_game.started = true;
                    },
                    (MainMenuButtonEnum::settings,Interaction::Clicked) => {
                        main_menu_res.to_settings.started = true;
                    }
                    _ => {}
                }
                match *interaction {
                    Interaction::Clicked => {
                        *color = colors::button::DEFAULT_BG_ACTIVE.into();
                    },
                    Interaction::Hovered => {
                        *color = colors::button::DEFAULT_BG_HOVER.into();
                    },
                    Interaction::None => {
                        *color = colors::button::DEFAULT_BG.into();
                    },
                    
                }
            }

            if main_menu_res.to_game.started {
                if main_menu_res.to_game.tick(time.delta()) {
                    next_state.set(AppState::InGame);
                }
                
                roots.for_each_mut(|mut p| p.position.top = Val::Percent(0.-main_menu_res.to_game.ease_in_out().unwrap()*100.));
            }
            if main_menu_res.to_settings.started {
                if !main_menu_res.to_settings.timer.finished() {
                    main_menu_res.to_settings.tick(time.delta());
                }
                roots.for_each_mut(|mut p| p.position.left = Val::Percent(0.-main_menu_res.to_settings.ease_in().unwrap()*100.));
            }
        }
}

pub fn prepare_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, ) {
    commands.spawn(Camera2dBundle::default()).insert(MainMenuMarker);

    commands
        .insert_resource(MainMenuResource { 
            to_game: TransitionMarker::new(false,Duration::from_secs(2)),
            to_settings: TransitionMarker::new(false,Duration::from_secs(1))
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.)),
                padding: UiRect::horizontal(Val::Percent(20.)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceEvenly,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: colors::button::DEFAULT_BG_HOVER.into(),
            ..Default::default()
        })
        .insert(RootNode)
        .insert(MainMenuMarker)
        .with_children(|parent| { //button "Start"
            parent
                .spawn(ButtonBundle {
                    style: Style { 
                        size: Size::height(Val::Px(200.0)),
                        border: bevy::ui::UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        
                        ..Default::default()
                    },
                    
                    // background_color: Color::rgb(0.65,0.65,0.65).into(),
                    ..Default::default()
                })
                .insert(MainMenuButtonMarker(MainMenuButtonEnum::start_game))
                .with_children(|p| {p.spawn(TextBundle::from_section("Start", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9,0.9,0.9)
                }));});

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::height(Val::Px(200.0)),
                        border: bevy::ui::UiRect::all(Val::Px(2.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(MainMenuButtonMarker(MainMenuButtonEnum::settings))
                .with_children(|p| {p.spawn(TextBundle::from_section("Settings", TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9,0.9,0.9)
                }));});
        }); 

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Percent(100.)),
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

pub fn destroy_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuMarker>> ) {
    for main_menu_entity in &query {
        commands.entity(main_menu_entity).despawn_recursive();
    }
    commands
        .remove_resource::<MainMenuResource>();
}
