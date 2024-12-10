use bevy::prelude::*;

#[derive(Resource, Debug, PartialEq)]
pub struct SoundFont {
    pub name: String,
    pub target_chunk: [f64; 2],
    pub volume: f64,
}
