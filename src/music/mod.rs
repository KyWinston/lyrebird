use bevy::prelude::*;

pub mod events;
// pub mod instruments;
pub mod resources;
pub mod systems;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, _app: &mut App) {}
}
