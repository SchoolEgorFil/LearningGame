use bevy::{prelude::{Resource, Entity, Handle}, gltf::Gltf};
use serde::{Serialize, Deserialize};

use crate::lib::main_menu::components::MainMenuVariants;

use super::transition::TransitionMarker;

#[derive(Resource, Serialize, Deserialize)]
pub struct AllSettings {
    pub volume: f64,
    pub fov: f32,
}

#[derive(Resource)]
pub struct MainMenuResource {
    pub current_position: MainMenuVariants,
    pub next_position: MainMenuVariants,
    pub transition_proccess: TransitionMarker,
}


#[derive(Resource)]
pub struct PlayerResource {
    pub player_entity: Entity,
}


#[derive(Resource)]
pub struct LoadingSceneInfo {
    pub name: String,
    pub handle: Handle<Gltf>, 
    pub is_loaded: bool
}