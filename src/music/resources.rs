use bevy::prelude::{Component, Resource};


#[derive(Resource, Component, Default, Clone)]
pub struct MusicChannel;

pub struct Instrument<F>(pub F);
