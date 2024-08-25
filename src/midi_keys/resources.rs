use bevy::prelude::*;
use bevy_fundsp::prelude::*;


#[derive(Resource)]
pub struct PitchVar(pub Shared);

impl PitchVar {
    pub fn set_pitch(&self, pitch: f32) {
        self.0.set_value(pitch);
    }
}
