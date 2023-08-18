use std::time::{Duration, Instant};

use bevy::{
    prelude::{in_state, Component, Entity, Handle, IntoSystemConfigs, Plugin, Query, Res, Update},
    time::Time,
};
use bevy_kira_audio::{Audio, AudioControl, AudioSource};
use bevy_rapier3d::prelude::RapierContext;

use crate::AppState;

use super::tools::markers::PlayerParentMarker;

pub struct AudioPlayerPlugin;

impl Plugin for AudioPlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            handle_audio_collision.run_if(in_state(AppState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct CollisionAudio {
    pub audio: Handle<AudioSource>,
    pub volume: f32,
    /// `None` means no replaying
    pub recursive_cooldown: Option<Duration>,
    pub last_played: Option<Instant>,
    pub was_colliding: bool,
}

impl CollisionAudio {
    pub fn from_handle(handle: Handle<AudioSource>) -> CollisionAudio {
        CollisionAudio {
            audio: handle,
            volume: 1.0,
            last_played: None,
            recursive_cooldown: None,
            was_colliding: false,
        }
    }

    pub fn with_volume(mut self: Self, volume: f32) -> Self {
        self.volume = volume;
        self
    }
}

fn handle_audio_collision(
    mut sensor_q: Query<(Entity, &mut CollisionAudio)>,
    player_q: Query<(Entity, &PlayerParentMarker)>,
    rapier_context: Res<RapierContext>,
    audio: Res<Audio>,
    time: Res<Time>,
) {
    let Ok(player) = player_q.get_single() else {
        return;
    };

    for mut sensor in sensor_q.iter_mut() {
        if rapier_context.intersection_pair(player.0, sensor.0) == Some(true) {
            if !sensor.1.was_colliding
                && (sensor.1.last_played.is_none()
                    || (sensor.1.recursive_cooldown.is_some_and(|cooldown| {
                        time.last_update()
                            .unwrap()
                            .duration_since(sensor.1.last_played.unwrap())
                            > cooldown
                    })))
            {
                audio.play(sensor.1.audio.clone());
            }
            sensor.1.last_played = Some(time.last_update().unwrap());
        }
    }
}
