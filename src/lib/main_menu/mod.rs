use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::AppState;

use self::ui::{button_interactivity, destroy_main_menu, prepare_main_menu};

pub mod components;
pub mod ui;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn name(&self) -> &str {
        "For handling main menu"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::MainMenu), prepare_main_menu)
            .add_systems(
                Update,
                button_interactivity.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), destroy_main_menu);
    }
}
