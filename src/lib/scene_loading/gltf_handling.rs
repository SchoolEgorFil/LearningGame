use std::path::PathBuf;
use std::time::Duration;

use super::super::tools::events::LoadLevel;

use super::super::tools::resources::LoadingSceneInfo;
use super::super::tools::{
     transition::TransitionMarker,
};
use super::components::MainSceneMarker;

use crate::GameState;
use crate::lib::scene_loading::components::GltfFileMarker;
use bevy::gltf::Gltf;
use bevy::prelude::{ State, NextState, EventReader };
use bevy::{
    prelude::{
        AssetServer, Assets,  Commands, 
        Name, Res, ResMut, 
    },
    scene::SceneBundle,
    
};

pub fn load_gltf_file(
    mut commands: Commands, 
    asset: Res<AssetServer>,
    mut ev: EventReader<LoadLevel>
) {
    if ev.len() > 1 {
        panic!("You should not load 2 gltfs at the same tick... or in general");
    }

    for i in  ev.read() {
        let mut s = PathBuf::new();
        s = s.join("levels");
        s = s.join(i.string.clone());
        s = s.join("main.gltf");
        println!("Loading {}",s.display());
        let glb = asset.load(s);
        commands.insert_resource(LoadingSceneInfo {
            handle: glb.clone(),
            is_loaded: false,
            name: i.string.clone().to_string_lossy().into_owned()
        });
        commands.spawn((
            GltfFileMarker,
            TransitionMarker::new(false, Duration::from_millis(400)),
            Name::new("The thing I put just in case TM"),
        ));
    }

}

pub fn spawn_loaded_gltf_scene(
    mut commands: Commands,
    loading_scene: Option<ResMut<LoadingSceneInfo>>,
    gltf_asset_manager: Res<Assets<Gltf>>,
    _state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(mut scene_info) = loading_scene {
        if scene_info.is_loaded {
            return;
        }
        if let Some(gltf_asset) = gltf_asset_manager.get(&scene_info.handle) {
            // for (k,v) in gltf.named_animations.iter() {
            //     println!("Action: {}", k);
            // }
            commands.spawn((
                MainSceneMarker,
                SceneBundle {
                    scene: gltf_asset.scenes[0].clone(),
                    ..Default::default()
                },
                Name::new("Main level scene"),
            ));
            scene_info.is_loaded = true;
            // commands.remove_resource::<SceneTempRes>();

            next_state.0 = Some(GameState::Game);
            // gltf.named_animations
        }
    }
}
