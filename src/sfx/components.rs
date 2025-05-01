use bevy::prelude::*;
use bevy_kira_audio::{AudioInstance, AudioSource};

use super::systems::play_sfx;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add = play_sfx)]
pub struct SfxEmitter {
    pub sound: Handle<AudioSource>,
    pub instance: Option<Handle<AudioInstance>>,
    pub intensity: f64,
    pub looped: bool,
    pub speed: f32,
    pub near: f32,
    pub far: f32,
    start: f64,
    end: f64,
}

impl SfxEmitter {
    pub fn new(sound: Handle<AudioSource>, intensity: f64, looped: bool) -> Self {
        Self {
            sound,
            instance: None,
            intensity,
            looped,
            speed: 1.0,
            near: 0.0,
            far: 100.0,
            start: 0.0,
            end: 1.0,
        }
    }
    pub fn with_duration(&mut self, start: f64, end: f64) {
        self.start = start;
        self.end = end;
    }

    pub fn get_duration(&self) -> (f64, f64) {
        (self.start, self.end)
    }
}

#[derive(Component)]
pub struct ListeningCamera;
