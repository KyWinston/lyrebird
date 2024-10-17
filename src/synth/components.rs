use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct InstrumentSource {
    pub name: String,
    pub abbr: String,
}
