use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::GameState;

use self::ui::{button_interactivity,level_interactivity, destroy_main_menu, prepare_main_menu, fix_images};
use self::load_settings::load_settings;

pub mod components;
pub mod ui;
pub mod load_settings;
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn name(&self) -> &str {
        "For handling main menu"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::MainMenu), (load_settings,prepare_main_menu).chain())
            .add_systems(
                Update,
                (fix_images, button_interactivity,level_interactivity).distributive_run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), destroy_main_menu);
    }
}
