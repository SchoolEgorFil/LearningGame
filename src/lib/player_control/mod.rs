use bevy::prelude::{in_state, IntoSystemConfig, IntoSystemConfigs, Plugin};

use crate::AppState;

use self::systems::{add_player, move_camera, move_player, queue_player_jump, tackle_jump};

pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Plugin for player control"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(add_player)
            .add_system(move_player.run_if(in_state(AppState::InGame)))
            .add_systems(
                (queue_player_jump, tackle_jump)
                    .chain()
                    .distributive_run_if(in_state(AppState::InGame)),
            )
            .add_system(move_camera.run_if(in_state(AppState::InGame)));
    }
}
