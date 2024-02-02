use bevy::{prelude::{Handle, AnimationClip, Assets, Entity, warn, Events, AnimationPlayer, Name}, gltf::Gltf};

use crate::lib::tools::{events::ButtonState, resources::LoadingSceneInfo};

use super::Action;

pub struct NamedAnimationAction {
    pub is_started: bool,
    pub repeat: bool,
    pub me: Entity,
    pub was_played: bool,
    pub name: String,
    pub animation: Option<Handle<AnimationClip>>,
    pub animation_name: String,
    pub id: u64,
}

impl Action for NamedAnimationAction {
    fn try_startup(&mut self, me: bevy::prelude::Entity, world: &mut bevy::prelude::World) {
        if !self.is_started {
            self.me = me.clone();
            self.is_started = true;

            let a = world.get_resource::<LoadingSceneInfo>().unwrap();
            if let Some(b) = world.get_resource::<Assets<Gltf>>().unwrap().get(a.handle.clone_weak()) {
                println!("{}", self.animation_name);
                if let Some(animation) = b.named_animations.get(&self.animation_name) {
                    self.animation = Some(animation.clone());
                } else {
                    println!("Warn: No action for NamedAnimation found");
                }
                return;
            }
            
        }
        return;
    }
    fn new(value: serde_json::Value, main: &serde_json::map::Map<String, serde_json::Value>) -> Self
        where
            Self: Sized {
        warn!("Todo: name_animation repeat");
        let (id,name) = (
            value.as_array().unwrap()[0].as_str().unwrap().parse::<u64>().expect("named_animation argument 0 should be u64"),
            value.as_array().unwrap()[1].as_str().unwrap()
        );
        NamedAnimationAction {
            id: id,
            is_started: false,
            repeat: false,
            me: Entity::PLACEHOLDER,
            was_played: false,
            name: String::new(),
            animation: None,
            animation_name: name.into()
        }
    }
    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        if self.was_played {
            return false;
        }
        
        let a = world.get_resource::<Events<ButtonState>>().unwrap();
        let mut b = a.get_reader();
        if let Some(a) = b.read(a).find(|&p| p.id == self.id) {
            self.was_played = true;
                return true;
        }
        false
    }
    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        println!("Running: {:?}", world.get_mut::<Name>(self.me).unwrap());
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