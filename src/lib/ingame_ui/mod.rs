pub mod systems;

use bevy::prelude::{in_state, IntoSystemAppConfig, IntoSystemConfig, OnEnter, Plugin};

use crate::AppState;

use self::systems::{prepare_cursor, unlock_cursor};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn name(&self) -> &str {
        "Plugin for handling in-game ui"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(prepare_cursor.in_schedule(OnEnter(AppState::InGame)))
            .add_system(unlock_cursor.run_if(in_state(AppState::InGame)));
    }
}
