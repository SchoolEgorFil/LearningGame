use bevy::prelude::{in_state, IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnExit, Plugin};

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
        app.add_system(prepare_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(button_interactivity.run_if(in_state(AppState::MainMenu)))
            .add_system(destroy_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
