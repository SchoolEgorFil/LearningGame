use std::time::Duration;

use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver};
use bevy::prelude::{
    Children, DespawnRecursiveExt, DirectionalLight, DirectionalLightBundle, TransformBundle,
};
use bevy::render::view::NoFrustumCulling;
use bevy::{
    pbr::DirectionalLightShadowMap,
    prelude::{
        in_state, AmbientLight, AssetServer, Assets, Color, Commands, Component, Entity,
        EventWriter, Handle, IntoSystemAppConfig, IntoSystemConfig, Mesh, Name, OnEnter, Parent,
        Plugin, Query, Res, ResMut, Transform, With, Without,
    },
    scene::SceneBundle,
    time::Time,
};

use bevy_rapier3d::prelude::{
    ActiveCollisionTypes, Ccd, Collider, CollisionGroups, Group, RapierConfiguration,
    RapierContext, RigidBody, TimestepMode,
};

use crate::AppState;

use super::tools::{
    events::SpawnPlayer, markers::ExploredGLTFObjectMarker, transition::TransitionMarker,
};

pub fn gltf_load_player(
    mut commands: Commands,
    gltf_obj: Query<(Entity, &Name, &Transform), Without<ExploredGLTFObjectMarker>>,
    asset_server: Res<AssetServer>,
    mut player_creation_ev_w: EventWriter<SpawnPlayer>,
) {
    for m in gltf_obj.iter() {
        if m.1.as_str().contains(TAGS::PLAYER_SPAWN) {
            player_creation_ev_w.send(SpawnPlayer {
                transform: Transform::from_translation(m.2.clone().translation),
            });

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
                let compute_shape = if object.1.as_str().contains(TAGS::MODIFIER_DECOMP_COLLIDER) {
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

        let collision_groups = if object.1.as_str().contains(TAGS::MODIFIER_PLACABLE) {
            unsafe {
                CollisionGroups::new(
                    Group::from_bits_unchecked(0b10000000_00000000_00000000_00000001),
                    Group::from_bits_unchecked(0b10000000_00000000_00000000_00000001),
                )
            }
        } else if object.1.as_str().contains(TAGS::MODIFIER_MIRROR) {
            unsafe {
                CollisionGroups::new(
                    Group::from_bits_unchecked(0b01000000_00000000_00000000_00000001),
                    Group::from_bits_unchecked(0b01000000_00000000_00000000_00000001),
                )
            }
        } else {
            unsafe {
                CollisionGroups::new(
                    Group::from_bits_unchecked(0b00000000_00000000_00000000_00000001),
                    Group::from_bits_unchecked(0b00000000_00000000_00000000_00000001),
                )
            }
        };

        let bundle = (
            rb,
            collider,
            ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC,
            Ccd::enabled(),
            collision_groups,
        );

        match object.2 {
            Some(parent_id) if object.1.as_str().contains(TAGS::MODIFIER_PARENT) => {
                commands.entity(parent_id.get()).insert(bundle);
                commands.entity(object.0).despawn_recursive();
            }
            _ => {
                commands.entity(object.0).insert(bundle);
                commands.entity(mesh.0).insert(ExploredGLTFObjectMarker);
                if object.1.as_str().contains(TAGS::MODIFIER_INVISIBLE) {
                    commands.entity(object.0).insert((
                        NoFrustumCulling,
                        NotShadowCaster,
                        NotShadowReceiver,
                    ));
                    commands.entity(mesh.0).despawn();
                }
            }
        }
    }
}

pub mod TAGS {
    pub const PLAYER_SPAWN: &str = "PLAYER_SPAWNPOINT";
    pub const GENERIC_COLLIDER: &str = "TRI_C";
    pub const SPHERE_COLLIDER: &str = "SPHERE_C";

    pub const SUN: &str = "DIR_SUN";
    pub const SKYBOX: &str = "SKYBOX";

    pub const MODIFIER_DECOMP_COLLIDER: &str = "TRI_C_DECOMP";
    pub const MODIFIER_INVISIBLE: &str = "INV";
    pub const MODIFIER_RIGIDBODY: &str = "RB";
    pub const MODIFIER_PARENT: &str = "PARENT";
    pub const MODIFIER_PLACABLE: &str = "PLACABLE";

    pub const MODIFIER_MIRROR: &str = "MIRROR";
}

#[derive(Component)]
pub struct LoaderMarker;

pub fn prepare_rapier(mut r_ctx: ResMut<RapierContext>) {
    // r_ctx.integration_parameters.max_ccd_substeps = 3;
    // r_ctx.integration_parameters.max_stabilization_iterations = 2;
}

pub fn load_scene(
    mut commands: Commands,
    asset: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 10.0 / 5.0f32,
    });
    commands.insert_resource(DirectionalLightShadowMap { size: 4096 });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 2,
            maximum_distance: 66.,
            ..Default::default()
        }
        .into(),
        ..Default::default()
    });

    commands.spawn((
        LoaderMarker,
        TransitionMarker::new(false, Duration::from_millis(400)),
        Name::new("The thing I put just in case TM"),
    ));

    let glb = asset.load("survival.glb#Scene0");

    commands.spawn((
        SceneBundle {
            scene: glb,

            ..Default::default()
        },
        Name::new("Main level scene"),
    ));
}

pub fn update_timer(
    mut t: Query<&mut TransitionMarker, With<LoaderMarker>>,
    mut config: ResMut<RapierConfiguration>,
    time: Res<Time>,
) {
    let mut t = t.single_mut();
    if !t.started {
        t.started = true;
        config.timestep_mode = TimestepMode::Fixed {
            dt: 0.000001,
            substeps: 1,
        };
    } else {
        t.tick(time.delta());
        if t.timer.just_finished() {
            config.timestep_mode = TimestepMode::Variable {
                max_dt: 1.0 / 60.0,
                time_scale: 1.0,
                substeps: 1,
            };
        }
    }
}

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn name(&self) -> &str {
        "For loading in-game scenes"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(prepare_rapier)
            .add_system(load_scene.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_timer.run_if(in_state(AppState::InGame)))
            .add_system(gltf_load_player.run_if(in_state(AppState::InGame)))
            .add_system(gltf_load_colliders.run_if(in_state(AppState::InGame)))
            .add_system(gltf_load_sun.run_if(in_state(AppState::InGame)));
    }
}

pub fn gltf_load_sun(
    mut commands: Commands,
    gltf_obj: Query<
        (Entity, &Name, &Transform, Option<&Children>),
        Without<ExploredGLTFObjectMarker>,
    >,
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
                    commands
                        .entity(e.clone())
                        .insert((NotShadowCaster, NotShadowReceiver));
                }
            });
        }
    }
}
