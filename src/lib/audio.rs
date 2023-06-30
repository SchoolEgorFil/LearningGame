use bevy_kira_audio::prelude::*;

use bevy::prelude::{self, AssetServer, Audio, Res, Vec3};

struct AudioEvent {
    position: Vec3,
    audio: Audio,
}
