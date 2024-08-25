use bevy::{prelude::*, utils::HashMap};

use bevy_fundsp::{
    prelude::{
        hacker::{shared, var},
        DspGraph, SourceType,
    },
    DspAppExt,
};
use bevy_kira_audio::AudioApp;
use events::{play_sf, SFplayEvent};
use resources::{tutorial_sf, SfChannel, SoundFonts};
use systems::SoundFont;

use crate::midi_keys::resources::PitchVar;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;
pub struct SoundFontPlugin;

impl Plugin for SoundFontPlugin {
    fn build(&self, app: &mut App) {
        let pitch = shared(440.0);
        let pitch2 = pitch.clone();
        let tut_sf = move || var(&pitch2) >> tutorial_sf();
        let tut_sf_dsp = SoundFont(tut_sf.clone());
        let tut_sf_id = tut_sf_dsp.id();
        let mut soundfonts = HashMap::new();
        soundfonts.insert("tutorial".to_string(), tut_sf_id);
        app.add_event::<SFplayEvent>()
            .add_dsp_source(tut_sf_dsp, SourceType::Static { duration: 0.2 })
            .insert_resource(PitchVar(pitch))
            .insert_resource(SoundFonts(soundfonts))
            .add_systems(Update, play_sf)
            .add_audio_channel::<SfChannel>();
    }
}
