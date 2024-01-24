use bevy::core::Name;
use bevy::prelude::{NextState, ResMut, State};
// use bevy::core_pipeline::Skybox;
// use bevy::prelude::{AmbientLight, AssetServer, EnvironmentMapLight};
use bevy::{
    prelude::{
        BuildChildren, Commands, Entity, EventReader, EventWriter, GlobalTransform, Input, KeyCode,
        Query, Res, SpatialBundle, Transform, Vec3, With,
    },
    utils::Instant,
};
use bevy_rapier3d::prelude::{
    ActiveCollisionTypes, Ccd, Collider, CollisionGroups, Damping, ExternalImpulse, Group,
    KinematicCharacterController, LockedAxes, QueryFilter, RapierContext, RigidBody, Velocity,
};

use crate::PlayerState;
use crate::lib::tools::resources::{PlayerResource, AllSettings};
use crate::lib::tools::{collision_groups, events, markers};

use super::components::{
    JumpableCharacter, PlayerBundle, PlayerCameraContainerBundle
};

use bevy::input::mouse::MouseMotion;

pub fn add_player(
    mut commands: Commands,
    mut player_ev_r: EventReader<events::SpawnPlayer>,
    mut camera_ev_w: EventWriter<events::SpawnPlayerCamera>,
) {
    
    if player_ev_r.len() != 1 {
        return;
    }
    let Some(x) = player_ev_r.read().next() else {panic!()};

    let id = commands
        .spawn(PlayerBundle {
            marker: markers::PlayerParentMarker,
            sp: SpatialBundle::from_transform(x.transform),
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
            collider: Collider::capsule_y(0.75 - 0.4, 0.4),
            velocity: Velocity::zero(),
            collision_group: 
                CollisionGroups::new(
                    Group::from_bits_retain(collision_groups::player_collision),
                    Group::from_bits_retain(collision_groups::player_collision),
                )
            ,
            name: Name::new("Player"),
        })
        .with_children(|p| {
            p.spawn(PlayerCameraContainerBundle {
                marker: markers::PlayerCameraContainerMarker,
                sp: SpatialBundle::from_transform(bevy::prelude::Transform::from_translation(
                    Vec3::Y * 0.67,
                )),
                name: Name::new("Player container for all cameras"),
            });
        })
        .id();

    commands.insert_resource(PlayerResource { player_entity: id });

    camera_ev_w.send(events::SpawnPlayerCamera {
        camera_params: x.camera_params.clone(),
    });
}

pub fn move_player(
    mut controllers: Query<&mut ExternalImpulse, With<KinematicCharacterController>>,
    player_camera_transform_q: Query<&Transform, With<markers::PlayerCameraContainerMarker>>,
    key: Res<Input<KeyCode>>,
    state: Res<State<PlayerState>>,

    mut windows: Query<&mut bevy::prelude::Window>,
    // btn: Res<Input<MouseButton>>,
    // mut next_state: ResMut<NextState<GameState>>,
) {
    if *state != PlayerState::Interactive {
        return;
    }
    
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
    if key.just_pressed(KeyCode::P) {
        let mut window = windows.single_mut();
        // window.cursor.grab_mode = CursorGrabMode::None;
        // next_state.0 = Some(GameState::MainMenu);
        window.cursor.visible = !window.cursor.visible;
    }

    if let Some(x) = v.try_normalize() {
        v = x;
    }

    c.impulse += Vec3::new(0.2, 0.0, 0.2) * v;
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
    state: Res<State<PlayerState>>
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

        // println!("toi: {}, buf: {:?}", toi, buf);

        // TODO un-hardocde buffer

        if delta.as_millis() < 300 && toi < 1.0 && *state == PlayerState::Interactive {
            jumpable_object.4.linvel.y = 0.0;
            jumpable_object.0.impulse += Vec3::new(0.0, 3.5, 0.0);
            // println!("done"); // TODO LOSE SPEED
            jumpable_object.1.jump_buffer = None;
        } else if delta.as_millis() >= 300 {
            jumpable_object.1.jump_buffer = None;
        }
    }
    // TODO : save last jumped time, check it, check multiple rays;
}

pub fn queue_player_jump(
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<&mut JumpableCharacter, With<markers::PlayerParentMarker>>,
    state: Res<State<PlayerState>>
) {
    if *state != PlayerState::Interactive {
        return;
    }
    
    let Ok(mut p) = player_q.get_single_mut() else {
        return;
    };

    if keys.just_pressed(KeyCode::Space) {
        // println!("buffered");
        p.jump_buffer = Some(Instant::now());
    }
}

pub fn move_camera(
    mut player_camera_transform_q: Query<
        &mut Transform,
        With<markers::PlayerCameraContainerMarker>,
    >,
    player: Res<AllSettings>,

    mut mouse_motion_events: EventReader<MouseMotion>,
    state: Res<State<PlayerState>>
) {
    if *state != PlayerState::Interactive {
        return;
    }
    
    for ev in mouse_motion_events.read() {
        player_camera_transform_q.for_each_mut(|mut p| {
            p.rotate_y(-ev.delta.x / 300. / ((125. - player.fov)/25.));
            p.rotate_local_x(-ev.delta.y / 300. / ((125. - player.fov)/25.));
        });
    }
}


pub fn unrestrict_player(
    mut state: ResMut<NextState<PlayerState>>
) {
    state.0 = Some(PlayerState::Interactive);
}


pub fn restrict_player(
    mut state: ResMut<NextState<PlayerState>>
) {
    state.0 = Some(PlayerState::Absent);
}