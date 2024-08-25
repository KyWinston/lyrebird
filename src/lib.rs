use bevy::prelude::*;
use bevy_fundsp::DspPlugin;
use bevy_kira_audio::AudioPlugin;
use dialogue::DialoguePlugin;
use dialogue_view::DialogueViewPlugin;
use midi_keys::MidiKeysPlugin;
use music::MusicPlugin;
use sfx::SfxPlugin;
use soundfont::SoundFontPlugin;
use systems::setup_cam_audio;

pub struct ParakeetPlugin;

pub mod dialogue;
pub mod dialogue_view;
pub mod midi_keys;
pub mod music;
pub mod resources;
pub mod sfx;
pub mod soundfont;
pub mod systems;

impl Plugin for ParakeetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AudioPlugin,
            DspPlugin::default(),
            MusicPlugin,
            MidiKeysPlugin,
            SfxPlugin,
            DialoguePlugin,
            DialogueViewPlugin,
            SoundFontPlugin,
        ))
        .observe(setup_cam_audio);
    }
}
