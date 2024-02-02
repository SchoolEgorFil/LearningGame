// #![windows_subsystem = "windows"]


use bevy::{
    // diagnostic::FrameTimeDiagnosticsPlugin,
    // gltf::GltfPlugin,
    // prelude::StandardMaterial,
    prelude::*,
    // render::{settings::WgpuFeatures, RenderPlugin},
     pbr::DefaultOpaqueRendererMethod,
};
// use std::time::Duration;

use bevy::winit::WinitWindows;
use winit::window::Icon;
use image::*;

mod lib;

// use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_editor_pls::{
    controls::{self, EditorControls},
    prelude::*,
};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    // render::RapierDebugRenderPlugin,
};
use lib::{tools::{events, resources::AllSettings}, *};

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");
    // println!("{:?}", std::env::var_os("CARGO_MANIFEST_DIR"));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Подорож з фізикою".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        // .add_plugins(OverlayPlugin {
        //     font_size: 23.0,
        //     // font: Some("internal/fonts/FiraSans-Bold.ttf"),
        //     ..default()
        // })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //
        .add_plugins((
            // EditorPlugin::default(),
            // FrameTimeDiagnosticsPlugin,
            // RapierDebugRenderPlugin::default(),
        ))
        // .insert_resource(editor_controls())
        .insert_resource(AllSettings { 
            volume: 1.0,
            fov: 90.,
        })
        //
        .add_state::<GameState>()
        .add_state::<UiState>()
        .add_state::<PlayerState>()
        //
        .add_event::<events::SpawnPlayer>()
        .add_event::<events::SpawnPlayerCamera>()
        // .add_event::<events::PlacementEvent>()
        .add_event::<events::AttachCollider>()
        .add_event::<events::ModifyCollisionGroup>()
        .add_event::<events::AttachSkybox>()
        .add_event::<events::ProposePopup>()
        .add_event::<events::ButtonState>()
        .add_event::<events::LoadLevel>()
        .add_event::<events::CustomEvent>()
        //
        .add_plugins((
            main_menu::MainMenuPlugin,
            scene_loading::SceneLoaderPlugin,
            camera::GameCameraPlugin,
            ingame_ui::InGameUiPlugin,
            player_control::PlayerPlugin,
            // placing_parts::PlayerPlacingPlugin,
            audio::AudioPlayerPlugin,
            broadcast::ManagerPlugin {},
            hint_overlay::HintOverlayPlugin,
        ))
        //
        .add_systems(Startup, settings)
        .run();
}


fn settings(
    // mut a: ResMut<DefaultOpaqueRendererMethod>,
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/internal/splash/thumbnail.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

// mod todo_post_process;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum PlayerState {
    #[default]
    Absent,
    Interactive,
    Restricted
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum UiState {
    #[default]
    NotSettings,
    PauseSettings,
}

fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(
        controls::Action::PlayPauseEditor,
        controls::Binding {
            input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::P)),
            conditions: vec![controls::BindingCondition::ListeningForText(false)],
        },
    );

    editor_controls
}
