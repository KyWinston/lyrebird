use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

#[cfg(feature = "debug")]
use midi_keys::MidiKeysPlugin;

use music::MusicPlugin;
use sequencer::SequencerPlugin;
use sfx::SfxPlugin;
use soundfont::SoundFontPlugin;
use systems::setup_cam_audio;

pub struct LyrebirdPlugin;

pub mod components;
#[cfg(feature = "debug")]
pub mod midi_keys;
pub mod music;
pub mod resources;
pub mod sequencer;
pub mod sfx;
pub mod soundfont;
pub mod systems;

impl Plugin for LyrebirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AudioPlugin,
            #[cfg(feature = "debug")]
            MidiKeysPlugin,
            MusicPlugin,
            SfxPlugin,
            SoundFontPlugin,
            SequencerPlugin,
        ))
        .observe(setup_cam_audio);
    }
}
