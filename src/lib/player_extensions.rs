use std::time::{Duration, Instant};

use bevy::{
    input::mouse::MouseMotion,
    prelude::{
        shape::Plane, AssetServer, Assets, BuildChildren, Bundle, Camera, Camera3dBundle, Color,
        Commands, Component, Entity, EnvironmentMapLight, EventReader, GlobalTransform, Input,
        KeyCode, Mesh, PbrBundle, Query, Res, ResMut, SpatialBundle, StandardMaterial, Transform,
        Vec3, With,
    },
    window::{CursorGrabMode, Window},
};
use bevy_rapier3d::prelude::{
    ActiveCollisionTypes, Ccd, Collider, Damping, ExternalImpulse, KinematicCharacterController,
    LockedAxes, QueryFilter, RapierContext, RigidBody, Velocity,
};

use super::markers::{PlayerCameraChildMarker, PlayerParentMarker};

///
/// Player cycle:
///
/// 1. Add player
///
/// 2. Get his desired motion (wasd + jump queuing)
///
/// 3. Update his motion if possible
///
/// 4. Update camera rotation
///
/// 5. Update mesh rotation (TODO)
///
/// 6. Repeat 2-5
///

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
pub struct PlayerSettings {
    pub camera_mode: CameraPlayerMode,
    pub creation_mode: CreationMode,
}

#[derive(Component)]
pub struct JumpableCharacter {
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
pub struct PlayerParentBundle {
    pub marker: PlayerParentMarker,
    pub settings: PlayerSettings,

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
}

#[derive(Bundle)]
pub struct PlayerCameraChildBundle {
    pub marker: PlayerCameraChildMarker,
    pub camera: Camera3dBundle,
    pub env_map_light: EnvironmentMapLight,
}

pub fn add_player(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    commands
        .spawn(PlayerParentBundle {
            marker: PlayerParentMarker,
            settings: PlayerSettings {
                camera_mode: CameraPlayerMode::Body,
                creation_mode: CreationMode::Creative,
            },
            sp: SpatialBundle::INHERITED_IDENTITY,
            rb: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            character_control: KinematicCharacterController {
                up: Vec3::Y,
                offset: bevy_rapier3d::prelude::CharacterLength::Absolute(0.1),
                slide: true,
                autostep: Some(bevy_rapier3d::prelude::CharacterAutostep {
                    max_height: bevy_rapier3d::prelude::CharacterLength::Absolute(1.0),
                    min_width: bevy_rapier3d::prelude::CharacterLength::Absolute(0.01),
                    include_dynamic_bodies: false,
                }),
                max_slope_climb_angle: std::f32::consts::FRAC_PI_4,
                min_slope_slide_angle: std::f32::consts::FRAC_PI_8,
                snap_to_ground: Some(bevy_rapier3d::prelude::CharacterLength::Absolute(0.3)),
                ..Default::default()
            },
            collision_type: ActiveCollisionTypes::DYNAMIC_DYNAMIC
                | ActiveCollisionTypes::DYNAMIC_STATIC
                | ActiveCollisionTypes::DYNAMIC_KINEMATIC,
            jump: JumpableCharacter { jump_buffer: None },
            impulse: ExternalImpulse::default(),
            ccd: Ccd::enabled(),
            damping: Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            collider: Collider::capsule_y(0.8 - 0.4, 0.4),
            velocity: Velocity::zero(),
        })
        .with_children(|parent| {
            parent.spawn(PlayerCameraChildBundle {
                marker: PlayerCameraChildMarker,
                camera: Camera3dBundle {
                    camera: Camera {
                        ..Default::default()
                    },
                    projection: bevy::prelude::Projection::Perspective(
                        bevy::prelude::PerspectiveProjection {
                            fov: std::f32::consts::FRAC_PI_2,
                            ..Default::default()
                        },
                    ),
                    transform: Transform {
                        translation: Vec3::Y * 0.8,
                        ..Default::default()
                    },
                    camera_3d: bevy::prelude::Camera3d::default(),
                    ..Default::default()
                },
                env_map_light: EnvironmentMapLight {
                    diffuse_map: asset_server
                        .load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
                    specular_map: asset_server
                        .load("environment_maps/pisa_specular_rbg9e5_zstd.ktx2"),
                },
            });
        });
}

pub fn queue_player_jump(
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<&mut JumpableCharacter, With<PlayerParentMarker>>,
) {
    let Ok(mut p) = player_q.get_single_mut() else {
        return;
    };

    if keys.just_pressed(KeyCode::Space) {
        println!("buffered");
        p.jump_buffer = Some(Instant::now());
    }
}

pub fn tackle_jump(
    mut jumpable_queue: Query<(
        &mut ExternalImpulse,
        &mut JumpableCharacter,
        &KinematicCharacterController,
        &GlobalTransform,
        &mut Velocity,
        Entity,
    )>,
    rapier_context: Res<RapierContext>,
) {
    // TODO move from player to general
    for mut jumpable_object in jumpable_queue.iter_mut() {
        let Some(buf) = jumpable_object.1.jump_buffer else {
            continue;
        };
        let delta = Instant::now().duration_since(buf);

        let Some((_entity,toi)) = rapier_context.cast_ray(
            jumpable_object.3.translation(), Vec3::NEG_Y, 1.6, false, QueryFilter::new().exclude_collider(jumpable_object.5)) else {
            continue;
        };

        println!("toi: {}, buf: {:?}", toi, buf);

        // TODO un-hardocde buffer

        if delta.as_millis() < 300 && toi < 1.0 {
            jumpable_object.4.linvel.y = 0.0;
            jumpable_object.0.impulse += Vec3::new(0.0, 5.0, 0.0);
            println!("done"); // TODO LOSE SPEED
            jumpable_object.1.jump_buffer = None;
        } else if delta.as_millis() >= 300 {
            jumpable_object.1.jump_buffer = None;
        }
    }
    // TODO : save last jumped time, check it, check multiple rays;
}

pub fn move_camera(
    mut player_camera_transform_q: Query<&mut Transform, With<PlayerCameraChildMarker>>,

    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    for ev in mouse_motion_events.iter() {
        player_camera_transform_q.for_each_mut(|mut p| {
            p.rotate_y(-ev.delta.x / 300.);
            p.rotate_local_x(-ev.delta.y / 300.);
        });
    }
}

pub fn move_player(
    mut controllers: Query<&mut ExternalImpulse, With<KinematicCharacterController>>,
    mut player_camera_transform_q: Query<&Transform, With<PlayerCameraChildMarker>>,
    key: Res<Input<KeyCode>>,
) {
    let Ok(mut c) = controllers.get_single_mut() else {return;};
    // let Ok(p) = player_mesh_transform_q.get_single() else {return;};
    let Ok(cam) = player_camera_transform_q.get_single() else {return;};
    let mut v = Vec3::ZERO;

    if key.pressed(KeyCode::W) {
        v += cam.forward();
    }
    if key.pressed(KeyCode::D) {
        v += cam.right();
    }
    if key.pressed(KeyCode::A) {
        v += cam.left();
    }
    if key.pressed(KeyCode::S) {
        v += cam.back();
    }

    if let Some(x) = v.try_normalize() {
        v = x;
    }

    c.impulse += Vec3::new(0.4, 0.0, 0.4) * v;
}

pub fn spawn_debug_plane(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Plane::from_size(15.0).into()),
            material: materials.add(Color::rgb(0.3, 0.4, 0.8).into()),
            ..Default::default()
        })
        .insert(Collider::halfspace(Vec3::Y).unwrap());
}

pub fn prepare_cursor(
    mut windows: Query<&mut Window>,
    // btn: Res<Input<MouseButton>>,
    // key: Res<Input<KeyCode>>
) {
    let mut window = windows.single_mut();

    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}

pub fn unlock_cursor(
    mut windows: Query<&mut Window>,
    // btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        let mut window = windows.single_mut();
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
