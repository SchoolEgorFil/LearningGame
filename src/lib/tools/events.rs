use std::{sync::Arc, ffi::OsString};

use bevy::{
    prelude::{Color, Entity, Event, Handle, Image, KeyCode, Transform},
    text::TextStyle,
};

use crate::lib::{
    // placing_parts::{PlacingForm, PlacingObject},
    scene_loading::attachements::ColliderType,
};

#[derive(Event)]
pub struct SpawnPlayer {
    pub transform: Transform,
    pub camera_params: (Option<(f32, Color)>, Option<String>, Option<String>,  Option<String>),
}

#[derive(Event)]
pub struct SpawnPlayerCamera {
    pub camera_params: (Option<(f32, Color)>, Option<String>, Option<String>,  Option<String>),
}

// #[derive(Debug, Event)]
// pub struct PlacementEvent {
//     pub object: PlacingObject,
//     pub form: PlacingForm,
// }

#[derive(Event)]
pub struct AttachCollider {
    pub entity: Entity,
    pub collider_type: ColliderType,
}

#[derive(Event)]
pub struct ModifyCollisionGroup {
    pub entity: Entity,
    pub members: u32,
    pub flags: u32,
    pub override_groups: bool,
}

#[derive(Event, Clone)]
pub struct AttachSkybox {
    pub image: Handle<Image>,
}

#[derive(Event, Clone)]
pub struct ProposePopup {
    pub key: Option<KeyCode>,
    pub text: Arc<String>,
    pub style: TextStyle,
    pub priority: u32,
}

#[derive(Event, Clone)]
pub struct ButtonState {
    pub is_pressed: bool,
    pub just_changed: bool,
    pub id: u64,
}


#[derive(Event)]
pub struct LoadLevel {
    pub string: OsString
}

#[derive(Event)]
pub struct CustomEvent {
    pub name: String,
    pub json_encoded: String
}