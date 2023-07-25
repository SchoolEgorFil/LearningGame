use bevy::{
    prelude::{in_state, Plugin, Update, IntoSystemConfigs},
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
        app.add_plugins((
            Material2dPlugin::<FirstPassMaterial>::default(),
            Material2dPlugin::<SecondPassMaterial>::default(),
            Material2dPlugin::<ThirdPassMaterial>::default()
        ))
            .add_systems(Update, setup.run_if(in_state(AppState::InGame)));
    }
}
