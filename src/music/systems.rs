use bevy::prelude::*;
use bevy_fundsp::prelude::{Backend, DefaultBackend, DspManager};
use bevy_kira_audio::{prelude::*, AudioSource};

use super::{instruments::resources::HiHatId, resources::MusicChannel};

#[allow(dead_code)]
pub fn play_instrument(
    dsp_manager: Res<DspManager>,
    mut assets: ResMut<Assets<AudioSource>>,
    id: Res<HiHatId>,
    audio: Res<AudioChannel<MusicChannel>>,
) {
    let source = dsp_manager
        .get_graph_by_id(&id.0)
        .unwrap_or_else(|| panic!("DSP source not found!"));
    let audio_source = DefaultBackend::convert_to_audio_source(source.clone());
    let audio_source = assets.add(audio_source);
    audio.play(audio_source).with_volume(1.0);
}
