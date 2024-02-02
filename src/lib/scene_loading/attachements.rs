


use super::super::tools::collision_groups;
use super::super::tools::events::{AttachCollider, ModifyCollisionGroup,};
use super::super::tools::markers::ExploredLightObjectMarker;

use super::super::tools::{
     transition::TransitionMarker,
};
use super::components::GltfFileMarker;


use bevy::prelude::{
     EventReader, PointLight,
};

use bevy::{
    
    prelude::{
        Assets, Commands, Entity,  Handle,
        Mesh,   Parent,  Query, Res, ResMut, Transform, With, Without,
    },
    time::Time,
};
use bevy_rapier3d::prelude::{
    Collider,  CollisionGroups, Group, RapierConfiguration, RapierContext,
      TimestepMode, 
};


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

pub fn attach_collider(
    mut commands: Commands,
    mut attach_collider_ev_r: EventReader<AttachCollider>,
    mesh_q: Query<(&Parent, &Handle<Mesh>)>,
    collision_group_q: Query<(Option<&CollisionGroups>, &Transform)>,
    mesh_ass: Res<Assets<Mesh>>,
) {
    for event in attach_collider_ev_r.read() {
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
            
                if collision_group_q.get(mesh.0.get()).unwrap().0.is_none() {
                    commands.entity(mesh.0.get()).insert(CollisionGroups::new(
                        Group::from_bits_truncate(collision_groups::player_collision),
                        Group::from_bits_truncate(collision_groups::player_collision),
                    ));
                }
                commands.entity(mesh.0.get()).insert(c);
            
        }
    }
}

pub fn attach_collision_groups(
    mut commands: Commands,
    qu: Query<Option<&CollisionGroups>, With<Collider>>,
    mut ev_r: EventReader<ModifyCollisionGroup>,
) {
    for ev in ev_r.read() {
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
            
                commands.entity(ev.entity).insert(CollisionGroups {
                    memberships: Group::from_bits_truncate(ev.members),
                    filters: Group::from_bits_truncate(ev.flags),
                });
            
        } else {
            
                commands.entity(ev.entity).insert(CollisionGroups {
                    memberships: Group::from_bits_truncate(group.0 | ev.members),
                    filters: Group::from_bits_truncate(group.1 | ev.flags),
                });
            
        }
    }
}

pub fn gltf_adjust_light(
    mut commands: Commands,
    mut gltf_node_q: Query<(Entity, &mut PointLight), Without<ExploredLightObjectMarker>>,
) {
    // for mut e in gltf_node_q.iter_mut() {
    //     e.1.intensity /= 720.;
    //     commands.entity(e.0).insert(ExploredLightObjectMarker);
    //     // e.1.shadows_enabled = true;
    //     // e.1.range = 200.;
    // }
}

pub fn prepare_rapier(mut r_ctx: ResMut<RapierContext>) {
    r_ctx.integration_parameters.max_ccd_substeps = 5;
    r_ctx.integration_parameters.max_stabilization_iterations = 3;
}

pub fn update_timer(
    mut t: Query<&mut TransitionMarker, With<GltfFileMarker>>,
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