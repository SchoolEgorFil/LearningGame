use std::time::Duration;

use bevy::{
    prelude::{warn, Entity, Events, Name, Transform, Vec3},
    time::Time,
    utils::Instant,
};
use bevy_rapier3d::prelude::RapierContext;
use itertools::izip;

use crate::lib::{tools::events::{ButtonState, CustomEvent}};

use super::Action;

pub struct Route {
    pub trigger: u64,
    pub right_path: u64,
    pub wrong_path: u64,
    pub answer: String
}

pub struct TestChamberAction {
    pub is_started: bool,
    pub is_triggered: bool,
    pub trigger: u64,
    pub routes: Vec<Route>,
    pub repeats_globally: bool,
    pub chosen_route_index: usize,
    pub name: String,
    pub input_field: String
}

impl Action for TestChamberAction {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
    fn try_startup(&mut self, me: bevy::prelude::Entity, world: &mut bevy::prelude::World) {
        if !self.is_started {
            
            self.is_started = true;
        }
    }
    fn new(value: serde_json::Value, main: &serde_json::map::Map<String, serde_json::Value>) -> Self
    where
        Self: Sized,
    {
        let binding = main.get("#test_chamber_routes").unwrap().as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect::<Vec<_>>();
        let q = binding;
        let binding = main.get("#test_chamber_wrongs").unwrap().as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect::<Vec<_>>();
        let w = binding;
        let binding = main.get("#test_chamber_rights").unwrap().as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect::<Vec<_>>();
        let r = binding;
        let binding = main.get("#test_chamber_answer").unwrap().as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        let a = binding;

        if q.len() != w.len() || q.len() != r.len() || q.len() != a.len() {
            panic!("action:test_chamber is all tangled up");
        }

        let len = q.len();

        unsafe {        
            // i don't care about repeating for now

            TestChamberAction {
                routes: izip!(q, w, r, a).map(|p| Route {
                    trigger: p.0.to_owned(),
                    wrong_path: p.1.to_owned(),
                    right_path: p.2.to_owned(),
                    answer: p.3.to_owned()
                }).collect::<Vec<_>>(),
                repeats_globally: main.get("#test_chamber_repeats_globally").unwrap().as_bool().unwrap(),
            // i don't care about repeating for now
                chosen_route_index: { /* let a =  */ (rand::random::<f32>() * len as f32).floor() as usize /* ; println!("{}", a); a  */},
                trigger: value.as_u64().unwrap(),
                is_started: false,
                is_triggered: false,
                name: "test chamber".into(),
                input_field: main.get("#test_chamber_input_name").unwrap().as_str().unwrap().to_owned()
            }
        }
    }

    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        if self.is_triggered {
            let a = world.get_resource::<Events<CustomEvent>>().unwrap();
            let mut b = a.get_reader();
            if let Some(a) = b.read(a).find(|&p| p.name == self.input_field) {
                // std::fs::write("foo.txt",format!("INPUT: {} vs {}", a.json_encoded, self.routes[self.chosen_route_index].answer));
                if a.json_encoded.replace(|c: char| !c.is_ascii(), "") == self.routes[self.chosen_route_index].answer {
                    world.get_resource_mut::<Events<ButtonState>>()
                        .unwrap().send(ButtonState { is_pressed: true, just_changed: true, id: self.routes[self.chosen_route_index].right_path });
                } else {
                    world.get_resource_mut::<Events<ButtonState>>()
                        .unwrap().send(ButtonState { is_pressed: true, just_changed: true, id: self.routes[self.chosen_route_index].wrong_path });
                }
            }
        } else {
            let a = world.get_resource::<Events<ButtonState>>().unwrap();
            let mut b = a.get_reader();
            if let Some(a) = b.read(a).find(|&p| p.id == self.trigger) {
                if a.is_pressed && a.just_changed && !self.is_triggered {
                    self.is_triggered = true;
                    return true;
                }
            }
        }
        false
    }

    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        world.get_resource_mut::<Events<ButtonState>>()
            .unwrap().send(ButtonState { is_pressed: true, just_changed: true, id: self.routes[self.chosen_route_index].trigger });
        true
    }
}
