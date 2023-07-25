use std::time::Duration;

use bevy::core_pipeline::Skybox;
use bevy::gltf::GltfExtras;
use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver};
use bevy::prelude::{
    Children, DespawnRecursiveExt, DirectionalLight, DirectionalLightBundle, TransformBundle, Startup, IntoSystemConfigs, Update, Local, World, EventReader, Visibility, PostUpdate,
};
use bevy::render::view::NoFrustumCulling;
use bevy::{
    pbr::DirectionalLightShadowMap,
    prelude::{
        in_state, AmbientLight, AssetServer, Assets, Color, Commands, Component, Entity,
        EventWriter, Handle, Mesh, Name, OnEnter, Parent,
        Plugin, Query, Res, ResMut, Transform, With, Without,
    },
    scene::SceneBundle,
    time::Time,
};
use bevy_rapier3d::prelude::{
    ActiveCollisionTypes, Ccd, Collider, CollisionGroups, Group, RapierConfiguration,
    RapierContext, RigidBody, TimestepMode, Sensor,
};
use serde_json::Value;
use crate::AppState;
use super::tools::events::{AttachCollider, ModifyCollisionGroup, AttachSkybox};
use super::tools::markers::PlayerMainCamera;
use super::tools::{
    events::SpawnPlayer, markers::ExploredGLTFObjectMarker, transition::TransitionMarker,
};

///
/// spawn_point: true,
/// mesh_collider_marker: true      <-------------------------- TODO
/// rigidbody_marker: "Dynamic" | "Fixed" | "KPB" | "KVB"
/// 
/// placable_plane_marker: true
/// placed_mirror_marker: true
/// 
/// invinsible: true
/// collider_sensor: true
/// 
/// skybox: string
/// 
enum CustomProps {
    _Unhandled,

    PlayerSpawnPoint,
    MeshTrisCollider,
    MeshRigidBody(RigidBody),

    ContructorPlacablePlane,
    PlayerPlacedMirror,

    Invinsible,
    ColliderSensor,

    Sun,
    Skybox(String),
}

impl CustomProps {
    fn convert(name: &String, value: &Value) -> Self {
        if name == "spawn_point" && value.as_bool().unwrap_or(false) {
            return CustomProps::PlayerSpawnPoint;
        }
        if name == "mesh_collider_marker" && value.as_bool().unwrap_or(false) {
            return CustomProps::MeshTrisCollider;
        }
        if name == "rigidbody_marker" && value.is_string() {
            return match value.as_str().unwrap() {
                "Dynamic" => CustomProps::MeshRigidBody(RigidBody::Dynamic),
                "Fixed" => CustomProps::MeshRigidBody(RigidBody::Fixed),
                "KPB" => CustomProps::MeshRigidBody(RigidBody::KinematicPositionBased),
                "KVB" => CustomProps::MeshRigidBody(RigidBody::KinematicVelocityBased),
                _ => {println!("RIGIDBODY MARKER INVALID VALUE"); return CustomProps::_Unhandled;}
            };
        }
        if name == "placable_plane_marker" && value.as_bool().unwrap_or(false) {
            return CustomProps::ContructorPlacablePlane;
        }
        if name == "placed_mirror_marker" && value.as_bool().unwrap_or(false) {
            return CustomProps::PlayerPlacedMirror;
        }
        if name == "invisible" && value.as_bool().unwrap_or(false) { 
            return  CustomProps::Invinsible;
        }
        if name == "sensor" && value.as_bool().unwrap_or(false) { 
            return  CustomProps::ColliderSensor;
        }
        if name == "sun" && value.as_bool().unwrap_or(false) { 
            return  CustomProps::Sun;
        }
        if name == "skybox" && value.is_string() {
            return CustomProps::Skybox(value.as_str().unwrap().to_string());
        }
        CustomProps::_Unhandled
    }
}

pub fn gltf_load_extras(
    mut commands: Commands,
    gltf_node_q: Query<(Entity, &GltfExtras, &Transform, Option<&Children>), Without<ExploredGLTFObjectMarker>>,
    mut player_creation_ev_w: EventWriter<SpawnPlayer>,
    mut mesh_collider_ev_w: EventWriter<AttachCollider>,
    mut mesh_collision_group_ev_w: EventWriter<ModifyCollisionGroup>,
    mut skybox_ev_w: EventWriter<AttachSkybox>,
    ass: Res<AssetServer>
) {
    for node in gltf_node_q.iter() {
        commands.entity(node.0).insert(ExploredGLTFObjectMarker);

        let object = serde_json::from_str::<Value>(node.1.value.as_str()).unwrap();
        let object = object.as_object().unwrap();

        for extra in object.iter() {
            match CustomProps::convert(extra.0, extra.1) {
                CustomProps::PlayerSpawnPoint => {
                    player_creation_ev_w.send(SpawnPlayer {
                        transform: Transform::from_translation(node.2.clone().translation),
                    });
        
                    commands.entity(node.0).despawn();
                },
                CustomProps::MeshTrisCollider => {
                    if let Some(children) = node.3 {
                        for mesh in children {
                            mesh_collider_ev_w.send(AttachCollider {
                                collider_type: super::tools::events::ColliderType::FromMeshTris,
                                entity: mesh.clone()
                            });
                        }
                    }
                },
                CustomProps::ColliderSensor => {
                    commands.entity(node.0).insert(Sensor);
                },
                CustomProps::Invinsible => {
                    if let Some(children) = node.3 {
                        for mesh in children {
                            commands.entity(*mesh).insert((Visibility::Hidden,NotShadowCaster,NotShadowReceiver));
                        }
                    }
                },
                CustomProps::MeshRigidBody(rb) => {
                    commands.entity(node.0).insert(rb);
                },
                CustomProps::PlayerPlacedMirror => {
                    mesh_collision_group_ev_w.send(ModifyCollisionGroup {
                        entity: node.0,
                        flags:   0b01000000_00000000_00000000_00000001,
                        members: 0b01000000_00000000_00000000_00000001,
                    });
                },
                CustomProps::ContructorPlacablePlane => {
                    mesh_collision_group_ev_w.send(ModifyCollisionGroup {
                        entity: node.0,
                        flags:   0b10000000_00000000_00000000_00000001,
                        members: 0b10000000_00000000_00000000_00000001,
                    });
                },
                CustomProps::Sun => {
                    commands.entity(node.0).insert(DirectionalLightBundle {
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
                            num_cascades: 4,
                            maximum_distance: 126.,
                            ..Default::default()
                        }
                        .into(),
                        ..Default::default()
                    });
                },
                CustomProps::Skybox(str) => {
                    skybox_ev_w.send(AttachSkybox { image: ass.load(str) });
                }
                CustomProps::_Unhandled => {
                    println!("UNHANDLED {:#?}",extra.1);
                }
            }
        }
        
    }

}

pub fn attach_collider(
    mut commands: Commands,
    mut attach_collider_ev_r: EventReader<AttachCollider>,
    mesh_q: Query<(&Parent,&Handle<Mesh>)>,
    mesh_ass: Res<Assets<Mesh>>
) {
    for event in attach_collider_ev_r.iter() {
        if let Ok(mesh) = mesh_q.get(event.entity) {
            let c = 
                match event.collider_type {
                    super::tools::events::ColliderType::FromMeshTris => 
                         Collider::from_bevy_mesh(mesh_ass.get(mesh.1).unwrap(), &bevy_rapier3d::prelude::ComputedColliderShape::TriMesh).unwrap(),
                    _ => panic!()       
                };
            unsafe {
                commands.entity(mesh.0.get())
                    .insert((
                        c, 
                        CollisionGroups::new(
                            Group::from_bits_unchecked(0b00000000_00000000_00000000_00000001), 
                            Group::from_bits_unchecked(0b00000000_00000000_00000000_00000001)
                        )));
            }
        }
    }
}

#[derive(Default)]
pub struct LocalEventSkybox(pub Option<AttachSkybox>);

pub fn attach_skybox(
    mut commands: Commands,
    query: Query<Entity, With<PlayerMainCamera>>,
    mut wait: Local<LocalEventSkybox>,
    mut ev_r: EventReader<AttachSkybox>
) {
    for ev in ev_r.iter() {
        wait.0 = Some(ev.clone());
    }
    if wait.0.is_some() {
    if let Ok(en) = query.get_single() {
        if let Some(sk) = &wait.0 {
            commands.entity(en).insert(Skybox(
                sk.image.clone()
            ));
            wait.0 = None;
        }
    }
    }
}

pub fn attach_collision_groups(
    mut commands: Commands,
    qu: Query<Option<&CollisionGroups>,With<Collider>>,
    mut ev_r: EventReader<ModifyCollisionGroup>
) {
    for ev in ev_r.iter() {
        let group: (u32,u32) = 'out: {
            let Ok(k) = qu.get(ev.entity) else {
                break 'out (
                    0b00000000_00000000_00000000_00000001,
                    0b00000000_00000000_00000000_00000001,
                );
            };
            let Some(e) = k else {
                break 'out (
                    0b00000000_00000000_00000000_00000001,
                    0b00000000_00000000_00000000_00000001,
                );
            };
            (
                e.filters.bits(),
                e.memberships.bits(),
            )
        };
        unsafe {
            commands.entity(ev.entity).insert(CollisionGroups {
                memberships: Group::from_bits_unchecked(
                    group.0 | ev.members
                ),
                filters: Group::from_bits_unchecked(
                    group.1 | ev.flags
                )
            });
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
        app.add_systems(Startup,prepare_rapier)
            .add_systems(OnEnter(AppState::InGame),load_scene)
            .add_systems(Update,
                (
                    update_timer,
                    // gltf_load_colliders,
                    (gltf_load_extras,
                    (attach_collider,
                    attach_skybox,
                    attach_collision_groups)).chain(),
                ).distributive_run_if(in_state(AppState::InGame))
            )
            
            ;
    }
}