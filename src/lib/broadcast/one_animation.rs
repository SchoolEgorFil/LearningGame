use bevy::prelude::{Handle, AnimationClip, Assets, Entity, warn, Events, AnimationPlayer, Name};

use crate::lib::tools::events::ButtonState;

use super::Action;

pub struct OneAnimationAction {
    pub is_started: bool,
    pub repeat: bool,
    pub me: Entity,
    pub was_played: bool,
    pub name: String,
    pub animation: Option<Handle<AnimationClip>>,
    pub id: u64,
}

impl Action for OneAnimationAction {
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
                let handle =  a.get_handle(
                    an.unwrap()
                );
                self.animation = Some(handle);
            } else {
                self.animation = None;
            }
            return;
        }
    }
    fn new(value: serde_json::Value, main: &serde_json::map::Map<String, serde_json::Value>) -> Self
        where
            Self: Sized {
                warn!("Todo: one_animation repeat");
        OneAnimationAction {
            id: value.as_u64().unwrap(),
            is_started: false,
            repeat: false,
            me: Entity::PLACEHOLDER,
            was_played: false,
            name: String::new(),
            animation: None,
        }
    }
    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        if self.was_played {
            return false;
        }
        
        let a = world.get_resource::<Events<ButtonState>>().unwrap();
        let mut b = a.get_reader();
        if let Some(a) = b.iter(a).find(|&p| p.id == self.id) {
            self.was_played = true;
                return true;
        }
        false
    }
    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        if self.animation.is_none() {
            println!("Animation is not found but played! {:?}", world.get_mut::<Name>(self.me).unwrap());
            return false;
        }
        let mut player = world.get_mut::<AnimationPlayer>(self.me).unwrap();
        player.play(self.animation.as_ref().unwrap().clone());
        if self.repeat {
            player.repeat();
        }
        true
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
}