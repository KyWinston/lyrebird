use bevy::{prelude::*, utils::HashMap};
use fundsp::prelude::*;
use uuid::Uuid;

#[derive(Resource)]
pub struct SoundFonts(pub HashMap<String, Uuid>);

#[derive(Resource)]
pub struct SfChannel;

pub fn tutorial_sf() -> An<Unit<U1, U2>> {
    unit::<U1, U2>(Box::new(square() >> (split::<U2>() * 0.4)))
}
