
use super::attachements::ColliderType;
use super::broadcast::{self, Action};
use bevy::prelude::Color;
use bevy_rapier3d::prelude::{
    RigidBody,
};
use serde_json::Value;

///
/// spawn_point
///     | ambient_intensity: f32
///     | ambient_color: (f32,f32,f32,f32)
///     | skybox: string
///
///
/// mesh_collider_marker
///     | collider_type = "tris" | "hull" | "decomposition" | "ball" | "cuboid" | "cone" | "height_map" | "from_mesh_convex"
///	| density = f64
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
/// action:collision_audio = string
/// #collision_audio_volume = f64
/// #collision_audio_cooldown = f64
///
/// action:delayed_teleport = u64
/// #teleport_delay = f64
/// #teleport_destination_absolute = [f32;3]
/// #teleport_destination_relative = [f32;3]
/// #teleport_destination_entity = Name (string)
/// 
/// 
/// action:collision_button = u64
///
pub enum CustomProps {
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
    Light {
        shadows: bool,
    },
    MassProp(f32),
    CollisionAudio(String), // todo volume etc...

    Action(Box<dyn Action>),
}

impl CustomProps {
    pub fn convert(name: &String, value: &Value, main: &serde_json::map::Map<String, Value>) -> Self {
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
        if name == "shadows" {
            // let int = match main.get("sun_intensity").and_then(|p| p.as_f64()) {
            //     Some(v) => v as f32,
            //     _ => panic!("sun intensity is not set"),
            // };
            // let color = match main
            //     .get("sun_color")
            //     .and_then(|p| p.as_array())
            //     .and_then(|p| {
            //         p.get(0..4).and_then(|x| {
            //             x.iter()
            //                 .map(|p| p.as_f64().and_then(|p| Some(p as f32)))
            //                 .collect::<Option<Vec<f32>>>()
            //         })
            //     }) {
            //     Some(v) => v,
            //     _ => panic!("sun color is not set"),
            // };
            // let shadows = match main.get("sun_shadows") {
            //     Some(v) => v.as_bool().unwrap(),
            //     _ => panic!("sun shadows is not set"),
            // };
            return CustomProps::Light {
                // intensity: int,
                // color: Color::Rgba {
                //     red: color[0],
                //     green: color[1],
                //     blue: color[2],
                //     alpha: color[3],
                // },
                shadows: value.as_bool().unwrap_or(false),
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
                    let  a = broadcast::open_door::OpenDoorAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "ball_falling_01" => {
                    let a =
                        broadcast::ball_falling_01::BallFalling01Action::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "stand_button" => {
                    let  a =
                        broadcast::stand_button::StandButtonAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "collision_button" => {
                    let a =
                        broadcast::collision_button::CollisionButtonAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }   
                // "explosion_test_01" => {
                //     let  a = broadcast::explosion_test_01::ExplosionTestAction::new(
                //         value.clone(),
                //         &main,
                //     );
                //     return CustomProps::Action(Box::new(a));
                // }
                "collision_audio" => {
                    let  a =
                        broadcast::collision_audio::CollisionAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "teleport" => {
                    let  a = broadcast::teleport::DelayedTeleportAction::new(
                        value.clone(),
                        &main,
                    );
                    return CustomProps::Action(Box::new(a));
                }
                "one_animation" => {
                    let  a =
                        broadcast::one_animation::OneAnimationAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "full_animation" => {
                    let  a =
                        broadcast::full_animation::FullAnimationAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "named_animation" => {
                    let  a =
                        broadcast::named_animation::NamedAnimationAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "delay_trasmitter" => {
                    let  a = 
                        broadcast::delay::DelayedAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "delay_transmitter" => {
                    let  a = 
                        broadcast::delay::DelayedAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "link" => {
                    let a = 
                    broadcast::link_opener::LinkOpenerAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "input_field" => {
                    let  a = 
                        broadcast::input_field::InputFieldAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }
                "test_chamber" => {
                    let a = 
                        broadcast::test_chamber::TestChamberAction::new(value.clone(), &main);
                    return CustomProps::Action(Box::new(a));
                }

                unknown => {
                    println!("There is some custom property unhandled! Name is {}", unknown);
                }
            }
        }
        CustomProps::_Unhandled
    }
}