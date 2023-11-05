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

pub struct LinkOpenerAction {
    pub startup: bool,
    pub link: Arc<String>,
    pub hint: Arc<String>,
    pub name: String,
    pub me: Entity
}

impl Default for LinkOpenerAction {
    fn default() -> Self {
        LinkOpenerAction {
            startup: false,
            name: "link_opener".into(),
            link: Arc::new("".into()),
            hint: Arc::new("Press E".into()),
            me: Entity::PLACEHOLDER
        }
    }
}

impl Action for LinkOpenerAction {
    fn new(value: Value, main: &serde_json::map::Map<String, Value>) -> Self {
        let link = value.as_str().unwrap();
        LinkOpenerAction {
            startup: false,
            name: "link_opener".into(),
            link: Arc::new(link.to_string()),
            hint: Arc::new("Press E".into()),
            me: Entity::PLACEHOLDER
        }
    }

    fn try_startup(&mut self, me: Entity, world: &mut World) {
        if !self.startup {
            self.me = me;
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

        let Some(q) = world.iter_entities().find(|p| p.contains::<PlayerCameraContainerMarker>()) else {
            return false;
        };

            let transform = q.get::<GlobalTransform>().unwrap();

            let door = world.get::<GlobalTransform>(self.me).unwrap();

            let d = (transform.translation() - door.translation()).length();

            if d > 2.7 {
                return false;
            }

            let Some(ctx) = world.get_resource::<RapierContext>() else {
                return false;
            };

            // todo!() move out into a funcion of its own

            if let Some((entity, toi)) = ctx.cast_ray(
                transform.translation(),
                transform.forward(),
                2.8,
                true,
                bevy_rapier3d::prelude::QueryFilter {
                    predicate: Some(&|entity| {
                        if entity == self.me {
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
        
        // println!("Usage door is {:?}", my_man.clone());
        return false;
    }

    fn execute(&mut self, world: &mut World) -> bool {
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
        webbrowser::open(self.link.as_str());
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
