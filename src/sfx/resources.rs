use bevy::prelude::*;

#[derive(Resource)]
pub struct SoundFont {
    pub name:String,
    pub target_chunk: [f64;2],
    pub volume: f32,
}