use bevy::{
    gltf::Gltf,
    prelude::{
        AnimationClip, AnimationPlayer, Assets, Children, Entity, EntityPath, GlobalTransform,
        Handle, Name, Parent, Res, Transform,
    },
    reflect::Struct,
    utils::HashMap,
};
use bevy_rapier3d::parry::partitioning::IndexedData;

use crate::lib::{tools::markers::PlayerCameraContainerMarker};

use super::Action;

pub struct ExplosionTestAction {
    pub is_started: bool,
    pub me: Entity,
    pub was_played: bool,
    pub name: String,
    pub animations: HashMap<Name, Handle<AnimationClip>>,
}

impl Default for ExplosionTestAction {
    fn default() -> Self {
        ExplosionTestAction {
            is_started: false,
            me: Entity::PLACEHOLDER,
            was_played: false,
            name: String::new(),
            animations: HashMap::new(),
        }
    }
}

impl Action for ExplosionTestAction {
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
    fn new(value: serde_json::Value, main: &serde_json::map::Map<String, serde_json::Value>) -> Self
    where
        Self: Sized,
    {
        ExplosionTestAction::default()
    }

    fn name(&self) -> String {
        "Explosion_test_action_01".into()
    }
    fn try_startup(&mut self, me: bevy::prelude::Entity, world: &mut bevy::prelude::World) {
        if !self.is_started {
            self.me = me.clone();
            self.is_started = true;
            return;

            let name = {
                let ent = world.get::<Parent>(me).unwrap().get();
                [
                    world.get::<Name>(ent).unwrap().clone(),
                    world.get::<Name>(me).unwrap().clone(),
                ]
            };
            let mut a = world.get_resource_mut::<Assets<AnimationClip>>().unwrap();
            let mut handle_ids = vec![];
            for (id, clip) in a.iter_mut() {
                if clip.compatible_with(&name[0]) {
                    {
                        let hashmap = (clip as &mut dyn Struct)
                            .field_mut("paths")
                            .unwrap()
                            .downcast_mut::<HashMap<EntityPath, usize>>()
                            .unwrap();
                        let mut i = 0;
                        loop {
                            let path = hashmap.iter().nth(i).and_then(|p| Some(p.0.clone()));
                            if path.is_none() {
                                break;
                            }
                            let mut path = path.unwrap();
                            if path.parts[0..=1] == name {
                                if let Some(v) = hashmap.remove(&path) {
                                    path.parts.drain(0..=1);
                                    handle_ids
                                        .push((id, (*path.parts.get(0).as_ref().unwrap()).clone()));
                                    hashmap.insert(path, v);
                                }
                            }
                            i += 1;
                        }
                    }
                }
            }
            for (id, e) in handle_ids.iter() {
                self.animations.insert(e.clone(), a.get_handle(*id));
            }
            unsafe {
                let unsaf = world.as_unsafe_world_cell();

                unsaf.world_mut().insert_or_spawn_batch(
                    unsaf
                        .world()
                        .get::<Children>(me)
                        .unwrap()
                        .iter()
                        .map(|p| (p.clone(), AnimationPlayer::default()))
                        .collect::<Vec<_>>(),
                );
            }
            world
                .entity_mut(world.get::<Parent>(self.me).unwrap().get())
                .remove::<AnimationPlayer>();
            println!("Animations: {:?}", self.animations);
        }
    }
    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        if self.was_played {
            return false;
        }

        let Some(q) = world.iter_entities().find(|p| p.contains::<PlayerCameraContainerMarker>()) else {
            return false;
        };

        if (q.get::<GlobalTransform>().unwrap().translation()
            - world.get::<GlobalTransform>(self.me).unwrap().translation())
        .length_squared()
            <= 5.
        {
            return true;
        }

        false
    }
    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        return true;
        unsafe {
            let unsaf = world.as_unsafe_world_cell();
            for child in unsaf.world().get::<Children>(self.me).unwrap().iter() {
                let mut player = unsaf
                    .world_mut()
                    .get_mut::<AnimationPlayer>(*child)
                    .unwrap();

                match self
                    .animations
                    .get(unsaf.world().get::<Name>(*child).unwrap())
                {
                    Some(animation) => {
                        player.play(animation.clone());
                    }
                    None => {
                        println!("{:?}", unsaf.world().get::<Name>(*child).unwrap());
                    }
                }
            }
        }
        // for animation in self.animations.iter() {
        //     player.play(animation.clone());
        // }
        self.was_played = true;
        true
    }
}
