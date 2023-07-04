use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerParentMarker;

#[derive(Component)]
pub struct PlayerCameraChildMarker;

#[derive(Component)]
pub struct PlayerCollisionChildMarker;

#[derive(Component)]
pub struct AddingObjectUiMarker;

#[derive(Component)]
pub struct ExploredGLTFObjectMarker;
