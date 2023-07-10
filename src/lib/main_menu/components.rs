use bevy::prelude::{Component, Resource};

use crate::lib::tools::transition::TransitionMarker;

#[derive(Component)]
pub struct MainMenuMarker;

#[derive(Resource)]
pub struct MainMenuResource {
    pub to_game: TransitionMarker,
    pub to_settings: TransitionMarker,
}

pub enum MainMenuButtonEnum {
    start_game,
    settings,
    exit,
}

#[derive(Component)]
pub struct MainMenuButtonMarker(pub MainMenuButtonEnum);

#[derive(Component)]
pub struct RootNode;
