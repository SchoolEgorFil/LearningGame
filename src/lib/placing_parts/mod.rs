use bevy::asset::Handle;
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::AlphaMode;
use bevy::prelude::{Image, Name};
use bevy::{
    prelude::{
        in_state,
        shape::{Plane, Quad},
        AssetServer, Assets, Bundle, Color, Commands, Component, EventReader, EventWriter,
        GlobalTransform, Input, IntoSystemAppConfig, IntoSystemConfig, IntoSystemConfigs, KeyCode,
        Mesh, MouseButton, OnEnter, Parent, Plugin, Query, Res, ResMut, Resource, SpatialBundle,
        StandardMaterial, Transform, Vec2, Visibility, With, Without,
    },
    scene::SceneBundle,
    transform::commands,
};
use bevy_rapier3d::prelude::{Group, QueryFilter, RapierContext};

use crate::AppState;

use super::{
    player_control,
    tools::{events::PlacementEvent, markers::PlayerCameraContainerMarker},
};

pub struct PlayerPlacingPlugin;

impl Plugin for PlayerPlacingPlugin {
    fn name(&self) -> &str {
        "For them to place things they want"
    }
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(PlayerPlacingResource {
            choosing_stage: PlacingObjectChoosingStage::NotChoosing,
            placing_object: None,
            placement: None,
        })
        .add_system(setup.in_schedule(OnEnter(AppState::InGame)))
        .add_system(initiate_placement.run_if(in_state(AppState::InGame)))
        .add_systems(
            (place_object, preview_placement_grid)
                .chain()
                .distributive_run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Resource, Debug)]
pub struct PlayerPlacingResource {
    pub choosing_stage: PlacingObjectChoosingStage,
    pub placing_object: Option<PlacingObject>,
    pub placement: Option<Transform>,
}

#[derive(Debug)]
pub enum PlacingObjectChoosingStage {
    NotChoosing,
    ChooseCarousel,
    ChoseAndPlacing,
}

#[derive(Clone, Copy, Debug)]
pub enum PlacingObject {
    LaserPointer,
    LaserMirror,
}

#[derive(Debug)]
pub enum PlacingForm {
    Grid,
    Snap,
    _Free,
}

pub fn initiate_placement(
    mut res: ResMut<PlayerPlacingResource>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut placement_ev_w: EventWriter<PlacementEvent>,
) {
    match res.choosing_stage {
        PlacingObjectChoosingStage::NotChoosing => {
            if keys.any_pressed([KeyCode::LShift, KeyCode::RShift]) && keys.just_pressed(KeyCode::A)
            {
                res.choosing_stage = PlacingObjectChoosingStage::ChooseCarousel;
            }
        }
        PlacingObjectChoosingStage::ChooseCarousel => {
            if keys.just_pressed(KeyCode::Key1) {
                res.placing_object = Some(PlacingObject::LaserMirror);
                res.choosing_stage = PlacingObjectChoosingStage::ChoseAndPlacing;
            } else if keys.just_pressed(KeyCode::Key2) {
                res.placing_object = Some(PlacingObject::LaserPointer);
                res.choosing_stage = PlacingObjectChoosingStage::ChoseAndPlacing;
            }
        }
        PlacingObjectChoosingStage::ChoseAndPlacing => {
            if mouse.just_pressed(MouseButton::Left) {
                placement_ev_w.send(PlacementEvent {
                    object: res.placing_object.unwrap().clone(),
                    form: PlacingForm::Grid,
                });
                res.choosing_stage = PlacingObjectChoosingStage::NotChoosing;
            }
        }
        _ => {}
    }
}

#[derive(Component)]
pub struct PreviewGridMarker;

pub fn setup(
    mut commands: Commands,
    mut mesh_server: ResMut<Assets<Mesh>>,
    mut std_mat_res: ResMut<Assets<StandardMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    let mesh = mesh_server.add(Plane::from_size(16.0).into());
    let image: Handle<Image> = asset_server.load("textures/placement_grid.png");
    let emissive: Handle<Image> = asset_server.load("textures/placement_grid_emissive.png");

    let material = std_mat_res.add(StandardMaterial {
        base_color_texture: Some(image.clone()),
        alpha_mode: AlphaMode::Blend,
        emissive_texture: Some(emissive.clone()),
        ..Default::default()
    });

    let mut sp = SpatialBundle::default();
    // sp.transform.rotate_x(-std::f32::consts::FRAC_PI_2);
    sp.visibility = Visibility::Hidden;
    commands.spawn((
        mesh,
        material,
        sp,
        PreviewGridMarker,
        Name::new("Grid placement plane"),
    ));
}

pub fn preview_placement_grid(
    mut mesh_q: Query<(&mut Visibility, &mut Transform), With<PreviewGridMarker>>,
    mut placing_res: ResMut<PlayerPlacingResource>,
    rapier_context: Res<RapierContext>,
    camera_q: Query<
        (&GlobalTransform, &Parent),
        (
            With<PlayerCameraContainerMarker>,
            Without<PreviewGridMarker>,
        ),
    >,
) {
    if mesh_q.is_empty() {
        return;
    }
    let mut grid = mesh_q.single_mut();
    match placing_res.choosing_stage {
        PlacingObjectChoosingStage::ChoseAndPlacing => {
            if camera_q.is_empty() {
                return;
            }
            let camera = camera_q.single();
            let r = rapier_context.cast_ray(
                camera.0.translation(),
                camera.0.forward(),
                30.,
                true,
                QueryFilter::new().groups(bevy_rapier3d::prelude::CollisionGroups::new(
                    Group::all(),
                    Group::GROUP_32,
                )),
            );
            if let Some((_, distance)) = r {
                *grid.0 = Visibility::Visible;
                grid.1.translation = camera.0.translation() + camera.0.forward() * distance * 0.96;

                // todo add normals,

                grid.1.translation.x = (grid.1.translation.x / 2.).round() * 2.;
                grid.1.translation.z = (grid.1.translation.z / 2.).round() * 2.;

                let rotation_vec = {
                    let mut a = camera.0.forward();
                    // let n = Vec3::new(a.x,0.0,a.z).try_normalize()?;
                    let angle = (a.x / a.z).abs().atan();
                    let norm = (angle / std::f32::consts::FRAC_PI_2 * 6.).round() / 6.
                        * std::f32::consts::FRAC_PI_2;
                    let x = std::f32::consts::FRAC_1_SQRT_2 * norm.sin() * a.x.signum();
                    let z = std::f32::consts::FRAC_1_SQRT_2 * norm.cos() * a.z.signum();
                    Vec3::new(x, 0.0, z).try_normalize()
                };
                let placement = {
                    let mut a = grid.1.clone();

                    if rotation_vec.is_some() {
                        a.rotation = Quat::from_rotation_arc(
                            Vec3::new(1.0, 0.0, 0.0),
                            rotation_vec.unwrap(),
                        );
                    }
                    a
                };

                placing_res.placement = Some(placement);
            } else {
                *grid.0 = Visibility::Hidden;
                placing_res.placement = None;
            }
        }
        _ => {
            *grid.0 = Visibility::Hidden;
            placing_res.placement = None;
        }
    };
}

fn place_object(
    mut commands: Commands,
    mut placement_ev_r: EventReader<PlacementEvent>,
    asset_server: Res<AssetServer>,
    placing_res: Res<PlayerPlacingResource>,
) {
    // println!("{:?} | ", placing_res,);
    if placing_res.placement.is_none() {
        return;
    }

    for placement_event in placement_ev_r.iter() {
        match placement_event.object {
            PlacingObject::LaserMirror => {
                let m = asset_server.load("components/laser_mirror.glb#Scene0");
                commands.spawn((
                    SceneBundle {
                        scene: m,
                        transform: placing_res.placement.clone().unwrap(),
                        ..Default::default()
                    },
                    Name::new("Component | Laser mirror"),
                ));
            }
            PlacingObject::LaserPointer => {
                let m = asset_server.load("components/laser_pointer.glb#Scene0");
                commands.spawn((
                    SceneBundle {
                        scene: m,
                        transform: placing_res.placement.clone().unwrap(),
                        ..Default::default()
                    },
                    Name::new("Component | Laser pointer"),
                ));
            }
        }
    }
}
