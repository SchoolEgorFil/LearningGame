use bevy::prelude::{Handle, AnimationClip, Assets, Entity, warn, Events, AnimationPlayer, Name};

use crate::lib::tools::events::ButtonState;

use super::Action;

pub struct FullAnimationAction {
    pub is_started: bool,
    pub me: Entity,
    pub name: String,
    pub animation: Option<Handle<AnimationClip>>,

    pub repeat: u64,
    pub loops: u64,
    pub loops_passed: u64,
    pub activation_id: u64,
    pub deactivation_id: u64,
}

impl Action for FullAnimationAction {
    fn try_startup(&mut self, me: bevy::prelude::Entity, world: &mut bevy::prelude::World) {
        if !self.is_started {
            self.me = me.clone();
            self.is_started = true;
            let name = world.get::<Name>(self.me).unwrap().clone();
            let mut a = world.get_resource_mut::<Assets<AnimationClip>>().unwrap();
            let an = a.iter().find(|p| {
                p.1.compatible_with(&name)
            }).and_then(|p| Some(p.0));
            if an.is_some() {
                let handle =  Handle::Weak(an.unwrap());
                self.animation = Some(handle);
            } else {
                self.animation = None;
            }
            return;
        }
    }
    fn new(value: serde_json::Value, main: &serde_json::map::Map<String, serde_json::Value>) -> Self
        where Self: Sized {
        let v = value.as_array().unwrap();
        println!("{:?}", v);
        let (activation_id,repeat,loops,deactivation_id) = (
            v[0].as_f64().unwrap() as u64,
            v[1].as_f64().unwrap() as u64,
            v[2].as_f64().unwrap() as u64,
            v[3].as_f64().unwrap() as u64,
        );


        
        FullAnimationAction {            
            is_started: false,
            me: Entity::PLACEHOLDER,
            name: String::new(),
            animation: None,

            activation_id,
            deactivation_id,
            loops,
            loops_passed: 0,
            repeat
        }
    }
    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        unsafe {
            let cell = world.as_unsafe_world_cell();
        let a = cell.world().get_resource::<Events<ButtonState>>().unwrap();
        let mut b = a.get_reader();
        for event in b.read(a) {
            if event.is_pressed {
                if event.id == self.activation_id {
                    if self.loops != 0 && self.loops_passed == self.loops {
                        return false;
                    }
                    self.loops_passed += 1;
                    return true;
                } else if event.id == self.deactivation_id {
                    let mut player = cell.world_mut().get_mut::<AnimationPlayer>(self.me);
                    if let Some(mut player) = player {
                        player.pause();
                    }
                }
            }
        }
        false
        }
    }
    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        if self.animation.is_none() {
            println!("Animation is not found but played! {:?}", world.get_mut::<Name>(self.me).unwrap());
            return false;
        }
        let mut player = world.get_mut::<AnimationPlayer>(self.me).unwrap();
        player.start(self.animation.as_ref().unwrap().clone());
        match self.repeat {
            0 => player.set_repeat(bevy::animation::RepeatAnimation::Forever),
            1 => player.set_repeat(bevy::animation::RepeatAnimation::Never),
            n => player.set_repeat(bevy::animation::RepeatAnimation::Count(n as u32)),
        };
        true
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
}