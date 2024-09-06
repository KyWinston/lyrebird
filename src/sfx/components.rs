use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SfxEmitter {
    pub sound: String,
    pub intensity: f64,
    pub looped: bool,
    start: f64,
    end: f64,
}

impl SfxEmitter {
    pub fn new(sound: String, intensity: f64, looped: bool) -> Self {
        Self {
            sound,
            intensity,
            looped,
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
