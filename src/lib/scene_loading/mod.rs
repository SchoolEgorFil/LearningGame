use std::time::Duration;

use super::broadcast::{self, Action, Actor};
use super::tools::collision_groups;
use super::tools::events::{AttachCollider, ModifyCollisionGroup};
use super::tools::markers::ExploredLightObjectMarker;
use super::tools::{
    events::SpawnPlayer, markers::ExploredGLTFObjectMarker, transition::TransitionMarker,
};
use crate::lib::audio::CollisionAudio;
use crate::AppState;
use bevy::gltf::{Gltf, GltfExtras};
use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver};
use bevy::prelude::{
    Children, DirectionalLight, DirectionalLightBundle, EventReader, IntoSystemConfigs, PointLight,
    Resource, Update, Visibility, World,
};
use bevy::utils::HashMap;
use bevy::{
    pbr::DirectionalLightShadowMap,
    prelude::{
        in_state, AssetServer, Assets, Color, Commands, Component, Entity, EventWriter, Handle,
        Mesh, Name, OnEnter, Parent, Plugin, Query, Res, ResMut, Transform, With, Without,
    },
    scene::SceneBundle,
    time::Time,
};
use bevy_rapier3d::prelude::{
    Collider, ColliderMassProperties, CollisionGroups, Group, RapierConfiguration, RapierContext,
    Real, RigidBody, Sensor, Sleeping, TimestepMode, Velocity,
};
use serde_json::Value;

#[derive(Clone, Copy)]
pub enum ColliderType {
    FromMeshTris,
    FromMeshConvexHull,
    FromMeshDecomp,
    Ball,
    Cuboid,
    Cone,
    HeightMap,
    _FromMeshConvexManual,
}

///
/// spawn_point
///     | ambient_intensity: f32
///     | ambient_color: (f32,f32,f32,f32)
///     | skybox: string
///
///
/// mesh_collider_marker
///     | collider_type = "tris" | "hull" | "decomposition" | "ball" | "cuboid" | "cone" | "height_map" | "from_mesh_convex"
///
/// rigidbody: "Dynamic" | "Fixed" | "KPB" | "KVB"
///
/// placable_plane: true
/// placed_mirror: true
///
/// is_visible: true
/// collider_sensor: true
///
/// sun_marker: true
///     | sun_intensity: f32
///     | sun_color: (f32,f32,f32,f32)
///     | sun_shadows: true
///
///
///
/// audio_on_collision: string
///     + collider_sensor
///     + invisisble?
///     + mesh_collider_marker
///         + ...
///
///
///
///
/// action:open_door: bool
/// action:key_door: u16 (key number)
/// action:stand_button: u16 (key number)
/// action:press_button: u16 (key number)
///
///
/// action:* : TODO
///
///  ============== ACTIONS ==========
///
/// ::stand_button = u64 - on press, fires an event with that key number
/// ::stand_button#press = u64 - how long will it be pressed? (0 for toggle button)
/// ::stand_button#cooldown = u64 - how often you can press button
///
/// ::open_door
/// ::open_door#keyed = u64 - if 0, not keyed
/// ::open_door#
///
///
///
///
///
enum CustomProps {
    // todo!() why names of object are included?
    _Unhandled,
    _Resolved,

    PlayerSpawnPoint {
        ambient: Option<(f32, Color)>,
        skybox: Option<String>,
    },
    MeshCollider(ColliderType),
    MeshRigidBody(RigidBody),
    ContructorPlacablePlane,
    PlayerPlacedMirror,
    IsVisible(bool),
    ColliderSensor,
    Sun {
        intensity: f32,
        color: Color,
        shadows: bool,
    },
    MassProp(f32),
    CollisionAudio(String), // todo volume etc...

    Action(Box<dyn Action>),
}

impl CustomProps {
    fn convert(name: &String, value: &Value, main: &serde_json::map::Map<String, Value>) -> Self {
        if name == "ambient_intensity"
            || name == "ambient_color"
            || name == "skybox"
            || name == "collider_type"
            || name == "sun_intensity"
            || name == "sun_color"
            || name == "sun_shadows"
        {
            return CustomProps::_Resolved;
        }
        if name == "spawn_point" && value.as_bool().unwrap_or(false) {
            let int = match main.get("ambient_intensity").and_then(|p| p.as_f64()) {
                Some(v) => Some(v as f32),
                _ => {
                    println!("ambient intensity is not set");
                    None
                }
            };
            let color = match main
                .get("ambient_color")
                .and_then(|p| p.as_array())
                .and_then(|p| {
                    p.get(0..4).and_then(|x| {
                        x.iter()
                            .map(|p| p.as_f64().and_then(|p| Some(p as f32)))
                            .collect::<Option<Vec<f32>>>()
                    })
                }) {
                Some(v) => Some(Color::Rgba {
                    red: v[0],
                    green: v[1],
                    blue: v[2],
                    alpha: v[3],
                }),
                _ => {
                    println!("ambient color is not set");
                    None
                }
            };
            let amb = match (int, color) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            };

            return CustomProps::PlayerSpawnPoint {
                ambient: amb,
                skybox: main
                    .get("skybox")
                    .and_then(|p| p.as_str())
                    .and_then(|p| Some(p.to_string())),
            };
        }
        if name == "mesh_collider_marker" && value.as_bool().unwrap_or(false) {
            let typ = match main.get("collider_type").and_then(|p| p.as_str()) {
                None => panic!(""),

                Some("tris") => ColliderType::FromMeshTris,
                Some("hull") => ColliderType::FromMeshConvexHull,
                Some("decomposition") => ColliderType::FromMeshDecomp,
                Some("from_mesh_convex") => ColliderType::_FromMeshConvexManual,
                Some("ball") => ColliderType::Ball,
                Some("cuboid") => todo!(),
                Some("cone") => todo!(),
                Some("heightmap") => todo!(),

                Some(_) => panic!(""),
            };
            return CustomProps::MeshCollider(typ);
        }
        if name == "rigidbody" && value.is_string() {
            return match value.as_str().unwrap() {
                "Dynamic" => CustomProps::MeshRigidBody(RigidBody::Dynamic),
                "Fixed" => CustomProps::MeshRigidBody(RigidBody::Fixed),
                "KPB" => CustomProps::MeshRigidBody(RigidBody::KinematicPositionBased),
                "KVB" => CustomProps::MeshRigidBody(RigidBody::KinematicVelocityBased),
                _ => {
                    println!("RIGIDBODY MARKER INVALID VALUE");
                    return CustomProps::_Unhandled;
                }
            };
        }
        if name == "placable_plane" && value.as_bool().unwrap_or(false) {
            return CustomProps::ContructorPlacablePlane;
        }
        if name == "placed_mirror" && value.as_bool().unwrap_or(false) {
            return CustomProps::PlayerPlacedMirror;
        }
        if name == "is_visible" {
            return CustomProps::IsVisible(value.as_bool().unwrap());
        }
        if name == "collider_sensor" && value.as_bool().unwrap_or(false) {
            return CustomProps::ColliderSensor;
        }
        if name == "sun_marker" && value.as_bool().unwrap_or(false) {
            let int = match main.get("sun_intensity").and_then(|p| p.as_f64()) {
                Some(v) => v as f32,
                _ => panic!("sun intensity is not set"),
            };
            let color = match main
                .get("sun_color")
                .and_then(|p| p.as_array())
                .and_then(|p| {
                    p.get(0..4).and_then(|x| {
                        x.iter()
                            .map(|p| p.as_f64().and_then(|p| Some(p as f32)))
                            .collect::<Option<Vec<f32>>>()
                    })
                }) {
                Some(v) => v,
                _ => panic!("sun color is not set"),
            };
            let shadows = match main.get("sun_shadows") {
                Some(v) => v.as_bool().unwrap(),
                _ => panic!("sun shadows is not set"),
            };
            return CustomProps::Sun {
                intensity: int,
                color: Color::Rgba {
                    red: color[0],
                    green: color[1],
                    blue: color[2],
                    alpha: color[3],
                },
                shadows: shadows,
            };
        }
        if name == "audio_on_collision" && value.is_string() {
            return CustomProps::CollisionAudio(value.as_str().unwrap().to_string());
        }
        if name == "density" && value.is_f64() {
            return CustomProps::MassProp(value.as_f64().unwrap() as f32);
        }
        // if name == "door_test" && value.as_bool().unwrap_or(false) {
        //     return CustomProps::TESTdoor;
        // }
        let a = name.split(":").collect::<Vec<_>>();
        // dbg!(a.clone());
        // dbg!(a.get(0) == Some(&&"action"), a.get(1).is_some(), a.get(2).is_none()); /////////////////////////////
        if a.get(0) == Some(&&"action") && a.get(1).is_some() && a.get(2).is_none() {
            match a[1] {
                "open_door" => {
                    let mut a = broadcast::open_door::OpenDoorAction::new(value.clone(), &main);
                    a.change_name("open_door".into());
                    return CustomProps::Action(Box::new(a));
                }
                "ball_falling_01" => {
                    let mut a =
                        broadcast::ball_falling_01::BallFalling01Action::new(value.clone(), &main);
                    a.change_name("ball_falling_01".into());
                    return CustomProps::Action(Box::new(a));
                }
                "stand_button" => {
                    let mut a =
                        broadcast::stand_button::StandButtonAction::new(value.clone(), &main);
                    a.change_name("stand_button".into());
                    return CustomProps::Action(Box::new(a));
                }
                _ => {
                    println!("ooohhh unhandled!");
                }
            }
        }
        CustomProps::_Unhandled
    }
}

pub fn gltf_adjust_light(
    mut commands: Commands,
    mut gltf_node_q: Query<(Entity, &mut PointLight), Without<ExploredLightObjectMarker>>,
) {
    for mut e in gltf_node_q.iter_mut() {
        e.1.intensity /= 170.;
        commands.entity(e.0).insert(ExploredLightObjectMarker);
        // e.1.shadows_enabled = true;
    }
}

pub fn gltf_load_extras(
    mut commands: Commands,
    gltf_node_q: Query<
        (
            Entity,
            &GltfExtras,
            &Transform,
            Option<&Children>,
            Option<&Name>,
        ),
        Without<ExploredGLTFObjectMarker>,
    >,
    mesh_bopys: Query<&Handle<Mesh>>,
    mut player_creation_ev_w: EventWriter<SpawnPlayer>,
    mut mesh_collider_ev_w: EventWriter<AttachCollider>,
    mut mesh_collision_group_ev_w: EventWriter<ModifyCollisionGroup>,
    ass: Res<AssetServer>,
) {
    for node in gltf_node_q.iter() {
        commands.entity(node.0).insert(ExploredGLTFObjectMarker);

        let object = serde_json::from_str::<Value>(node.1.value.as_str()).unwrap();
        let object = object.as_object().unwrap();

        for extra in object.iter() {
            match CustomProps::convert(extra.0, extra.1, &object) {
                CustomProps::_Resolved => {}
                CustomProps::PlayerSpawnPoint { ambient, skybox } => {
                    player_creation_ev_w.send(SpawnPlayer {
                        transform: Transform::from_translation(node.2.clone().translation),
                        camera_params: (ambient, skybox),
                    });

                    commands.entity(node.0).despawn();
                }
                CustomProps::MeshCollider(collider) => {
                    if let Some(children) = node.3 {
                        for mesh in children {
                            if mesh_bopys.contains(*mesh) {
                                mesh_collider_ev_w.send(AttachCollider {
                                    collider_type: collider,
                                    entity: mesh.clone(),
                                });
                            }
                        }
                    }
                }
                CustomProps::ColliderSensor => {
                    commands.entity(node.0).insert(Sensor);
                }
                CustomProps::IsVisible(v) => {
                    if !v {
                        commands.entity(node.0).insert((
                            Visibility::Hidden,
                            NotShadowCaster,
                            NotShadowReceiver,
                        ));
                    } else {
                        commands.entity(node.0).insert(Visibility::Visible);
                    }
                    // if let Some(children) = node.3 {
                    //     println!("why? {:#?} \n {:#?}",node.4, node.1);
                    //     for mesh in children {
                    //         commands.entity(*mesh).insert((
                    //             Visibility::Hidden,
                    //             NotShadowCaster,
                    //             NotShadowReceiver,
                    //         ));
                    //     }
                    // }
                }
                CustomProps::MeshRigidBody(rb) => {
                    commands
                        .entity(node.0)
                        .insert((Sleeping::default(), rb, Velocity::default()));
                }
                CustomProps::MassProp(mass) => {
                    commands
                        .entity(node.0)
                        .insert(ColliderMassProperties::Density(mass));
                }
                CustomProps::PlayerPlacedMirror => {
                    mesh_collision_group_ev_w.send(ModifyCollisionGroup {
                        entity: node.0,
                        flags: collision_groups::mirror_system,
                        members: collision_groups::mirror_system,
                        override_groups: false,
                    });
                }
                CustomProps::ContructorPlacablePlane => {
                    mesh_collision_group_ev_w.send(ModifyCollisionGroup {
                        entity: node.0,
                        flags: collision_groups::mirror_system,
                        members: collision_groups::mirror_system,
                        override_groups: false,
                    });
                }
                CustomProps::Sun {
                    color,
                    intensity,
                    shadows,
                } => {
                    commands.insert_resource(DirectionalLightShadowMap { size: 4096 });

                    commands.entity(node.0).insert(DirectionalLightBundle {
                        directional_light: DirectionalLight {
                            shadows_enabled: shadows,
                            illuminance: intensity,
                            color,
                            ..Default::default()
                        },
                        cascade_shadow_config: CascadeShadowConfigBuilder {
                            num_cascades: 4,
                            maximum_distance: 126.,
                            ..Default::default()
                        }
                        .into(),
                        ..Default::default()
                    });
                }
                CustomProps::CollisionAudio(audio) => {
                    commands
                        .entity(node.0)
                        .insert(CollisionAudio::from_handle(ass.load(audio)));
                    // commands.entity(node.0).insert(bundle)
                }
                CustomProps::Action(action) => {
                    commands.entity(node.0).add(|id, world: &mut World| {
                        if let Some(mut actor) = world.get_mut::<Actor>(id) {
                            if let Some(_) = actor.0.insert(action.name(), action) {
                                panic!();
                            }
                        } else {
                            let mut h = HashMap::new();
                            if let Some(_) = h.insert(action.name(), action) {
                                panic!();
                            }
                            world.entity_mut(id).insert(Actor(h));
                        }
                    });
                    // commands.entity(node.0).insert(Actor(Box::new(OpenDoorAction::default())));
                }
                CustomProps::_Unhandled => {
                    println!("UNHANDLED {:#?}: {:#?}", extra.0, extra.1);
                }
            }
        }
    }
}

// struct CollisionAudio {
//     asset: Handle<Audio>
// }

pub fn attach_collider(
    mut commands: Commands,
    mut attach_collider_ev_r: EventReader<AttachCollider>,
    mesh_q: Query<(&Parent, &Handle<Mesh>)>,
    collision_group_q: Query<(Option<&CollisionGroups>, &Transform)>,
    mesh_ass: Res<Assets<Mesh>>,
) {
    for event in attach_collider_ev_r.iter() {
        if let Ok(mesh) = mesh_q.get(event.entity) {
            let c = match event.collider_type {
                ColliderType::FromMeshTris => Collider::from_bevy_mesh(
                    mesh_ass.get(mesh.1).unwrap(),
                    &bevy_rapier3d::prelude::ComputedColliderShape::TriMesh,
                )
                .unwrap(),
                ColliderType::FromMeshConvexHull => Collider::from_bevy_mesh(
                    mesh_ass.get(mesh.1).unwrap(),
                    &bevy_rapier3d::prelude::ComputedColliderShape::ConvexHull,
                )
                .unwrap(),
                ColliderType::FromMeshDecomp => Collider::from_bevy_mesh(
                    mesh_ass.get(mesh.1).unwrap(),
                    &bevy_rapier3d::prelude::ComputedColliderShape::ConvexDecomposition(
                        bevy_rapier3d::prelude::VHACDParameters {
                            ..Default::default()
                        },
                    ),
                )
                .unwrap(), // todo!()
                ColliderType::Ball => {
                    // println!("{:?}", collision_group_q.get(mesh.0.get()).unwrap().1.scale);
                    // Collider::ball(collision_group_q.get(mesh.0.get()).unwrap().1.scale.x)
                    Collider::ball(1.0)
                    // Collider::from_bevy_mesh(
                    //     mesh_ass.get(mesh.1).unwrap(),
                    //     &bevy_rapier3d::prelude::ComputedColliderShape::ConvexHull,
                    // )
                    // .unwrap()
                }
                _ => todo!(),
            };
            unsafe {
                if collision_group_q.get(mesh.0.get()).unwrap().0.is_none() {
                    commands.entity(mesh.0.get()).insert(CollisionGroups::new(
                        Group::from_bits_unchecked(collision_groups::player_collision),
                        Group::from_bits_unchecked(collision_groups::player_collision),
                    ));
                }
                commands.entity(mesh.0.get()).insert(c);
            }
        }
    }
}

pub fn attach_collision_groups(
    mut commands: Commands,
    qu: Query<Option<&CollisionGroups>, With<Collider>>,
    mut ev_r: EventReader<ModifyCollisionGroup>,
) {
    for ev in ev_r.iter() {
        let group: (u32, u32) = 'out: {
            let Ok(k) = qu.get(ev.entity) else {
                break 'out (
                    collision_groups::empty,
                    collision_groups::empty,
                );
            };
            let Some(e) = k else {
                break 'out (
                    collision_groups::empty,
                    collision_groups::empty,
                );
            };
            (e.filters.bits(), e.memberships.bits())
        };
        if ev.override_groups {
            unsafe {
                commands.entity(ev.entity).insert(CollisionGroups {
                    memberships: Group::from_bits_unchecked(ev.members),
                    filters: Group::from_bits_unchecked(ev.flags),
                });
            }
        } else {
            unsafe {
                commands.entity(ev.entity).insert(CollisionGroups {
                    memberships: Group::from_bits_unchecked(group.0 | ev.members),
                    filters: Group::from_bits_unchecked(group.1 | ev.flags),
                });
            }
        }
    }
}

#[derive(Component)]
pub struct LoaderMarker;

pub fn prepare_rapier(mut r_ctx: ResMut<RapierContext>) {
    r_ctx.integration_parameters.max_ccd_substeps = 5;
    r_ctx.integration_parameters.max_stabilization_iterations = 3;
}

#[derive(Resource)]
pub struct SceneTempRes(Handle<Gltf>);

pub fn load_scene(mut commands: Commands, asset: Res<AssetServer>) {
    commands.spawn((
        LoaderMarker,
        TransitionMarker::new(false, Duration::from_millis(400)),
        Name::new("The thing I put just in case TM"),
    ));

    let glb = asset.load("tutorial_level.glb");

    commands.insert_resource(SceneTempRes(glb.clone()));
}

pub fn spawn_gltf(mut commands: Commands, gltf: Option<Res<SceneTempRes>>, ass: Res<Assets<Gltf>>) {
    if let Some(gltf) = gltf {
        if let Some(gltf) = ass.get(&gltf.0) {
            commands.spawn((
                SceneBundle {
                    scene: gltf.scenes[0].clone(),

                    ..Default::default()
                },
                Name::new("Main level scene"),
            ));
            commands.remove_resource::<SceneTempRes>();
        }
    }
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
        app
            // .add_systems(Startup, prepare_rapier)
            .add_systems(OnEnter(AppState::InGame), (load_scene).chain())
            .add_systems(
                Update,
                (
                    spawn_gltf,
                    update_timer,
                    // gltf_load_colliders,
                    prepare_rapier,
                    (gltf_load_extras, (attach_collider, attach_collision_groups)).chain(),
                    gltf_adjust_light,
                )
                    .distributive_run_if(in_state(AppState::InGame)),
            );
    }
}
