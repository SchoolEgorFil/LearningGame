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
    println!("{:?}", std::env::var_os("CARGO_MANIFEST_DIR"));
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //
        .add_plugin(EditorPlugin::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        //
        .add_state::<AppState>()
        //
        .add_event::<events::SpawnPlayer>()
        .add_event::<events::SpawnPlayerCamera>()
        .add_event::<events::PlacementEvent>()
        //
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(scene_loading::SceneLoaderPlugin)
        .add_plugin(camera::GameCameraPlugin)
        .add_plugin(ingame_ui::InGameUiPlugin)
        .add_plugin(player_control::PlayerPlugin)
        .add_plugin(placing_parts::PlayerPlacingPlugin)
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
