use std::{rc::Rc, sync::Arc, time::Duration};

use bevy::{
    prelude::{
        Children, Entity, Event, GlobalTransform, Input, KeyCode, Name, Parent, Transform, World,
    },
    text::TextStyle,
    time::Time,
    utils::Instant,
};
use bevy_rapier3d::prelude::{CollisionGroups, Group, QueryFilterFlags, RapierContext};
use serde_json::Value;

use crate::lib::tools::{
    collision_groups,
    events::{self, ButtonState, ModifyCollisionGroup, ProposePopup},
    markers::{PlayerCameraContainerMarker, PlayerParentMarker},
};

use super::Action;

pub struct StandButtonAction {
    pub startup: bool,
    pub name: String,
    pub when_pressed: Option<Instant>,
    pub is_pressed: bool,
    pub cooldown: Duration,
    pub can_be_pressed: bool,
    pub hint: Arc<String>,
    pub stand_entity: Entity,
    pub button_entity: Entity,

    pub press_longetivity: Duration,
    pub retarget_index: u64,
}

impl Default for StandButtonAction {
    fn default() -> Self {
        StandButtonAction {
            startup: false,
            name: "stand_button".into(),
            hint: Arc::new("Press button".into()),
            can_be_pressed: true,
            button_entity: Entity::PLACEHOLDER,
            when_pressed: None,
            press_longetivity: Duration::from_secs_f32(2.),
            retarget_index: 0,
            cooldown: Duration::from_secs_f32(5.),
            stand_entity: Entity::PLACEHOLDER,
            is_pressed: false,
        }
    }
}

impl Action for StandButtonAction {
    fn new(value: Value, main: &serde_json::map::Map<String, Value>) -> Self {
        let mut a = StandButtonAction::default();
        a.retarget_index = value.as_u64().unwrap();
        a.cooldown =
            Duration::from_secs_f32(main.get("#cooldown").unwrap().as_f64().unwrap() as f32);
        a.press_longetivity = Duration::from_secs_f32(
            main.get("#press_longetivity").unwrap().as_f64().unwrap() as f32,
        );
        a
    }

    fn try_startup(&mut self, me: Entity, world: &mut World) {
        if !self.startup {
            self.stand_entity = me;
            world.get::<Children>(me).unwrap().iter().find(|child| {
                match world.get::<Name>(**child) {
                    Some(p) if p.starts_with("TheButton") => {
                        self.button_entity = *child.clone();
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            });
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
        // return true;
        // if self.when_pressed.is_some() {
        //     println!(
        //         "{:?}",
        //         world
        //             .get_resource::<Time>()
        //             .unwrap()
        //             .last_update()
        //             .unwrap()
        //             .duration_since(self.when_pressed.unwrap())
        //     );
        // }
        if !self.press_longetivity.is_zero()
            && self.is_pressed
            && world
                .get_resource::<Time>()
                .unwrap()
                .last_update()
                .unwrap()
                .duration_since(self.when_pressed.unwrap())
                > self.press_longetivity
        {
            // println!("eveveveveve");
            world.send_event(ButtonState {
                id: self.retarget_index,
                is_pressed: false,
                just_changed: true,
            });
            // self.when_pressed = None;
            self.is_pressed = false;
        }
        if !self.can_be_pressed {
            return false;
        }

        let Some(q) = world.iter_entities().find(|p| p.contains::<PlayerCameraContainerMarker>()) else {
            return false;
        };

        let transform = q.get::<GlobalTransform>().unwrap();

        let Some(ctx) = world.get_resource::<RapierContext>() else {
            return false;
        };

        if let Some((entity, toi)) = ctx.cast_ray(
            transform.translation(),
            transform.forward(),
            2.,
            true,
            bevy_rapier3d::prelude::QueryFilter {
                predicate: Some(&|entity| entity == self.stand_entity),
                ..Default::default()
            },
        ) {
            return true;
        }

        false
    }

    fn execute(&mut self, world: &mut World) -> bool {
        if self.when_pressed.is_some()
            && (world
                .get_resource::<Time>()
                .unwrap()
                .last_update()
                .unwrap()
                .duration_since(self.when_pressed.unwrap())
                < self.cooldown
                || self.cooldown.is_zero())
        {
            return false;
        }

        world.send_event(ProposePopup {
            text: self.hint.clone(),
            priority: 1,
            style: TextStyle {
                font_size: 26.0,
                ..Default::default()
            },
            key: Some(KeyCode::E),
        });

        // let Some(me) = self.button_entity else {
        //     return false;
        // };

        let Some(ok) = world.get_resource::<Input<KeyCode>>() else {
            return false;
        };

        if ok.just_pressed(KeyCode::E) == false {
            return false;
        }

        world.send_event(ButtonState {
            id: self.retarget_index,
            is_pressed: true,
            just_changed: true,
        });

        self.when_pressed = world.get_resource::<Time>().unwrap().last_update();
        self.is_pressed = true;

        true //:D
    }
}
