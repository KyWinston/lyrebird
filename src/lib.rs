use bevy::prelude::*;
// use music::MusicPlugin;
use sfx::SfxPlugin;
// use synth::{resources::Vol, SynthPlugin};
use systems::setup_cam_audio;

pub struct LyrebirdPlugin;

pub mod components;
// pub mod midi_keys;
// pub mod music;
pub mod resources;
pub mod sfx;
pub mod synth;
pub mod systems;

impl Plugin for LyrebirdPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(Vol(1.0))
            app.add_plugins((
                // MusicPlugin,
                SfxPlugin,
                // SynthPlugin,
            ))
            .add_observer(setup_cam_audio);
    }
}
