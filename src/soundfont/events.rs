use bevy::prelude::*;
use bevy_fundsp::prelude::{Backend, DefaultBackend, DspManager};
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource};

use crate::midi_keys::resources::PitchVar;

use super::resources::{SfChannel, SoundFonts};

#[derive(Event)]
pub struct SFplayEvent(pub f32, pub String);

pub fn play_sf(
    mut assets: ResMut<Assets<AudioSource>>,
    mut sf_play: EventReader<SFplayEvent>,
    dsp_manager: Res<DspManager>,
    audio: Res<AudioChannel<SfChannel>>,
    pitch: Res<PitchVar>,
    ids: Res<SoundFonts>,
) {
    for ev in sf_play.read() {
        pitch.set_pitch(ev.0);
        let source = dsp_manager
            .get_graph_by_id(&ids.0["tutorial"])
            .unwrap_or_else(|| panic!("DSP source not found!"));
        let audio_source = DefaultBackend::convert_to_audio_source(source.clone());
        let audio_source = assets.add(audio_source);
        audio.play(audio_source).with_volume(0.3);
    }
}
