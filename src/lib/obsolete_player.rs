use bevy::{
    input::mouse::MouseMotion,
    pbr::{CascadeShadowConfigBuilder, NotShadowCaster},
    prelude::{
        AssetServer, Assets, BuildChildren, Bundle, Camera3d, Camera3dBundle, Children, Commands,
        Component, DirectionalLight, DirectionalLightBundle, Entity, EnvironmentMapLight,
        EventReader, Handle, Input, KeyCode, Mesh, MouseButton, Name, Parent,
        PerspectiveProjection, Projection, Query, Res, ResMut, Transform, Vec3, With, Without,
    },
    transform::TransformBundle,
    window::{CursorGrabMode, Window},
};
use bevy_rapier3d::prelude::{
    ActiveCollisionTypes, Ccd, CharacterAutostep, CharacterLength, Collider,
    ColliderMassProperties, Damping, ExternalImpulse, GravityScale, KinematicCharacterController,
    KinematicCharacterControllerOutput, LockedAxes, RigidBody, Sleeping,
};

use super::markers::{ExploredGLTFObjectMarker, PlayerParentMarker};

pub fn gltf_load_player(
    mut commands: Commands,
    gltf_obj: Query<(Entity, &Name, &Transform), Without<ExploredGLTFObjectMarker>>,
    player_query: Query<(Entity, &PlayerParentMarker)>,
    asset_server: Res<AssetServer>,
) {
    if player_query.is_empty() {
        return;
    }
    for m in gltf_obj.iter() {
        if m.1.as_str().contains(TAGS::PLAYER_SPAWN) {
            let Ok(player) = player_query.get_single() else {
                continue;
            };
            println!("PLAYER LOADING");
            commands
                .entity(player.0)
                .insert(TransformBundle::from_transform(
                    Transform::from_translation(m.2.clone().translation),
                ));

            commands.entity(m.0).despawn();
        }
    }
}

pub fn gltf_load_colliders(
    mut commands: Commands,
    gltf_mesh_query: Query<(Entity, &Parent, &Handle<Mesh>), Without<ExploredGLTFObjectMarker>>,
    gltf_object_query: Query<
        (Entity, &Name, Option<&Parent>, Option<&Transform>),
        Without<ExploredGLTFObjectMarker>,
    >,
    mesh_assets: Res<Assets<Mesh>>,
) {
    for mesh in gltf_mesh_query.iter() {
        let Ok(object) = gltf_object_query.get(mesh.1.get()) else {
            continue;
        };

        let Some(mesh_data) = mesh_assets.get(mesh.2) else {
            continue;
        };

        let collider = match object.1.as_str() {
            x if x.contains(TAGS::GENERIC_COLLIDER) => {
                let compute_shape = if object.1.as_str().contains(TAGS::MODIFIER_CONVEX_COLLIDER) {
                    bevy_rapier3d::prelude::ComputedColliderShape::ConvexDecomposition(
                        bevy_rapier3d::prelude::VHACDParameters::default(),
                    )
                } else {
                    bevy_rapier3d::prelude::ComputedColliderShape::TriMesh
                };
                Collider::from_bevy_mesh(mesh_data, &compute_shape).unwrap()
            }
            x if x.contains(TAGS::SPHERE_COLLIDER) => {
                let Some(transform) = object.3 else { panic!() };

                Collider::ball(transform.scale.x)
            }
            _ => {
                continue;
            }
        };

        let rb = if object.1.as_str().contains(TAGS::MODIFIER_RIGIDBODY) {
            RigidBody::Dynamic
        } else {
            RigidBody::Fixed
        };

        let bundle = (
            rb,
            collider,
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
            Ccd::enabled(),
        );

        match object.2 {
            Some(parent_id) if object.1.as_str().contains(TAGS::MODIFIER_PARENT) => {
                commands.entity(parent_id.get()).insert(bundle);
                commands.entity(mesh.0).despawn();
            }
            _ => {
                commands.entity(object.0).insert(bundle);
                commands.entity(mesh.0).insert(ExploredGLTFObjectMarker);
                // commands.entity(mesh.0).despawn();
            }
        }
    }
}

pub mod TAGS {
    pub const PLAYER_SPAWN: &'static str = "PLAYER_SPAWNPOINT";
    pub const GENERIC_COLLIDER: &'static str = "TRI_C";
    pub const SPHERE_COLLIDER: &'static str = "SPHERE_C";
    pub const MODIFIER_CONVEX_COLLIDER: &'static str = "TRI_C";
    pub const MODIFIER_RIGIDBODY: &'static str = "RB";
    // pub const RIGIDBODY: &'static str = "RB";
    // pub const BALL_RIGIDBODY: &'static str = "BALL_RB";
    pub const SUN: &'static str = "DIR_SUN";
    pub const SKYBOX: &'static str = "SKYBOX";
    pub const MODIFIER_PARENT: &'static str = "PARENT";
    // pub const SPHERICAL_RIGIDBODY: &'static str = "SPHERE_RB";
    // pub const GENERIC_RIGIDBODY: &'static str = "TRI_RB";
}

// pub fn gltf_load_sun(
//     mut commands: Commands,
//     gltf_obj: Query<(Entity, &Name, &Transform, Option<&Children>), Without<Inserted>>,
//     gltf_m: Query<Entity, (With<Parent>, With<Handle<Mesh>>)>,
//     asset_server: Res<AssetServer>,
// ) {
//     for m in gltf_obj.iter() {
//         if m.1.as_str().contains(TAGS::SUN) {
//             commands
//                 .spawn(DirectionalLightBundle {
//                     directional_light: DirectionalLight {
//                         shadows_enabled: true,
//                         illuminance: 32000.0,
//                         ..Default::default()
//                     },
//                     // This is a relatively small scene, so use tighter shadow
//                     // cascade bounds than the default for better quality.
//                     // We also adjusted the shadow map to be larger since we're
//                     // only using a single cascade.
//                     cascade_shadow_config: CascadeShadowConfigBuilder {
//                         num_cascades: 2,
//                         maximum_distance: 126.,
//                         ..Default::default()
//                     }
//                     .into(),
//                     ..Default::default()
//                 })
//                 .insert(TransformBundle::from_transform(Transform::from_rotation(
//                     m.2.clone().rotation,
//                 )));
//             commands.entity(m.0).despawn();
//         } else if m.1.as_str().contains(TAGS::SKYBOX) && m.3.is_some() {
//             m.3.unwrap().iter().for_each(|e| {
//                 if gltf_m.contains(e.clone()) {
//                     commands.entity(e.clone()).insert(NotShadowCaster);
//                 }
//             });
//         }
//     }
// }
