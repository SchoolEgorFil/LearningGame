use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::GameState;

use self::ui::{button_interactivity,level_interactivity, destroy_main_menu, prepare_main_menu};

pub mod components;
pub mod ui;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn name(&self) -> &str {
        "For handling main menu"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::MainMenu), prepare_main_menu)
            .add_systems(
                Update,
                (button_interactivity,level_interactivity).distributive_run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), destroy_main_menu);
    }
}
