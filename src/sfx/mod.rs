use bevy::prelude::*;
use components::SfxEmitter;
use systems::play_sfx;
// use systems::play_sfx;

pub mod components;
pub mod systems;

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<SfxEmitter>()
        // .observe(play_sfx);
        .add_systems(Startup, play_sfx);
    }
}
