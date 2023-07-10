use bevy::prelude::*;

pub mod lib;

use bevy_editor_pls::prelude::*;
use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use lib::{tools::events, *};

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    App::new()
        // .add_plugin(EditorPlugin::default())
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(AudioPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        // .add_plugin(shader::CharcoalMaterialPlugin)
        // .add_plugin(MaterialPlugin::<shader::CharcoalMaterial>::default())
        // .add_event::<events::PlayerCameraAddEvent>()
        // .add_event::<events::camera::PlayerCameraPrepareEvent>()
        .add_state::<AppState>()
        .add_event::<events::SpawnPlayer>()
        .add_event::<events::SpawnPlayerCamera>()
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(scene_loading::SceneLoaderPlugin)
        .add_plugin(camera::GameCameraPlugin)
        .add_plugin(ingame_ui::InGameUiPlugin)
        .add_plugin(player_control::PlayerPlugin)
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
