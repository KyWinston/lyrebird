use bevy::prelude::*;


#[derive(Event)]
pub struct PlayTone(pub [u8;3], pub String);