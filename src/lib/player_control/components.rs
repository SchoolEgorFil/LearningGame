use bevy::prelude::Name;
use bevy::{
    prelude::{Bundle, Component, SpatialBundle},
    utils::Instant,
};
use bevy_rapier3d::prelude::{
    ActiveCollisionTypes, Ccd, Collider, CollisionGroups, Damping, ExternalImpulse,
    KinematicCharacterController, LockedAxes, RigidBody, Velocity,
};

use crate::lib::tools::markers;

#[derive(Component)]
pub enum CameraPlayerMode {
    Body,
    Freecam,
    _Controlled,
}

#[derive(Component)]
pub enum CreationMode {
    Adventure,
    Creative,
}

#[derive(Component)]
pub struct JumpableCharacter {
    // todo move?
    // for those who wish to jump
    // acceleration: f32,
    pub jump_buffer: Option<Instant>, // if you don't want to jump, its value is None, but if you do, set it to Time of creation
                                      // impulse: ExternalImpulse,
}

impl JumpableCharacter {
    pub fn new() -> JumpableCharacter {
        JumpableCharacter { jump_buffer: None }
    }
    pub fn queue_jump(&mut self) {
        self.jump_buffer = Some(Instant::now());
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: markers::PlayerParentMarker,

    pub sp: SpatialBundle,
    pub rb: RigidBody,
    pub locked_axes: LockedAxes,
    // collider: Collider,
    pub character_control: KinematicCharacterController,
    pub collision_type: ActiveCollisionTypes,
    pub jump: JumpableCharacter,
    pub impulse: ExternalImpulse,
    pub ccd: Ccd,
    pub damping: Damping,
    pub collider: Collider,
    pub velocity: Velocity,
    pub collision_group: CollisionGroups,
    pub name: Name,
}

#[derive(Bundle)]
pub struct PlayerCameraContainerBundle {
    pub marker: markers::PlayerCameraContainerMarker,

    pub sp: SpatialBundle,
    pub name: Name,
}
