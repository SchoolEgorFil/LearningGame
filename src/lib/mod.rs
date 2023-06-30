use std::time::Duration;

//use bevy::audio::AudioPlugin;
use bevy::pbr::DirectionalLightShadowMap;
use bevy::sprite::Material2dPlugin;
use bevy::{self, prelude::*};
use bevy_kira_audio::prelude::*;
use bevy_rapier3d::prelude::*;

// use self::todo_post_process::{FirstPassMaterial, SecondPassMaterial, ThirdPassMaterial};
use self::main_menu::*;
use self::transition::TransitionMarker;

mod audio;
mod colors;
mod main_menu;
mod player;
mod scene_loader;
mod transition;
// mod todo_post_process;

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    InfoScreen,
}


pub struct GamePlugin {}

impl Plugin for GamePlugin {
    fn name(&self) -> &str {
        "Main Game Plugin"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugins(DefaultPlugins.set(AssetPlugin {
                watch_for_changes: true,
                ..Default::default()
            }))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(AudioPlugin)
            // .add_plugin(RapierDebugRenderPlugin::default())
            // .add_plugin(Material2dPlugin::<FirstPassMaterial>::default())
            // .add_plugin(Material2dPlugin::<SecondPassMaterial>::default())
            // .add_plugin(Material2dPlugin::<ThirdPassMaterial>::default())
            // .add_plugin(shader::CharcoalMaterialPlugin)
            // .add_plugin(MaterialPlugin::<shader::CharcoalMaterial>::default())
            .add_startup_system(
                scene_loader::prepare_rapier
            )
            .add_state::<AppState>()
            .add_system(
                prepare_main_menu
                    .in_schedule(OnEnter(AppState::MainMenu))
            )
            .add_system(
                button_interactivity
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_system(
                destroy_main_menu
                    .in_schedule(OnExit(AppState::MainMenu))
            )
            .add_system(
                scene_loader::load_scene
                    .in_schedule(OnEnter(AppState::InGame))
            )
            .add_system(
                player::prepare_cursor
                    .in_schedule(OnEnter(AppState::InGame))
            )
            .add_system(
                player::gltf_load_player
                    .run_if(in_state(AppState::InGame))
            )
            .add_system(
                player::update_position
                    .run_if(in_state(AppState::InGame))
            )
            .add_system(
                player::gltf_load_colliders
                    .run_if(in_state(AppState::InGame))
            )
            .add_system(
                player::move_camera
                    .run_if(in_state(AppState::InGame))
            )
            .add_system(
                player::move_player
                    .run_if(in_state(AppState::InGame))
            )
            .add_system(
                player::gltf_load_rigidbodies
                    .run_if(in_state(AppState::InGame))
            )
            .add_system(
                player::gltf_load_sun
                    .run_if(in_state(AppState::InGame))
            )
            .add_system(
                player::unlock_cursor
                    .run_if(in_state(AppState::InGame))
            )
            .add_system(
                player::jump
                    .run_if(in_state(AppState::InGame))
            )
            .add_system(
                scene_loader::update_timer
                    .run_if(in_state(AppState::InGame))
            )
            // .add_system(self::todo_post_process::setup.in_schedule(OnEnter(AppState::InGame))) 
            // .add_system(self::todo_post_process::resize.run_if(in_state(AppState::InGame)))
            // .add_system(self::todo_post_process::keyboard_input.run_if(in_state(AppState::InGame)))
            ;
    }
}
