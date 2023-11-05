use std::time::{Duration, Instant};

use bevy_kira_audio::{Audio, AudioControl, AudioSource};

use bevy::{
    prelude::{AssetServer, Entity, Handle},
    time::Time,
};
use bevy_rapier3d::prelude::RapierContext;

use crate::lib::tools::markers::{PlayerCameraContainerMarker, PlayerParentMarker};

use super::Action;

#[derive(Debug)]
pub struct CollisionAction {
    me: Entity,
    audio: Handle<AudioSource>,
    pub volume: f32,
    pub recursive_cooldown: Option<Duration>,
    pub last_played: Option<Instant>,
    pub was_colliding: bool,
    pub is_startupped: bool,
    pub additional_info: Option<Box<(String, Option<f64>, Option<f64>)>>,
    
}

impl Default for CollisionAction {
    fn default() -> Self {
        CollisionAction {
            me: Entity::PLACEHOLDER,
            audio: Handle::default(),
            volume: 0.5,
            recursive_cooldown: None,
            last_played: None,
            was_colliding: false,
            is_startupped: false,
            additional_info: Some(Box::new(("".into(), None, None))),
        }
    }
}

impl Action for CollisionAction {
    fn name(&self) -> String {
        "CollisionAction".into()
    }
    fn try_startup(&mut self, me: bevy::prelude::Entity, world: &mut bevy::prelude::World) {
        if self.additional_info.is_none() {
            return;
        }
        let s = self.additional_info.clone().unwrap();
        self.me = me;
        self.audio = world
            .get_resource::<AssetServer>()
            .unwrap()
            .load(s.0.clone());
        if s.1.is_some() {
            self.volume = s.1.unwrap() as f32;
        }
        if s.2.is_some() {
            self.recursive_cooldown = Some(Duration::from_secs_f64(s.2.unwrap()));
        }
        self.additional_info = None;
        // println!("{:?}", self);
        self.is_startupped = true;
    }
    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        let Some(t) = world.iter_entities().find(|p| p.contains::<PlayerParentMarker>()) else {
            return false;
        };
        let s = world
            .get_resource::<RapierContext>()
            .unwrap()
            .intersection_pair(t.id(), self.me);
        // println!("{:?}", s.clone());

        if s == Some(true) {
            if !self.was_colliding
                && (self.last_played.is_none()
                    || (self.recursive_cooldown.is_some_and(|cooldown| {
                        !cooldown.is_zero() && 
                        world
                            .get_resource::<Time>()
                            .unwrap()
                            .last_update()
                            .unwrap()
                            .duration_since(self.last_played.unwrap())
                            > cooldown
                    })))
            {
                world
                    .get_resource::<Audio>()
                    .unwrap()
                    .play(self.audio.clone());
            }
            self.last_played = Some(world.get_resource::<Time>().unwrap().last_update().unwrap());
        }
        false
    }
    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        false
    }
    fn change_name(&mut self, name: String) {}
    fn new(value: serde_json::Value, main: &serde_json::map::Map<String, serde_json::Value>) -> Self
    where
        Self: Sized,
    {
        CollisionAction {
            additional_info: Some(Box::new((
                value.as_str().unwrap().to_owned(),
                main.get("#collision_audio_volume").and_then(|p| p.as_f64()),
                main.get("#collision_audio_cooldown")
                    .and_then(|p| p.as_f64()),
            ))),
            ..Default::default()
        }
    }
}
