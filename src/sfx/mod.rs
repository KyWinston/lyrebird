use bevy::prelude::*;
use components::SfxEmitter;
use events::PlayTone;
use systems::play_sfx;

pub mod components;
pub mod events;
pub mod resources;
mod systems;
pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayTone>()
            .register_type::<SfxEmitter>()
            .add_systems(Startup, play_sfx);
    }
}
