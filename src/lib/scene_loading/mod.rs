use self::attachements::{update_timer, prepare_rapier, attach_collider, attach_collision_groups, gltf_adjust_light};
use self::changing_cusom_properties::gltf_load_extras;
use self::gltf_handling::{load_gltf_file, spawn_loaded_gltf_scene};
use self::unload::unload;
use super::broadcast;
use crate::GameState;
use bevy::prelude::{ IntoSystemConfigs, Update, OnExit, Commands, TextBundle, OnEnter };
use bevy::text::TextStyle;
use bevy::transform::commands;
use bevy::{
    prelude::{
        in_state, Plugin,
    },
};

pub mod attachements;
pub mod changing_cusom_properties;
pub mod components;
pub mod custom_properties;
pub mod gltf_handling;
pub mod unload;

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn name(&self) -> &str {
        "For loading in-game scenes"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .add_systems(Startup, prepare_rapier)
            .add_systems(OnEnter(GameState::Game), load_some_scene)
            .add_systems(
                Update,
                (
                    load_gltf_file, // spawns GltfFileMarker
                    spawn_loaded_gltf_scene, // spawns MainSceneMarker
                )
                    .distributive_run_if(in_state(GameState::MainMenu)),
            ).add_systems(
                Update,
                (
                    update_timer,
                    // gltf_load_colliders,
                    prepare_rapier,
                    (gltf_load_extras, (attach_collider, attach_collision_groups)).chain(),
                    gltf_adjust_light,
                )
                    .distributive_run_if(in_state(GameState::Game)),
            ).add_systems(
                OnExit(GameState::Game), 
                unload
            );
    }
}


fn load_some_scene(mut commands: Commands) {
    // commands.spawn(TextBundle::from_section("asdhjlashjsahjklads", TextStyle { font_size: 36., ..Default::default()}));
}