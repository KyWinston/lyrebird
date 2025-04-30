use bevy::prelude::*;
use components::SfxEmitter;
use systems::{play_sfx, process_spatial_damping};

pub mod components;
pub mod resources;
mod systems;
pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SfxEmitter>()
            .add_systems(Update, process_spatial_damping);
    }
}
