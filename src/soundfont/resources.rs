use bevy::{prelude::*, utils::HashMap};
use bevy_fundsp::prelude::*;
use uuid::Uuid;

#[derive(Resource)]
pub struct SoundFonts(pub HashMap<String, (Uuid, Shared)>);

#[derive(Resource)]
pub struct SfChannel;

pub fn tutorial_sf() -> impl AudioUnit {
    square_hz(500.0) >> (split::<U2>() * 0.2)
}
