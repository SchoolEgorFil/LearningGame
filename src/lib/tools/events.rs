use bevy::prelude::Transform;

use crate::lib::placing_parts::{PlacingForm, PlacingObject};

pub struct SpawnPlayer {
    pub transform: Transform,
}

pub struct SpawnPlayerCamera;

#[derive(Debug)]
pub struct PlacementEvent {
    pub object: PlacingObject,
    pub form: PlacingForm,
}
