use bevy::{
    prelude::{Input, KeyCode, Query, Res},
    window::{CursorGrabMode, Window},
};

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
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
