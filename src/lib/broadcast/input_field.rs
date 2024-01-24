use std::{time::Duration, sync::Arc};

use bevy::{
    prelude::{warn, Entity, Events, Name, Transform, Vec3, GlobalTransform, Input, KeyCode, NextState, TextBundle, AssetServer, Visibility},
    time::Time,
    utils::Instant, window::ReceivedCharacter, text::{TextStyle, TextSection}, ui::{BackgroundColor, Style},
};
use bevy_rapier3d::prelude::RapierContext;
use itertools::izip;

use crate::{lib::{tools::{events::{ButtonState, ProposePopup, CustomEvent}, markers::PlayerCameraContainerMarker, consts::font_names}}, PlayerState};

use super::Action;

pub struct InputFieldAction {
    pub is_started: bool,
    pub name: String,
    pub enterred_string: String,
    pub typing_mode: bool,
    pub hint: Arc<String>,
    pub stand_entity: Entity,
    pub text_input_field: Entity,
}

impl Action for InputFieldAction {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn change_name(&mut self, name: String) {
        self.name = name;
    }
    fn try_startup(&mut self, me: bevy::prelude::Entity, world: &mut bevy::prelude::World) {
        if !self.is_started {
            self.stand_entity = me;
            
            let font = world.resource::<AssetServer>();
            let handle = font.load(font_names::NOTO_SANS_BOLD);
            self.text_input_field = world.spawn((TextBundle {
                background_color: BackgroundColor(bevy::prelude::Color::WHITE),
                text: bevy::text::Text {
                    sections: vec![
                        TextSection {
                            value: " ".into(),
                            style: TextStyle { font: handle, font_size: 48., color: bevy::prelude::Color::BLACK }
                        }
                    ],
                    alignment: bevy::text::TextAlignment::Center,
                    ..Default::default()
                },
                style: Style {
                    position_type: bevy::ui::PositionType::Absolute,
                    bottom: bevy::ui::Val::Px(20.),
                    left: bevy::ui::Val::Px(60.),
                    right: bevy::ui::Val::Px(60.),
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                ..Default::default()
            })).id();

            self.is_started = true;

        }
    }
    fn new(value: serde_json::Value, main: &serde_json::map::Map<String, serde_json::Value>) -> Self
    where
        Self: Sized,
    {
        InputFieldAction {
            hint: Arc::new("Press button".into()),
            is_started: false,
            name: value.as_str().unwrap().to_owned(),
            enterred_string: "".into(),
            stand_entity: Entity::PLACEHOLDER,
            text_input_field: Entity::PLACEHOLDER,
            typing_mode: false
        }
    }

    fn predicate(&mut self, world: &mut bevy::prelude::World) -> bool {
        if !self.typing_mode {
            //
            // if !self.can_be_pressed {
            //     return false;
            // }
            
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
                world.send_event(ProposePopup {
                    text: self.hint.clone(),
                    priority: 1,
                    style: TextStyle {
                        font: world.resource::<bevy::prelude::AssetServer>().load(font_names::NOTO_SANS_MEDIUM),
                        font_size: 32.0,
                        color: bevy::prelude::Color::WHITE,
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
                return true;
            }
        } else {
            let mut a = false;
            unsafe {
                let cell = world.as_unsafe_world_cell();
                let character_event = cell.world().get_resource::<Events<ReceivedCharacter>>().unwrap();
                let time_res = cell.world().get_resource::<Time>().unwrap();                

                let mut reader = character_event.get_reader();
                for char in reader.read(character_event) {
                    if(char.char.is_alphanumeric() || char.char == ' ' || char.char == ',' || char.char == '.' || char.char == '*' || char.char == '^') {
                        self.enterred_string.push(char.char);
                    }
                }
                let mut keyboard_event = cell.world_mut().get_resource_mut::<Input<KeyCode>>().unwrap();
                if keyboard_event.just_pressed(KeyCode::Back) {
                    self.enterred_string.pop();
                    println!("Back is pressed {}", self.enterred_string);
                }
                if keyboard_event.just_pressed(KeyCode::Return) {
                    println!("I think it works: {}", &*self.enterred_string);
                    let mut state = cell.world_mut().resource_mut::<NextState<PlayerState>>();
                    state.0 = Some(PlayerState::Interactive);
                    *cell.world_mut().entity_mut(self.text_input_field).get_mut::<Visibility>().unwrap() = Visibility::Hidden;
                    a = true;
                }
                if keyboard_event.just_pressed(KeyCode::Escape) {
                    let mut state = cell.world_mut().resource_mut::<NextState<PlayerState>>();
                    state.0 = Some(PlayerState::Interactive);
                    self.typing_mode = false;
                    *cell.world_mut().entity_mut(self.text_input_field).get_mut::<Visibility>().unwrap() = Visibility::Hidden;
                }
                keyboard_event.clear();
                let mut text = cell.world_mut().entity_mut(self.text_input_field);
                let mut text_text = text.get_mut::<bevy::text::Text>().unwrap();
                text_text.sections[0].value = self.enterred_string.clone();
                text_text.sections[0].value.push(
                    if time_res.elapsed().as_secs() % 2 == 0 {
                        '|'
                    } else { 
                        ' '
                    }
                );
                
                return a;
            }
        }
        false
    }

    fn execute(&mut self, world: &mut bevy::prelude::World) -> bool {
        if !self.typing_mode {
            let mut state = world.resource_mut::<NextState<PlayerState>>();
            state.0 = Some(PlayerState::Restricted);
            println!("Entered typing mode");
            *world.entity_mut(self.text_input_field).get_mut::<Visibility>().unwrap() = Visibility::Visible;
            self.typing_mode = true;
        } else {
            world.send_event(CustomEvent {
                name: self.name.clone(),
                json_encoded: self.enterred_string.clone()
            });
            self.typing_mode = false;
            self.enterred_string.clear();
        }
        true
    }
}
