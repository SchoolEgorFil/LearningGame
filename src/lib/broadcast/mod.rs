use bevy::{
    prelude::{Component, Entity, Name, Plugin, PostUpdate, PreUpdate, World},
    utils::HashMap,
};
use serde_json::Value;

pub mod ball_falling_01;
pub mod collision_audio;
pub mod collision_button;
pub mod teleport;
pub mod link_opener;
// pub mod explosion_test_01;
pub mod open_door;
pub mod stand_button;
pub mod one_animation;
pub mod full_animation;
pub mod named_animation;
pub mod delay;

pub struct ManagerPlugin {}

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, run_all);
    }
}

fn run_all(world: &mut World) {
    let world = world.as_unsafe_world_cell();
    unsafe {
        for (entity, mut actor, name) in world
            .world_mut()
            .query::<(Entity, &mut Actor, Option<&Name>)>()
            .iter_mut(world.world_mut())
        {
            for action in actor.0.iter_mut() {
                action.1.try_startup(entity, world.world_mut());

                let pred = action.1.predicate(world.world_mut());
                if pred {
                    let exec = action.1.execute(world.world_mut());
                    // println!(
                    //     "in entity {} action named {} executed {}",
                    //     name.and_then(|p| Some(p.to_string()))
                    //         .unwrap_or_else(|| format!(
                    //             "{}v{}",
                    //             entity.index(),
                    //             entity.generation()
                    //         )),
                    //     action.1.name(),
                    //     exec
                    // );
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Actor(pub HashMap<String, Box<dyn Action>>);

pub trait Action: Send + Sync {
    fn new(value: Value, main: &serde_json::map::Map<String, Value>) -> Self
    where
        Self: Sized;
    /// It is advised to use 'startup' bool in your struct to track whether you actually done this step
    fn change_name(&mut self, name: String);
    fn name(&self) -> String;
    fn try_startup(&mut self, me: Entity, world: &mut World);
    fn predicate(&mut self, world: &mut World) -> bool;
    fn execute(&mut self, world: &mut World) -> bool;
}

// impl action

// pub struct ScriptAction {
//     pub script_string: String, // todo!() replace with AST
// }

// impl Action for ScriptAction {
//     fn predicate(&self, world: &World) -> bool {

//     }
// }
