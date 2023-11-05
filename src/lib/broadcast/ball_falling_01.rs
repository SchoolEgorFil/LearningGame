use std::sync::Arc;

use crate::lib::tools::events::{self, ButtonState};
use bevy::prelude::DetectChangesMut;
use bevy::prelude::{Children, Entity, Events, Name, Parent, Quat, Transform, Vec3, World};
use bevy_rapier3d::prelude::{RigidBody, Sleeping, Velocity};
use serde_json::Value;

use super::Action;

pub struct BallFalling01Action {
    pub startup: bool,
    pub starting_position: Transform,
    pub name: String,
    pub hint: Arc<String>,
    pub cell_entity: Entity,
    pub ball: Entity,
    pub start_button_i: u64,
    pub reset_button_i: u64,
    pub action_type: u64,
}

impl Default for BallFalling01Action {
    fn default() -> Self {
        BallFalling01Action {
            startup: false,
            name: "ball_falling_01".into(),
            hint: Arc::new("Open door".into()),
            cell_entity: Entity::PLACEHOLDER,
            ball: Entity::PLACEHOLDER,
            starting_position: Transform::default(),
            start_button_i: 0,
            reset_button_i: 0,
            action_type: 0,
        }
    }
}

impl Action for BallFalling01Action {
    fn new(value: Value, main: &serde_json::map::Map<String, Value>) -> Self {
        let mut a = BallFalling01Action::default();
        a.start_button_i = value.as_array().unwrap()[0].as_u64().unwrap();
        a.reset_button_i = value.as_array().unwrap()[1].as_u64().unwrap();
        a
    }

    fn try_startup(&mut self, me: Entity, world: &mut World) {
        if !self.startup {
            self.cell_entity = me.clone();

            for child in world.get::<Children>(me).unwrap() {
                if let Some(s) = world.get::<Name>(child.clone()) {
                    if s.starts_with("TheBall") {
                        // println!("w");
                        self.ball = child.clone();
                        self.starting_position = world.get::<Transform>(self.ball).unwrap().clone();
                    }
                }
            }
            // world.insert_or_spawn_batch(vec![(self.ball,Velocity {..Default::default()})]);
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
        let a = world.get_resource::<Events<ButtonState>>().unwrap();
        let mut b = a.get_reader();
        for ev in b.iter(a) {
            if ev.is_pressed && ev.just_changed {
                if ev.id == self.reset_button_i {
                    self.action_type = 1;
                    return true;
                } else if ev.id == self.start_button_i {
                    self.action_type = 2;
                    return true;
                }
            }
        }

        false
    }

    fn execute(&mut self, world: &mut World) -> bool {
        // println!("{:?}",*world.get::<Name>(self.ball).unwrap());
        // println!("=====");
        // println!("{:?}",self.starting_position);
        if self.action_type == 1 {
            *world.get_mut::<RigidBody>(self.ball).unwrap() = RigidBody::Fixed;
            world.get_mut::<RigidBody>(self.ball).unwrap().set_changed();

            *world.get_mut::<Transform>(self.ball).unwrap() = self.starting_position.clone();

            // println!("sdsddsdsasasaaaaaaaa {:?}", a);
            // println!("=====");
            *world.get_mut::<Velocity>(self.ball).unwrap() = Velocity::default();
            let mut a = world.get_mut::<Sleeping>(self.ball).unwrap();
            a.sleeping = false;
            a.set_changed();
        } else if self.action_type == 2 {
            *world.get_mut::<RigidBody>(self.ball).unwrap() = RigidBody::Dynamic;
            let mut a = world.get_mut::<Sleeping>(self.ball).unwrap();
            a.sleeping = false;
            a.set_changed();
        }
        true //:D
    }
}
