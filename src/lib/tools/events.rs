use bevy::prelude::{Transform, Event, Entity, Handle, Image};

use crate::lib::placing_parts::{PlacingForm, PlacingObject};

#[derive(Event)]
pub struct SpawnPlayer {
    pub transform: Transform,
}

#[derive(Event)]
pub struct SpawnPlayerCamera;

#[derive(Debug, Event)]
pub struct PlacementEvent {
    pub object: PlacingObject,
    pub form: PlacingForm,
}


// GLTF
pub enum ColliderType {
    FromMeshTris,
    FromMeshConvexHull,
    FromMeshDecomp,
    Ball,
    Cuboid,
    Cone,
    HeightMap,
    _FromMeshConvexManual
}

#[derive(Event)]
pub struct AttachCollider {
    pub entity: Entity,
    pub collider_type: ColliderType
}

#[derive(Event)]
pub struct ModifyCollisionGroup {
    pub entity: Entity,
    pub members: u32,
    pub flags: u32
}

#[derive(Event, Clone)]
pub struct AttachSkybox {
    pub image: Handle<Image>
}