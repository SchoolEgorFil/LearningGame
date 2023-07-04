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

use super::scene_loader::Inserted;

#[derive(Component)]
pub struct PlayerMeshMarker;

#[derive(Component)]
pub struct PlayerCameraMarker;

#[derive(Bundle)]
pub struct PlayerMeshBundle {
    pub marker: PlayerMeshMarker,
    pub rb: RigidBody,
    pub locked_axes: LockedAxes,
    // pub transform: TransformBundle,
    pub collider: Collider,
    pub character_control: KinematicCharacterController,
    pub collision_type: ActiveCollisionTypes,
    pub jump_impulse: ExternalImpulse,
    pub ccd: Ccd,
    pub damping: Damping,
}

#[derive(Bundle)]
pub struct PlayerCameraBundle {
    pub marker: PlayerCameraMarker,
    pub camera: Camera3dBundle,
    pub environment_map_light: EnvironmentMapLight,
}

pub fn gltf_load_player(
    mut commands: Commands,
    gltf_obj: Query<(Entity, &Name, &Transform), Without<Inserted>>,
    asset_server: Res<AssetServer>,
) {
    for m in gltf_obj.iter() {
        if m.1.as_str().contains(TAGS::PLAYER_SPAWN) {
            let skybox_handle = asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2");

            commands
                .spawn(super::player::PlayerMeshBundle {
                    marker: PlayerMeshMarker,
                    rb: RigidBody::Dynamic,
                    locked_axes: LockedAxes::ROTATION_LOCKED,
                    collider: Collider::capsule_y(0.8, 0.5),
                    character_control: KinematicCharacterController {
                        apply_impulse_to_dynamic_bodies: true,
                        autostep: Some(CharacterAutostep {
                            max_height: CharacterLength::Absolute(0.2),
                            min_width: CharacterLength::Absolute(0.01),
                            include_dynamic_bodies: false,
                        }),
                        ..Default::default()
                    },

                    collision_type: ActiveCollisionTypes::default()
                        | ActiveCollisionTypes::KINEMATIC_STATIC,
                    jump_impulse: ExternalImpulse {
                        impulse: Vec3::ZERO,
                        torque_impulse: Vec3::ZERO,
                    },
                    // ccd: Ccd::disabled(),
                    ccd: Ccd::enabled(),
                    damping: Damping {
                        linear_damping: 0.2,
                        angular_damping: 1.0,
                    },
                })
                .insert(TransformBundle::from_transform(
                    Transform::from_translation(m.2.clone().translation),
                ))
                .insert(Sleeping::disabled())
                .with_children(|mut p| {
                    p.spawn(PlayerCameraBundle {
                        marker: PlayerCameraMarker,
                        camera: Camera3dBundle {
                            transform: Transform::from_translation(Vec3::new(0.0, 0.8, 0.0)),
                            projection: Projection::Perspective(PerspectiveProjection {
                                fov: 110. / 180. * std::f32::consts::PI,
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        environment_map_light: EnvironmentMapLight {
                            diffuse_map: skybox_handle.clone(),
                            specular_map: asset_server
                                .load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
                        },
                    });
                });
            commands.entity(m.0).despawn();
        }
    }
}

pub fn gltf_load_colliders(
    mut commands: Commands,
    gltf_m: Query<(Entity, &Parent, &Handle<Mesh>)>,
    gltf_obj: Query<(Entity, &Name), Without<Inserted>>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    for m in gltf_m.iter() {
        let Ok(object) = gltf_obj.get(m.1.get()) else {
            continue;
        };

        if object.1.as_str().contains(TAGS::COLLIDER_MESH) {
            let Some(mesh) = mesh_assets.get(m.2) else {
                continue;
            };

            let Some(mesh_collider) = Collider::from_bevy_mesh(mesh, &bevy_rapier3d::prelude::ComputedColliderShape::TriMesh) else {
                panic!();
            };

            /// obviusly, TODO
            ///
            // let mesh_collider = Collider::cuboid(1.0, 1.0, 1.0);
            commands.entity(m.0).insert((
                RigidBody::Fixed,
                mesh_collider,
                ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
                Ccd::enabled(),
            ));
            commands.entity(object.0).insert(Inserted);
        }
    }
}

pub fn gltf_load_rigidbodies(
    mut commands: Commands,
    gltf_m: Query<(Entity, &Parent, &Handle<Mesh>)>,
    gltf_obj: Query<(Entity, &Name), Without<Inserted>>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    for m in gltf_m.iter() {
        let Ok(object) = gltf_obj.get(m.1.get()) else {
            continue;
        };

        if object.1.as_str().contains(TAGS::RIGIDBODY) {
            let Some(mesh) = mesh_assets.get(m.2) else {
                continue;
            };

            let mesh_collider = if object.1.as_str().contains(TAGS::BALL_RIGIDBODY) {
                // Collider::ball(radius)
                Collider::from_bevy_mesh(
                    mesh,
                    &bevy_rapier3d::prelude::ComputedColliderShape::TriMesh,
                )
                .unwrap()
            } else {
                Collider::from_bevy_mesh(
                    mesh,
                    &bevy_rapier3d::prelude::ComputedColliderShape::TriMesh,
                )
                .unwrap()
            };

            /// obviusly, TODO
            ///
            // let mesh_collider = Collider::cuboid(1.0, 1.0, 1.0);
            commands.entity(m.0).insert((
                RigidBody::Dynamic,
                mesh_collider,
                ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
                ColliderMassProperties::Density(0.4),
                Damping {
                    linear_damping: 0.6,
                    angular_damping: 2.,
                },
                // Ccd::enabled()
            ));
            commands.entity(object.0).insert(Inserted);
        }
    }
}

pub fn update_position(mut controllers: Query<&mut KinematicCharacterController>) {
    for mut controller in controllers.iter_mut() {
        controller.translation = Some(Vec3::new(0.0, -0.0001, 0.0));
    }
}




pub fn jump(
    mut controllers: Query<(
        &mut ExternalImpulse,
        &mut KinematicCharacterControllerOutput,
    )>,
    mut player_camera_transform_q: Query<&Transform, With<PlayerCameraMarker>>,
    key: Res<Input<KeyCode>>,
) {
    // println!("{}",controllers.iter().count());
    let Ok(mut c) = controllers.get_single_mut() else {return;};
    // let Some(out) = c.1 else {return;};

    // println!("{}",c.1.grounded);

    if key.pressed(KeyCode::Space) && c.1.grounded {
        c.0.impulse += Vec3::new(0.0, 6., 0.0);
    }
}

pub mod TAGS {
    pub const PLAYER_SPAWN: &'static str = "ACTION_PLAYER_SPAWN";
    pub const COLLIDER_MESH: &'static str = "COLLIDER";
    pub const RIGIDBODY: &'static str = "RB";
    pub const BALL_RIGIDBODY: &'static str = "BALL_RB";
    pub const SUN: &'static str = "DIR_SUN";
    pub const SKYBOX: &'static str = "SKYBOX";
}

pub fn gltf_load_sun(
    mut commands: Commands,
    gltf_obj: Query<(Entity, &Name, &Transform, Option<&Children>), Without<Inserted>>,
    gltf_m: Query<Entity, (With<Parent>, With<Handle<Mesh>>)>,
    asset_server: Res<AssetServer>,
) {
    for m in gltf_obj.iter() {
        if m.1.as_str().contains(TAGS::SUN) {
            commands
                .spawn(DirectionalLightBundle {
                    directional_light: DirectionalLight {
                        shadows_enabled: true,
                        illuminance: 32000.0,
                        ..Default::default()
                    },
                    // This is a relatively small scene, so use tighter shadow
                    // cascade bounds than the default for better quality.
                    // We also adjusted the shadow map to be larger since we're
                    // only using a single cascade.
                    cascade_shadow_config: CascadeShadowConfigBuilder {
                        num_cascades: 2,
                        maximum_distance: 126.,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .insert(TransformBundle::from_transform(Transform::from_rotation(
                    m.2.clone().rotation,
                )));
            commands.entity(m.0).despawn();
        } else if m.1.as_str().contains(TAGS::SKYBOX) && m.3.is_some() {
            m.3.unwrap().iter().for_each(|e| {
                if gltf_m.contains(e.clone()) {
                    commands.entity(e.clone()).insert(NotShadowCaster);
                }
            });
        }
    }
}
