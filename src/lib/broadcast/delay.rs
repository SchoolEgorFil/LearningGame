use std::time::Duration;

use bevy::{
    prelude::{warn, Entity, Events, Name, Transform, Vec3},
    time::Time,
    utils::Instant,
};
use bevy_rapier3d::prelude::RapierContext;

use crate::lib::{tools::events::ButtonState};

use super::Action;

pub struct DelayedAction {
    // pub is_started: bool,
    pub start_time: Option<Duration>,
    pub duration: Duration,
    pub from_id: u64,
    pub to_id: u64,
    pub name: String,
    pub button_state: Option<ButtonState>,
    pub will_not_override: bool,
    pub only_edge: bool
}

impl Action for DelayedAction {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
    fn try_startup(&mut self, me: bevy::prelude::Entity, world: &mut bevy::prelude::World) {
        // if !self.is_started {
        //     self.me = me.clone();
        //     if let TeleportDestination::EntityStr(ref v) = self.destination {
        //         self.destination = TeleportDestination::Entity(
        //             world
        //                 .iter_entities()
        //                 .find(|p| **p.get::<Name>().unwrap() == *v)
        //                 .unwrap()
        //                 .id(),
        //         );
        //         // warn!("Teleporting to entity is not supported yet");
        //     }
        //     self.is_started = true;
        // }
    }
    fn new(value: serde_json::Value, main: &serde_json::map::Map<String, serde_json::Value>) -> Self
    where
        Self: Sized,
    {
        let a = value.as_array().unwrap();
        if a.len() != 5 {panic!("Delay Action met incompatible number array. {:?}",a)}
        let (from_id,to_id,delay,will_not_override,only_edge) = 
            (a[0].as_f64().unwrap() as u64,
            a[1].as_f64().unwrap() as u64,
            a[2].as_f64().unwrap(),
            match a[3].as_f64().unwrap() { 1. => true, 0. => false, _ => panic!("Delay Action argument a[3] is incorrect") },
            match a[4].as_f64().unwrap() { 1. => true, 0. => false, _ => panic!("Delay Action argument a[4] is incorrect") },
        );

        println!("Delay transmitter goes {} to {} in {} seconds", from_id, to_id, delay);

        DelayedAction {
            name: "DelayedTeleportAction".into(),
            duration: Duration::from_secs_f64(delay),
            start_time: None,
            from_id,
            to_id,
            button_state: None,
            will_not_override,
            only_edge
        }
    }

    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        if self.start_time.is_some()
            && world
                .get_resource::<Time>()
                .unwrap()
                .elapsed()
                -
                self.start_time.unwrap()
                > self.duration
        {
            self.start_time = None;
            return true;
        } else if self.start_time.is_some() && self.will_not_override {
            return false;
        }

        let a = world.get_resource::<Events<ButtonState>>().unwrap();
        let mut b = a.get_reader();
        if let Some(a) = b.read(a).find(|&p| p.id == self.from_id) {
            if self.start_time.is_none() {
                println!("{:?} |  {:?}", self.only_edge, a.just_changed);
                if self.only_edge && !a.just_changed {
                    return false;
                }

                println!("Delay started");

                self.start_time = Some(world
                    .get_resource::<Time>()
                    .unwrap()
                    .elapsed());
                self.button_state = Some(ButtonState {
                    id: self.to_id,
                    is_pressed: a.is_pressed,
                    just_changed: a.just_changed
                });
                return false;
            }
        }
        false
    }

    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        println!("Delay ended");
        world.get_resource_mut::<Events<ButtonState>>()
            .unwrap().send(self.button_state.as_ref().unwrap().clone());
        self.start_time = None;
        self.button_state = None;
        true
    }
}
