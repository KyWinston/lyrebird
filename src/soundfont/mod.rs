use bevy::{prelude::*, utils::hashbrown::HashMap};

use bevy_fundsp::{prelude::SourceType, DspAppExt};
use bevy_kira_audio::AudioApp;
use resources::{tutorial_sf, SfChannel, SoundFonts};
use systems::play_sf;

use crate::dialogue_view::typewriter::resources::Typewriter;

pub mod components;
pub mod resources;
pub mod systems;
pub struct SoundFontPlugin;

impl Plugin for SoundFontPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<SoundFonts>(SoundFonts(HashMap::new()))
            .add_dsp_source(tutorial_sf, SourceType::Static { duration: 0.2 })
            .add_systems(Update, play_sf.run_if(resource_exists::<Typewriter>))
            .add_audio_channel::<SfChannel>();
    }
}
