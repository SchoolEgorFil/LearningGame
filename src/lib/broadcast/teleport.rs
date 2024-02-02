use bevy::{
    prelude::{Entity, Events, Name, Transform, Vec3},
};

use serde_json::Value;

use crate::lib::{tools::{events::ButtonState, resources::PlayerResource}};

use super::Action;

pub struct DelayedTeleportAction {
    pub is_started: bool,
    pub name: String,
    pub me: Entity,
    pub destination: TeleportDestination,
    pub id: u64,
}

pub enum TeleportDestination {
    Relative(Vec3),
    Absolute(Vec3),
    EntityStr(String),
    Entity(Entity),
}

impl Action for DelayedTeleportAction {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
    fn try_startup(&mut self, me: bevy::prelude::Entity, world: &mut bevy::prelude::World) {
        if !self.is_started {
            self.me = me.clone();
            self.is_started = true;
        }
        if let TeleportDestination::EntityStr(ref v) = self.destination {
            self.destination = TeleportDestination::Entity(
                world
                    .iter_entities()
                    .find(|p| **p.get::<Name>().unwrap() == *v)
                    .unwrap()
                    .id(),
            );
            // warn!("Teleporting to entity is not supported yet");
        }
    }
    fn new(value: serde_json::Value, _main: &serde_json::map::Map<String, serde_json::Value>) -> Self
    where
        Self: Sized,
    {
        let a = value.as_array().unwrap();
        let teleport = {
            let get_values = |a: &Vec<Value>| { Vec3::from((
                a[2].as_str().unwrap().parse::<f32>().unwrap(),
                a[3].as_str().unwrap().parse::<f32>().unwrap(),
                a[4].as_str().unwrap().parse::<f32>().unwrap()
            )) };
            match a[0].as_str().expect("Action:teleport is an array with first element to be a string") {
                "absolute" => TeleportDestination::Absolute(get_values(a)),
                "relative" => TeleportDestination::Relative(get_values(a)),
                "entity" => TeleportDestination::EntityStr(a[2].as_str().unwrap().to_string()),
                _ => panic!("abolute|relative|entity")
            }
        };

        DelayedTeleportAction {
            is_started: false,
            name: "TeleportAction".into(),
            me: Entity::PLACEHOLDER,
            destination: teleport,
            id: a[1].as_str().unwrap().parse::<u64>().unwrap(),
        }
    }

    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        let a = world.get_resource::<Events<ButtonState>>().unwrap();
        let mut b = a.get_reader();
        if let Some(_) = b.iter(a).find(|&p| p.id == self.id) {
            return true;
        }
        false
    }

    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        unsafe {
            let unsaf = world.as_unsafe_world_cell();
            let mut entity = unsaf.world_mut().entity_mut(
                unsaf
                    .world()
                    .get_resource::<PlayerResource>()
                    .unwrap()
                    .player_entity,
            );
            let og_transform = entity.get::<Transform>().unwrap();
            match self.destination {
                TeleportDestination::EntityStr(_) => {
                    panic!("EntityStr should be resolved in startup")
                }
                TeleportDestination::Absolute(d) => {
                    entity.insert(Transform {
                        rotation: og_transform.rotation,
                        scale: og_transform.scale,
                        translation: d,
                    });
                }
                TeleportDestination::Relative(d) => {
                    entity.insert(Transform {
                        rotation: og_transform.rotation,
                        scale: og_transform.scale,
                        translation: og_transform.translation + d,
                    });
                }
                TeleportDestination::Entity(e) => {
                    let transform = unsaf.world().entity(e).get::<Transform>().unwrap();
                    entity.insert(Transform {
                        rotation: og_transform.rotation,
                        scale: og_transform.scale,
                        translation: transform.translation,
                    });
                }
            }
        }
        true
    }
}
