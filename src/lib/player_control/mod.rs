use bevy::prelude::{in_state, IntoSystemConfigs, Plugin, Update, OnEnter, OnExit};

use crate::{GameState, UiState};

use self::systems::{add_player, move_camera, move_player, queue_player_jump, tackle_jump, unrestrict_player, restrict_player};

pub mod components;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn name(&self) -> &str {
        "Plugin for player control"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::Game), unrestrict_player)
        .add_systems(OnExit(GameState::Game), restrict_player)
        .add_systems(
            Update,
            (
                add_player,
                move_player,
                move_camera,
                (queue_player_jump, tackle_jump).chain(),
            )
                .distributive_run_if(in_state(GameState::Game))
                .distributive_run_if(in_state(UiState::NotSettings)),
        );
    }
}
