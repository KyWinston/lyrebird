use bevy::prelude::*;
use systems::play_sfx;

pub mod components;
pub mod systems;

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.observe(play_sfx);
    }
}
