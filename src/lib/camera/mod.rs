use bevy::{
    prelude::{in_state, IntoSystemConfig, Plugin},
    sprite::Material2dPlugin,
};

use crate::AppState;

use self::{
    materials::{FirstPassMaterial, SecondPassMaterial, ThirdPassMaterial},
    setup::setup,
};

mod materials;
// mod resize;
mod components;
mod setup;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn name(&self) -> &str {
        "Plugin for every camera this game ever needs"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(Material2dPlugin::<FirstPassMaterial>::default())
            .add_plugin(Material2dPlugin::<SecondPassMaterial>::default())
            .add_plugin(Material2dPlugin::<ThirdPassMaterial>::default())
            .add_system(setup.run_if(in_state(AppState::InGame)));
    }
}
