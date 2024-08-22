use bevy::prelude::*;
use bevy_fundsp::prelude::{AudioUnit, Backend, DefaultBackend, DspGraph, DspManager};
use bevy_kira_audio::{AudioChannel, AudioControl, AudioSource};
use uuid::Uuid;

use crate::{
    dialogue_view::typewriter::resources::Typewriter, music::instruments::resources::HiHatId,
};

use super::resources::SfChannel;

pub struct SoundFont<F>(pub F);

impl<T: AudioUnit + 'static, F: Send + Sync + 'static + Fn() -> T> DspGraph for SoundFont<F> {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit> {
        Box::new((self.0)())
    }
}

pub fn play_sf(
    mut assets: ResMut<Assets<AudioSource>>,
    typewrite: Res<Typewriter>,
    dsp_manager: Res<DspManager>,
    id: Res<HiHatId>,
    audio: Res<AudioChannel<SfChannel>>,
) {
    if typewrite.is_changed() && !typewrite.is_added() {
        let source = dsp_manager
            .get_graph_by_id(&id.0)
            .unwrap_or_else(|| panic!("DSP source not found!"));
        let audio_source = DefaultBackend::convert_to_audio_source(source.clone());
        let audio_source = assets.add(audio_source);
        audio.play(audio_source).with_volume(0.3);
    }
}
