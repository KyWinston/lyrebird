use bevy::prelude::*;
use components::SfxEmitter;
use systems::{process_sound_properties, process_spatial_damping};

pub mod components;
pub mod resources;
mod systems;
pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SfxEmitter>()
            .add_systems(Update, (process_sound_properties, process_spatial_damping));
    }
}
