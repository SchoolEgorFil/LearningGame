use std::sync::Arc;

use bevy::{
    prelude::{
        BuildChildren, Color, Commands, Component, EventReader, NodeBundle, Plugin, Query,
        TextBundle, Update, Visibility, With,
    },
    text::Text,
    transform::commands,
};

use super::tools::events::ProposePopup;

pub struct HintOverlayPlugin;

impl Plugin for HintOverlayPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, show_hint);
    }
}

#[derive(Component)]
struct HintTextUI;

fn show_hint(
    mut commands: Commands,
    mut ev_r: EventReader<ProposePopup>,
    mut query: Query<(&mut Text, &mut Visibility), With<HintTextUI>>,
) {
    let mut max: Option<(u32, usize)> = None;
    if ev_r.is_empty() {
        if !query.is_empty() {
            *query.single_mut().1 = Visibility::Hidden;
        }
        return;
    }
    let ev = {
        let evs = ev_r.iter().collect::<Vec<_>>();
        for i in 0..evs.len() {
            if max.is_none() || evs[i].priority > max.unwrap().0 {
                max = Some((evs[i].priority, i));
            }
        }
        evs[max.unwrap().1]
    };
    if query.is_empty() {
        commands
            .spawn(NodeBundle {
                style: bevy::ui::Style {
                    align_items: bevy::ui::AlignItems::Center,
                    justify_content: bevy::ui::JustifyContent::Center,
                    width: bevy::ui::Val::Percent(100.),
                    height: bevy::ui::Val::Percent(100.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((
                    HintTextUI,
                    TextBundle::from_section(ev.text.as_str(), ev.style.clone()),
                ));
            });
    } else {
        *query.single_mut().1 = Visibility::Visible;
        if query.single_mut().0.sections[0].value != *ev.text {
            query.single_mut().0.sections[0].value = (*ev.text).clone();
            query.single_mut().0.sections[0].style = ev.style.clone();
            println!("{:?}", ev.style);
        }
    }
}
