use std::time::Duration;

use bevy::{prelude::*, gltf::GltfPlugin};

pub mod lib;

use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use lib::{tools::events, *};

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");
    println!("{:?}", std::env::var_os("CARGO_MANIFEST_DIR"));
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: Some(bevy::asset::ChangeWatcher { delay: Duration::from_millis(200) }),
            ..Default::default()
        }))
        .add_plugins(OverlayPlugin {
            font_size: 23.0,
            // font: Some("fonts/FiraSans-Bold.ttf"),

            ..default()
        })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
//
        .add_plugins((
            EditorPlugin::default(),
            RapierDebugRenderPlugin::default()
        ))
//
        .add_state::<AppState>()
//
        .add_event::<events::SpawnPlayer>()
        .add_event::<events::SpawnPlayerCamera>()
        .add_event::<events::PlacementEvent>()
        .add_event::<events::AttachCollider>()
        .add_event::<events::ModifyCollisionGroup>()
        .add_event::<events::AttachSkybox>()
//
        .add_plugins((
            main_menu::MainMenuPlugin, 
            scene_loading::SceneLoaderPlugin,
            camera::GameCameraPlugin,
            ingame_ui::InGameUiPlugin,
            player_control::PlayerPlugin,
            placing_parts::PlayerPlacingPlugin
        ))
//
        .run();
}

// mod todo_post_process;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    InfoScreen,
}
