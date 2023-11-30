use std::time::Duration;

use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct TransitionMarker {
    pub started: bool,
    pub timer: Timer,
}

impl TransitionMarker {
    pub fn new(start: bool, time: Duration) -> TransitionMarker {
        TransitionMarker {
            started: start,
            timer: Timer::new(time, bevy::time::TimerMode::Once),
        }
    }

    pub fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta);
        self.timer.just_finished()
    }

    pub fn reset(&mut self) {
        self.timer.reset();
    }

    pub fn linear(&self) -> Option<f32> {
        if !self.started {
            return None;
        }
        Some(self.timer.percent())
    }

    pub fn ease_in(&self) -> Option<f32> {
        if !self.started {
            return None;
        }
        Some(self.timer.percent() * self.timer.percent())
    }

    pub fn ease_out(&self) -> Option<f32> {
        if !self.started {
            return None;
        }
        let a = self.timer.percent_left();
        Some(1. - a * a)
    }

    pub fn ease_in_out(&self) -> Option<f32> {
        if !self.started {
            return None;
        }
        let a = self.timer.percent();
        Some((1. - a) * self.ease_in().unwrap() + a * self.ease_out().unwrap())
    }
}
