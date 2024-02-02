use bevy::{
    prelude::{Input, KeyCode, Query, Res, NextState, ResMut, State},
    window::{CursorGrabMode, Window},
};

use crate::{GameState, PlayerState};

pub fn prepare_cursor(
    // todo move to ui somewhere
    mut windows: Query<&mut Window>,
    // btn: Res<Input<MouseButton>>,
    // key: Res<Input<KeyCode>>
) {
    let mut window = windows.single_mut();

    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

pub fn unlock_cursor(
    mut windows: Query<&mut Window>,
    // btn: Res<Input<MouseButton>>,
    mut next_state: ResMut<NextState<GameState>>,
    key: Res<Input<KeyCode>>,
    state: Res<State<PlayerState>>
) {
    if *state != PlayerState::Interactive {
        return;
    }
    if key.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        window.cursor.grab_mode = CursorGrabMode::None;
        next_state.0 = Some(GameState::MainMenu);
        window.cursor.visible = true;
    }
}
