use std::time::Duration;

use bevy::{prelude::{Res, Commands, AssetServer, Handle, Resource, Assets, AmbientLight, Color, Camera3dBundle, Transform, Vec3, PbrBundle, ResMut, Mesh, StandardMaterial, shape, Visibility, ComputedVisibility, GlobalTransform, MaterialMeshBundle, Entity, Name, Without, Query, Component, Camera3d, Camera, With, DirectionalLightBundle, DirectionalLight}, gltf::Gltf, scene::SceneBundle, render::{mesh::VertexAttributeValues, view::RenderLayers}, core_pipeline::clear_color::ClearColorConfig, transform::TransformBundle, time::Time, pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap}};
use bevy_rapier3d::{prelude::{RigidBody, Collider, Restitution, RapierContext, RapierConfiguration, TimestepMode}, rapier::prelude::IntegrationParameters};

use super::transition::TransitionMarker;


#[derive(Component)]
pub struct LoaderMarker;


pub fn prepare_rapier(mut r_ctx: ResMut<RapierContext>) {
    // r_ctx.integration_parameters.max_ccd_substeps = 3;
    // r_ctx.integration_parameters.max_stabilization_iterations = 2;
}

pub fn load_scene(mut commands: Commands, asset: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>) {

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 10.0/5.0f32,
    });
    commands.insert_resource(DirectionalLightShadowMap { size: 4096 });

    // commands.spawn(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         ..Default::default()
    //     },
    //     // This is a relatively small scene, so use tighter shadow
    //     // cascade bounds than the default for better quality.
    //     // We also adjusted the shadow map to be larger since we're
    //     // only using a single cascade.
    //     cascade_shadow_config: CascadeShadowConfigBuilder {
    //         num_cascades: 1,
    //         maximum_distance: 66.,
    //         ..Default::default()
    //     }
    //     .into(),
    //     ..Default::default()
    // });

    commands.spawn((
        LoaderMarker,
        super::transition::TransitionMarker::new(false,Duration::from_millis(400))
    ));

    let glb = asset.load("untitled.glb#Scene0");

    commands.spawn(SceneBundle {
        scene: glb,

        ..Default::default()
    });



    // commands
    // .spawn(RigidBody::Dynamic)
    // .insert(Collider::ball(0.5))
    // .insert(Restitution::coefficient(0.7))
    // .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

#[derive(Component)]
pub struct Inserted;

pub fn update_timer(
    mut t: Query<&mut TransitionMarker, With<LoaderMarker>>,
    mut config: ResMut<RapierConfiguration>,
    time: Res<Time>
) {
    let mut t = t.single_mut();
    if !t.started {
        t.started = true;
        config.timestep_mode = TimestepMode::Fixed { dt: 0.000001, substeps: 1 };
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