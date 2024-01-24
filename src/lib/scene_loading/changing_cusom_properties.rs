use super::broadcast::Actor;
use super::super::tools::collision_groups;
use super::super::tools::events::{AttachCollider, ModifyCollisionGroup};
use super::super::tools::{
    events::SpawnPlayer, markers::ExploredGLTFObjectMarker,
};
use super::custom_properties::CustomProps;
use crate::lib::audio::CollisionAudio;
use bevy::gltf::GltfExtras;
use bevy::pbr::{CascadeShadowConfigBuilder, NotShadowCaster, NotShadowReceiver};
use bevy::prelude::{
    
    Children, DirectionalLight, DirectionalLightBundle,
     Visibility, EntityWorldMut, PointLight, SpotLight,
};
use bevy::utils::HashMap;
use bevy::{
    pbr::DirectionalLightShadowMap,
    prelude::{
        AssetServer,Commands, Entity, EventWriter, Handle,
        Mesh, Name,  Query, Res, Transform,  Without,
    },
};
use bevy_rapier3d::prelude::{
     ColliderMassProperties,
    Sensor, Sleeping, Velocity,
};
use serde_json::Value;



pub fn gltf_load_extras(
    mut commands: Commands,
    mut gltf_node_q: Query<
        (
            Entity,
            &GltfExtras,
            &Transform,
            Option<&Children>,
            Option<&Name>,
            Option<&mut DirectionalLight>,
            Option<&mut PointLight>,
            Option<&mut SpotLight>
        ),
        Without<ExploredGLTFObjectMarker>,
    >,
    mesh_bopys: Query<&Handle<Mesh>>,
    mut player_creation_ev_w: EventWriter<SpawnPlayer>,
    mut mesh_collider_ev_w: EventWriter<AttachCollider>,
    mut mesh_collision_group_ev_w: EventWriter<ModifyCollisionGroup>,
    ass: Res<AssetServer>,
) {
    for mut node in gltf_node_q.iter_mut() {
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
                CustomProps::Light {
                    // color,
                    // intensity,
                    shadows,
                } => {
                    commands.insert_resource(DirectionalLightShadowMap { size: 4096 });
                    println!("{}", shadows);
                    if let Some(ref mut a) = node.5 {
                        a.shadows_enabled = shadows;
                    }
                    if let Some(ref mut a) = node.6 {
                        a.shadows_enabled = shadows;
                    }
                    if let Some(ref mut a) = node.7 {
                        a.shadows_enabled = shadows;
                    }

                    // commands.entity(node.0).insert(DirectionalLightBundle {
                    //     directional_light: DirectionalLight {
                    //         shadows_enabled: shadows,
                    //         illuminance: intensity,
                    //         color,
                    //         ..Default::default()
                    //     },
                    //     cascade_shadow_config: CascadeShadowConfigBuilder {
                    //         num_cascades: 4,
                    //         maximum_distance: 126.,
                    //         ..Default::default()
                    //     }
                    //     .into(),
                    //     ..Default::default()
                    // });
                }
                CustomProps::CollisionAudio(audio) => {
                    commands
                        .entity(node.0)
                        .insert(CollisionAudio::from_handle(ass.load(audio)));
                    // commands.entity(node.0).insert(bundle)
                }
                CustomProps::Action(action) => {
                    commands.entity(node.0).add(|mut entity: EntityWorldMut| {
                        
                        if let Some(mut actor) = entity.get_mut::<Actor>() {
                            if let Some(_) = actor.0.insert(action.name(), action) {
                                panic!();
                            }
                        } else {
                            let mut h = HashMap::new();
                            if let Some(_) = h.insert(action.name(), action) {
                                panic!();
                            }
                            entity.insert(Actor(h));
                        }
                    });
                    // commands.entity(node.0).insert(Actor(Box::new(OpenDoorAction::default())));
                }
                CustomProps::_Unhandled => {
                    // println!("UNHANDLED {:#?}: {:#?}", extra.0, extra.1);
                }
            }
        }
    }
}
