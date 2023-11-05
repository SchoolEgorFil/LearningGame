use std::{rc::Rc, sync::Arc};

use bevy::{
    prelude::{
        Children, DetectChangesMut, Entity, Event, Events, GlobalTransform, Input, KeyCode, Name,
        Parent, Transform, World,
    },
    text::TextStyle,
};
use bevy_rapier3d::prelude::{CollisionGroups, Group, QueryFilterFlags, RapierContext};
use serde_json::Value;

use crate::lib::tools::{
    collision_groups,
    events::{self, ButtonState, ModifyCollisionGroup, ProposePopup},
    markers::{PlayerCameraContainerMarker, PlayerParentMarker},
};

use super::Action;

pub struct OpenDoorAction {
    pub startup: bool,
    pub name: String,
    pub is_opened: bool,
    pub opening_strategy: DoorOpenStrategy,
    pub hint: Arc<String>,
    pub usage_area_entity: Entity,
    pub door_top_entity: Entity,
    pub door_bottom_entity: Entity,
}

impl Default for OpenDoorAction {
    fn default() -> Self {
        OpenDoorAction {
            startup: false,
            name: "open_door".into(),
            is_opened: false,
            opening_strategy: DoorOpenStrategy::Player,
            hint: Arc::new("Open door".into()),
            door_top_entity: Entity::PLACEHOLDER,
            door_bottom_entity: Entity::PLACEHOLDER,
            usage_area_entity: Entity::PLACEHOLDER,
        }
    }
}

impl Action for OpenDoorAction {
    fn new(value: Value, main: &serde_json::map::Map<String, Value>) -> Self {
        let mut a = OpenDoorAction::default();
        if value.as_str() != Some("openable") {
            println!("Door opening strategy is: {}", value.as_str().unwrap());
            a.opening_strategy =
                DoorOpenStrategy::HandledBy(value.as_str().unwrap().parse::<u64>().unwrap());
        }
        a
    }

    fn try_startup(&mut self, me: Entity, world: &mut World) {
        if !self.startup {
            for child in world.get::<Children>(me).unwrap() {
                if let Some(s) = world.get::<Name>(child.clone()) {
                    // println!("{:?}",s);
                    if s.starts_with("system:Usage_door") {
                        self.usage_area_entity = child.clone();
                    } else if s.starts_with("system:Door_Left") {
                        self.door_top_entity = child.clone();
                    } else if s.starts_with("system:Door_Right") {
                        self.door_bottom_entity = child.clone();
                    }
                }
            }

            if self.door_top_entity == Entity::PLACEHOLDER
                || self.usage_area_entity == Entity::PLACEHOLDER
                || self.door_bottom_entity == Entity::PLACEHOLDER
            {
                panic!();
            }

            world.send_event(events::ModifyCollisionGroup {
                entity: self.usage_area_entity.clone(),
                flags: u32::MAX - 1u32,
                members: u32::MAX - 1u32,
                override_groups: true,
            });

            self.startup = true;
        }
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
    fn name(&self) -> String {
        self.name.clone()
    }

    fn predicate(&mut self, world: &mut World) -> bool {
        // check if app state is good
        let my_man = self.usage_area_entity;

        let Some(q) = world.iter_entities().find(|p| p.contains::<PlayerCameraContainerMarker>()) else {
            return false;
        };

        if self.opening_strategy == DoorOpenStrategy::Player {
            let transform = q.get::<GlobalTransform>().unwrap();

            let door = world.get::<GlobalTransform>(my_man).unwrap();

            let d = (transform.translation() - door.translation()).length();

            if d > 1.3 {
                return false;
            }

            let Some(ctx) = world.get_resource::<RapierContext>() else {
                return false;
            };

            // todo!() move out into a funcion of its own

            if let Some((entity, toi)) = ctx.cast_ray(
                transform.translation(),
                transform.forward(),
                2.,
                true,
                bevy_rapier3d::prelude::QueryFilter {
                    predicate: Some(&|entity| {
                        if entity == my_man {
                            // dbg!(entity);
                            return true;
                        }
                        return false;
                    }),
                    ..Default::default()
                },
            ) {
                return true;
            }
        } else if let DoorOpenStrategy::HandledBy(whomm) = self.opening_strategy {
            //ok
            return true;
        } else {
            println!("Door open strat is not handled");
        }
        // println!("Usage door is {:?}", my_man.clone());
        return false;
    }

    fn execute(&mut self, world: &mut World) -> bool {
        let should_be_opened;
        if self.opening_strategy == DoorOpenStrategy::Player {
            world.send_event(ProposePopup {
                text: self.hint.clone(),
                priority: 1,
                style: TextStyle {
                    font_size: 26.0,
                    ..Default::default()
                },
                key: Some(KeyCode::E),
            });

            let Some(ok) = world.get_resource::<Input<KeyCode>>() else {
                return false;
            };

            if ok.just_pressed(KeyCode::E) == false {
                return false;
            }
            should_be_opened = !self.is_opened;
        } else if let DoorOpenStrategy::HandledBy(whom) = self.opening_strategy {
            // cool it is, then
            let a = world.get_resource::<Events<ButtonState>>().unwrap();
            let mut b = a.get_reader();
            if let Some(a) = b.iter(a).find(|&p| p.id == whom) {
                should_be_opened = a.is_pressed;
            } else {
                return false;
            }
        } else {
            return false;
        }

        if should_be_opened {
            // println!("llol");
            world
                .get_mut::<Transform>(self.door_top_entity)
                .unwrap()
                .rotate_y(std::f32::consts::FRAC_PI_2);
            world
                .get_mut::<Transform>(self.door_bottom_entity)
                .unwrap()
                .rotate_y(-std::f32::consts::FRAC_PI_2);
            self.is_opened = true;
        } else {
            world
                .get_mut::<Transform>(self.door_top_entity)
                .unwrap()
                .rotate_y(-std::f32::consts::FRAC_PI_2);
            world
                .get_mut::<Transform>(self.door_bottom_entity)
                .unwrap()
                .rotate_y(std::f32::consts::FRAC_PI_2);
            self.is_opened = false;
        }
        true //:D
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum DoorOpenStrategy {
    Player,
    HandledBy(u64),
    Obstructed,
    Broken,
}
