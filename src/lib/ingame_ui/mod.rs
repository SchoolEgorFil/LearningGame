pub mod systems;

use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, Plugin, Update};

use crate::AppState;

use self::systems::{prepare_cursor, unlock_cursor};

pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
    fn name(&self) -> &str {
        "Plugin for handling in-game ui"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::InGame), prepare_cursor)
            .add_systems(Update, unlock_cursor.run_if(in_state(AppState::InGame)));
    }
}
