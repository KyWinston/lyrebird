use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct SoundFonts(pub HashMap<String, SoundFont>);

#[derive(Debug, PartialEq, Clone)]
pub struct SoundFont {
    pub name: String,
    pub path: String,
    pub target_chunk: [f64; 2],
    pub volume: f64,
}

#[derive(Resource)]
pub struct SfChannel;
