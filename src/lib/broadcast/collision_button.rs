use std::{sync::Arc, time::Duration};

use bevy::{prelude::{Entity, warn}, utils::Instant};
use bevy_rapier3d::prelude::RapierContext;

use crate::lib::{player_control::components::PlayerResource, tools::events::ButtonState};

use super::Action;

pub struct CollisionButtonAction {
    pub startup: bool,
    pub name: String,
    pub is_pressed: bool,
    pub can_be_pressed: bool,
    pub me: Entity,
    pub retarget_index: u64,
}

impl Default for CollisionButtonAction {
    fn default() -> Self {
        CollisionButtonAction {
            startup: false,
            name: "collision_button".into(),
            is_pressed: false,
            can_be_pressed: true,
            me: Entity::PLACEHOLDER,
            retarget_index: 0,
        }
    }
}

impl Action for CollisionButtonAction {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
    fn try_startup(&mut self, me: Entity, world: &mut bevy::prelude::World) {
        if !self.startup {
            self.me = me;
            self.startup = true;
        }
    }
    fn new(value: serde_json::Value, main: &serde_json::map::Map<String, serde_json::Value>) -> Self
    where
        Self: Sized,
    {
        let mut a = CollisionButtonAction::default();
        a.retarget_index = value.as_u64().unwrap();
        a
    }

    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        if !self.can_be_pressed {
            return false;
        }

        let Some(t) = world.get_resource::<PlayerResource>() else {
            warn!("glTF is loaded but no player is found");
            return false;
        };

        let s = world
            .get_resource::<RapierContext>()
            .unwrap()
            .intersection_pair(t.player_entity, self.me);

        if s == Some(true) && !self.is_pressed {
            world.send_event(ButtonState {
                id: self.retarget_index,
                is_pressed: true,
                just_changed: true,
            });
            self.is_pressed = true;
        } else if self.is_pressed {
            world.send_event(ButtonState {
                id: self.retarget_index,
                is_pressed: false,
                just_changed: true,
            });
            self.is_pressed = false;
        }

        false
    }
    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        unreachable!();
    } 
}
